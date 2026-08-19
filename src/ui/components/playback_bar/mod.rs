use iced::{
    Alignment, Element, Font, Length, Padding, Renderer, Task,
    widget::{Space, button, column, container, row, slider, text},
};
use iced_palace::widget::ellipsized_text;
use rustc_hash::FxHashMap;
use tracing::{instrument, trace};

use crate::{
    app::PlaybackOwner,
    event::Event,
    outcome::PlaybackOutcome,
    playback::{
        controller::PlaybackControllerStatus,
        queue::{PlaybackQueue, PlaybackQueueOrder, PlaybackRepeatMode},
    },
    track::{
        models::{Track, TrackId},
        utils::{get_track_duration_label, get_track_label},
    },
    ui::{
        components::playback_bar::widgets::volume_bar,
        theme::{Theme, catalog},
        utils::label::format_duration,
        widgets::icons::{self, icon},
    },
};

pub mod handler;
pub mod widgets;

#[derive(Debug)]
pub struct PlaybackBar {
    pub current_position: f64,
    pub current_playing_track_id: Option<TrackId>,

    pub status: PlaybackBarStatus,

    // TODO: These values must live in the app state, declaring them here for mocking UI.
    volume_percentage: u8,
    muted: bool,
}

#[derive(Debug)]
pub enum PlaybackBarStatus {
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub enum Message {
    Pause,
    Resume,
    PlayNext,
    PlayPrevious,
    Scrubbed(f64),
    Seeked,
    ChangeVolumePercentage(u8),
    MutePlayback,
    CycleRepeatMode,
    CycleQueueOrder,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Playback(PlaybackOutcome),
}

/*
 * TODO:
 * - Fix icons with consistent design.
 */
impl PlaybackBar {
    pub fn new() -> Self {
        Self {
            status: PlaybackBarStatus::Playing,
            current_position: 0.0,
            current_playing_track_id: None,

            muted: false,
            volume_percentage: 100,
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub fn update(
        &mut self,
        message: Message,
        playback_controller_status: &PlaybackControllerStatus,
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Scrubbed(position) => {
                self.current_position = position;

                if matches!(
                    playback_controller_status,
                    PlaybackControllerStatus::Playing
                ) {
                    outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
                }
            }

            Message::Seeked => {
                let pre_seek_status = match self.status {
                    PlaybackBarStatus::Playing => PlaybackControllerStatus::Playing,
                    PlaybackBarStatus::Paused => PlaybackControllerStatus::Stopped,
                };

                outcomes.push(Outcome::Playback(PlaybackOutcome::Seek {
                    timestamp: self.current_position.round() as u64,
                    post_seek_status: Some(pre_seek_status),
                }));
            }
            Message::Resume if self.current_playing_track_id.is_some() => {
                self.status = PlaybackBarStatus::Playing;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Resume));
            }
            Message::Pause if self.current_playing_track_id.is_some() => {
                self.status = PlaybackBarStatus::Paused;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
            }
            Message::PlayNext => outcomes.push(Outcome::Playback(PlaybackOutcome::PlayNext)),
            Message::PlayPrevious => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::PlayPrevious));
            }

            // TODO: Add playback outcome to change volume
            Message::ChangeVolumePercentage(volume_percentage) => {
                self.volume_percentage = volume_percentage;
            }
            // TODO: Add playback outcome to change volume
            Message::MutePlayback => {
                self.muted = !self.muted;
            }
            // TODO: Wire these two changes in upper for queue functionality
            Message::CycleRepeatMode => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::CycleRepeatMode));
            }
            Message::CycleQueueOrder => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::CycleOrder));
            }

            Message::Resume | Message::Pause => {}
        }

        (task, outcomes)
    }

    #[allow(clippy::single_match)]
    #[instrument(skip(self), level = "debug")]
    pub fn on_event(
        &mut self,
        event: &Event,
        current_playback_owner: &PlaybackOwner,
    ) -> Task<Message> {
        let task = Task::none();

        if !matches!(current_playback_owner, PlaybackOwner::PlaybackBar) {
            trace!("Playback Bar does not own the playback currently, ignoring event");

            return task;
        }

        match event {
            Event::AttemptedPlayingTrack => {
                self.status = PlaybackBarStatus::Playing;

                self.current_position = 0.0;
            }
            Event::PlaybackProgressed(position) => {
                self.current_position = *position;
            }
            Event::ActiveTrackChanged(track_id) => {
                self.current_playing_track_id = *track_id;
            }
            _ => {}
        }

        task
    }

    pub fn view<'a>(
        &'a self,
        theme: &Theme,
        tracks: &FxHashMap<TrackId, Track>,
        playback_queue: &PlaybackQueue,
    ) -> Element<'a, Message, Theme, Renderer> {
        let mut total_frames = 1.0;
        let mut current_position = 0.0;

        let mut track_name_label = String::new();
        let (track_duration_timestamp, current_position_timestamp) = self
            .current_playing_track_id
            .and_then(|track_id| tracks.get(&track_id))
            .map_or_else(
                || ("0:00".to_owned(), "0:00".to_owned()),
                |track| {
                    total_frames = track.frames as f64;
                    current_position = self.current_position;
                    track_name_label = get_track_label(track);

                    (
                        get_track_duration_label(track),
                        format_duration(
                            (current_position / track.sample_rate as f64).floor() as u64
                        ),
                    )
                },
            );

        let play_previous = button(icon(icons::PLAY_PREVIOUS)).on_press(Message::PlayPrevious);
        let play_next = button(icon(icons::PLAY_NEXT)).on_press(Message::PlayNext);
        let play_button = match self.status {
            PlaybackBarStatus::Paused => button(icon(icons::PLAY)).on_press(Message::Resume),
            PlaybackBarStatus::Playing => button(icon(icons::PAUSE)).on_press(Message::Pause),
        };

        let current_time_label =
            format!("{current_position_timestamp} / {track_duration_timestamp}");

        let repeat_mode_icon = match playback_queue.repeat_mode {
            PlaybackRepeatMode::NoRepeat => icons::MENU, //Placeholder
            PlaybackRepeatMode::Repeat => icons::REPEAT,
            PlaybackRepeatMode::RepeatOne => icons::PLAY, //Placeholder
        };

        let queue_order_icon = match playback_queue.order {
            PlaybackQueueOrder::Sequential => icons::SEQUENTIAL,
            PlaybackQueueOrder::Shuffle => icons::SHUFFLE,
        };

        container(
            row![
                row![play_previous, play_button, play_next].spacing(theme.sizes.space.md),
                column![
                    row![
                        ellipsized_text(track_name_label),
                        Space::new().width(Length::Fill),
                        text(current_time_label).font(Font::MONOSPACE)
                    ],
                    slider(0.0..=total_frames, current_position, Message::Scrubbed)
                        .on_release(Message::Seeked)
                ]
                .spacing(theme.sizes.space.md),
                volume_bar(self.volume_percentage, self.muted),
                button(icon(repeat_mode_icon)).on_press(Message::CycleRepeatMode),
                button(icon(queue_order_icon)).on_press(Message::CycleQueueOrder),
            ]
            .align_y(Alignment::Center)
            .spacing(theme.sizes.space.xxxl),
        )
        .height(Length::Fixed(theme.sizes.component.playback_bar_height))
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .padding(Padding::from(theme.sizes.space.xl))
        .style(catalog::container::background_surface_raised)
        .into()
    }
}

impl Default for PlaybackBar {
    fn default() -> Self {
        Self::new()
    }
}
