use iced::{
    Element, Length, Padding, Renderer, Task, keyboard,
    widget::{column, container},
};
use rustc_hash::FxHashMap;
use tracing::{debug, instrument};

use crate::{
    app::PlaybackOwner,
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome, TagOutcome},
    playback::controller::PlaybackControllerStatus,
    tag::{
        index::TrackTagIndex,
        models::{Tag, TagGroup, TagId},
    },
    track::models::{Track, TrackId},
    ui::{
        modals::tag_tracks::{
            tag::{get_tag_group_tags, get_tag_index},
            widgets::{footer, header, playback, tag_group_list, tag_list},
        },
        theme::{Theme, catalog},
        widgets::separator::vertical_separator,
    },
};

pub mod handler;
pub mod tag;
pub mod widgets;

pub struct TagTracksModal {
    track_tagging_queue: Vec<TrackId>,
    track_tagging_queue_cursor: usize,
    tag_groups_cursor: usize,

    scrubbing_keyboard_action: ScrubbingKeyboardAction,

    current_playback_position: f64,
    playback_status: PlaybackStatus,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToPreviousTrack,
    GoToNextTrack,
    SelectTabGroup(usize),
    ToggleTag(TagId),

    Keyboard(keyboard::Event),

    Play(TrackId),
    Resume,
    Pause,
    Close,
    PlaybackScrubbed(f64),
    PlaybackSeeked,
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
    Tag(TagOutcome),
}

#[derive(Debug)]
pub enum PlaybackStatus {
    Playing,
    Paused,
}

#[derive(Debug, Default)]
pub struct ScrubbingKeyboardAction {
    pub left: bool,
    pub right: bool,
}

#[derive(Debug)]
pub enum KeyboardScrubDirection {
    Right,
    Left,
}

// TODO: Add UI for empty states
impl TagTracksModal {
    pub fn new(track_tagging_queue: Vec<TrackId>) -> Self {
        Self {
            track_tagging_queue,
            track_tagging_queue_cursor: 0,
            tag_groups_cursor: 0,
            scrubbing_keyboard_action: ScrubbingKeyboardAction::default(),

            current_playback_position: 0.0,
            playback_status: PlaybackStatus::Playing,
        }
    }

    #[instrument(
        skip(self, tracks, tags, tag_groups)
        fields(tracks_len = tracks.len(), tags_len = tags.len(), tag_groups_len = tag_groups.len()),
        level = "debug"
    )]
    pub fn update(
        &mut self,
        message: Message,
        tracks: &FxHashMap<TrackId, Track>,
        tags: &[Tag],
        tag_groups: &[TagGroup],
        playback_controller_status: &PlaybackControllerStatus,
    ) -> (Task<Message>, Vec<Outcome>) {
        let mut task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Close => outcomes.push(Outcome::Modal(ModalOutcome::CloseModal)),
            Message::SelectTabGroup(tab_group_index) => {
                self.tag_groups_cursor = tab_group_index;
            }
            Message::ToggleTag(tag_id)
                if let Some(track_id) = self
                    .track_tagging_queue
                    .get(self.track_tagging_queue_cursor) =>
            {
                outcomes.push(Outcome::Tag(TagOutcome::ToggleTag(*track_id, tag_id)));
            }
            Message::GoToNextTrack => {
                outcomes.extend(self.go_to_next_track());
            }
            Message::GoToPreviousTrack => {
                outcomes.extend(self.go_to_previous_track());
            }

            Message::Keyboard(event) => {
                (task, outcomes) = self.handle_keyboard(
                    event,
                    tracks,
                    tags,
                    tag_groups,
                    playback_controller_status,
                );
            }

            Message::ToggleTag(_) => {}

            Message::Resume => {
                self.playback_status = PlaybackStatus::Playing;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Resume));
            }
            Message::Pause => {
                self.playback_status = PlaybackStatus::Paused;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
            }
            Message::Play(track_id) => {
                self.playback_status = PlaybackStatus::Playing;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Play(track_id)));
            }
            Message::PlaybackScrubbed(position) => {
                self.current_playback_position = position;

                if matches!(
                    playback_controller_status,
                    PlaybackControllerStatus::Playing,
                ) {
                    outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
                }
            }
            Message::PlaybackSeeked => {
                let pre_seek_status = match self.playback_status {
                    PlaybackStatus::Playing => PlaybackControllerStatus::Playing,
                    PlaybackStatus::Paused => PlaybackControllerStatus::Stopped,
                };

                outcomes.push(Outcome::Playback(PlaybackOutcome::Seek {
                    timestamp: self.current_playback_position as u64,
                    post_seek_status: Some(pre_seek_status),
                }));
            }
        }

        (task, outcomes)
    }

    #[allow(clippy::too_many_lines)] // TODO: Break down later.
    #[instrument(skip_all)]
    fn handle_keyboard(
        &mut self,
        event: keyboard::Event,
        tracks: &FxHashMap<TrackId, Track>,
        tags: &[Tag],
        tag_groups: &[TagGroup],
        playback_controller_status: &PlaybackControllerStatus,
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match event {
            // alphanumeric character: tag the current track with the corresponding tag
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Character(character),
                repeat: false,
                ..
            } => {
                let Some(track_id) = self
                    .track_tagging_queue
                    .get(self.track_tagging_queue_cursor)
                else {
                    return (task, outcomes);
                };

                let Some(tag_index) = character.chars().next().and_then(get_tag_index) else {
                    return (task, outcomes);
                };

                let Some(tag_id) = tag_groups
                    .get(self.tag_groups_cursor)
                    .and_then(|tag_group| {
                        get_tag_group_tags(tags, tag_group.id)
                            .get(tag_index)
                            .map(|tag| tag.id)
                    })
                else {
                    return (task, outcomes);
                };

                outcomes.push(Outcome::Tag(TagOutcome::ToggleTag(*track_id, tag_id)));
            }
            // -> : Scrub playback to the right
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowRight),
                ..
            } => {
                outcomes.extend(self.handle_keyboard_scrub(
                    KeyboardScrubDirection::Right,
                    tracks,
                    playback_controller_status,
                ));
            }
            // <- : Scrub playback to the left
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowLeft),
                ..
            } => {
                outcomes.extend(self.handle_keyboard_scrub(
                    KeyboardScrubDirection::Left,
                    tracks,
                    playback_controller_status,
                ));
            }
            // Release -> : Commit seek
            keyboard::Event::KeyReleased {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowRight),
                ..
            } => {
                self.scrubbing_keyboard_action.right = false;

                outcomes.extend(self.handle_keyboard_seek());
            }
            // Release <- : Commit seek
            keyboard::Event::KeyReleased {
                key: keyboard::Key::Named(keyboard::key::Named::ArrowLeft),
                ..
            } => {
                self.scrubbing_keyboard_action.left = false;

                outcomes.extend(self.handle_keyboard_seek());
            }
            // Space: Pause/Unpause
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Space),
                repeat: false,
                ..
            } if !self.scrubbing_keyboard_action.left && !self.scrubbing_keyboard_action.right => {
                match self.playback_status {
                    PlaybackStatus::Paused => {
                        self.playback_status = PlaybackStatus::Playing;

                        outcomes.push(Outcome::Playback(PlaybackOutcome::Resume));
                    }
                    PlaybackStatus::Playing => {
                        self.playback_status = PlaybackStatus::Paused;

                        outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
                    }
                }
            }
            // Tab: Select next tab group
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Tab),
                repeat: false,
                modifiers: keyboard::Modifiers::NONE,
                ..
            } => {
                self.tag_groups_cursor = (self.tag_groups_cursor + 1) % tag_groups.len();
            }
            // Shift + Tab: Select previous tab group
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Tab),
                repeat: false,
                modifiers: keyboard::Modifiers::SHIFT,
                ..
            } => {
                self.tag_groups_cursor =
                    (tag_groups.len() + self.tag_groups_cursor - 1) % tag_groups.len();
            }
            // Enter: Go to the next track
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Enter),
                repeat: false,
                modifiers: keyboard::Modifiers::NONE,
                ..
            } => outcomes.extend(self.go_to_next_track()),
            // Shift + Enter: go to the previous track
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Enter),
                repeat: false,
                modifiers: keyboard::Modifiers::SHIFT,
                ..
            } => outcomes.extend(self.go_to_previous_track()),
            // Ctrl + Shift + Enter: go to the first track
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Enter),
                repeat: false,
                modifiers,
                ..
            } if modifiers == keyboard::Modifiers::SHIFT | keyboard::Modifiers::COMMAND => {
                outcomes.extend(self.go_to_first_track());
            }
            // Ctrl + Enter: go to the last track
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Enter),
                repeat: false,
                modifiers: keyboard::Modifiers::COMMAND,
                ..
            } => {
                outcomes.extend(self.go_to_last_track());
            }
            // Escape: Close the modal
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Escape),
                repeat: false,
                ..
            } => outcomes.push(Outcome::Modal(ModalOutcome::CloseModal)),
            _ => {}
        }

        (task, outcomes)
    }

    pub fn go_to_next_track(&mut self) -> Option<Outcome> {
        if self.track_tagging_queue_cursor == self.track_tagging_queue.len() - 1 {
            return None;
        }

        self.track_tagging_queue_cursor += 1;

        let track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)?;

        Some(Outcome::Playback(PlaybackOutcome::Play(*track_id)))
    }

    pub fn go_to_previous_track(&mut self) -> Option<Outcome> {
        if self.track_tagging_queue_cursor == 0 {
            return None;
        }

        self.track_tagging_queue_cursor -= 1;

        let track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)?;

        Some(Outcome::Playback(PlaybackOutcome::Play(*track_id)))
    }

    pub fn go_to_first_track(&mut self) -> Option<Outcome> {
        if self.track_tagging_queue_cursor == 0 {
            return None;
        }

        self.track_tagging_queue_cursor = 0;

        let track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)?;

        Some(Outcome::Playback(PlaybackOutcome::Play(*track_id)))
    }

    pub fn go_to_last_track(&mut self) -> Option<Outcome> {
        if self.track_tagging_queue_cursor == self.track_tagging_queue.len() - 1 {
            return None;
        }

        self.track_tagging_queue_cursor = self.track_tagging_queue.len() - 1;

        let track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)?;

        Some(Outcome::Playback(PlaybackOutcome::Play(*track_id)))
    }

    pub fn handle_keyboard_scrub(
        &mut self,
        scrub_direction: KeyboardScrubDirection,
        tracks: &FxHashMap<TrackId, Track>,
        playback_controller_status: &PlaybackControllerStatus,
    ) -> Option<Outcome> {
        let track = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)
            .and_then(|track_id| tracks.get(track_id))?;

        let frames_delta = track.sample_rate * 5; // 5 seconds of frames

        match scrub_direction {
            KeyboardScrubDirection::Left => {
                self.current_playback_position =
                    (self.current_playback_position - frames_delta as f64).max(0.0);

                self.scrubbing_keyboard_action.left = true;
            }
            KeyboardScrubDirection::Right => {
                self.current_playback_position =
                    (self.current_playback_position + frames_delta as f64).min(track.frames as f64);

                self.scrubbing_keyboard_action.right = true;
            }
        }

        if matches!(
            playback_controller_status,
            PlaybackControllerStatus::Playing,
        ) {
            return Some(Outcome::Playback(PlaybackOutcome::Pause));
        }

        None
    }

    pub fn handle_keyboard_seek(&mut self) -> Option<Outcome> {
        if self.scrubbing_keyboard_action.left || self.scrubbing_keyboard_action.right {
            return None;
        }

        let pre_seek_status = match self.playback_status {
            PlaybackStatus::Playing => PlaybackControllerStatus::Playing,
            PlaybackStatus::Paused => PlaybackControllerStatus::Stopped,
        };

        Some(Outcome::Playback(PlaybackOutcome::Seek {
            timestamp: self.current_playback_position as u64,
            post_seek_status: Some(pre_seek_status),
        }))
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(
        &mut self,
        event: &Event,
        current_playback_owner: &PlaybackOwner,
    ) -> Task<Message> {
        let task = Task::none();

        if !matches!(current_playback_owner, PlaybackOwner::TagTrackModal) {
            debug!("Playback Bar does not own the playback currently, ignoring event");

            return task;
        }

        match event {
            Event::AttemptedPlayingTrack => {
                self.playback_status = PlaybackStatus::Playing;

                self.current_playback_position = 0.0;
            }
            Event::PlaybackProgressed(position) => {
                self.current_playback_position = *position;
            }
            _ => {}
        }

        task
    }

    #[instrument(skip_all, level = "debug")]
    pub fn view<'a>(
        &self,
        theme: &Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
        track_tag_index: &'a TrackTagIndex,
    ) -> Element<'a, Message, Theme, Renderer> {
        let width = 1000.0;

        let current_tagging_track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor);
        let track = current_tagging_track_id
            .and_then(|current_tagging_track_id| tracks.get(current_tagging_track_id));
        let track_number = self.track_tagging_queue_cursor + 1;
        let track_total = self.track_tagging_queue.len();
        let track_tags = current_tagging_track_id.and_then(|current_tagging_track_id| {
            track_tag_index.get_track_tags(*current_tagging_track_id)
        });

        container(
            column![
                header(theme, track, track_number, track_total),
                vertical_separator(),
                playback(
                    theme,
                    tracks,
                    current_tagging_track_id,
                    self.current_playback_position
                ),
                vertical_separator(),
                tag_group_list(theme, tag_groups, self.tag_groups_cursor),
                vertical_separator(),
                tag_list(
                    theme,
                    tag_groups,
                    tags,
                    self.tag_groups_cursor,
                    track_tags.map(Vec::as_slice)
                ),
                vertical_separator(),
                // TODO: Add keyboard controls later on.
                // container(text("Keyboard controls"))
                //     .height(140.0)
                //     .width(Length::Fill),
                // vertical_separator(),
                footer(
                    theme,
                    &self.track_tagging_queue,
                    self.track_tagging_queue_cursor
                ),
            ]
            .width(Length::Fill)
            .height(Length::Shrink),
        )
        .width(width)
        .height(Length::Shrink)
        // Offsets inner containers so they don't overlap modal container border.
        .padding(Padding::from(1.0))
        .style(catalog::container::modal)
        .into()
    }
}
