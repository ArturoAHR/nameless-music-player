use iced::{
    Element, Length, Padding, Renderer, Task, keyboard,
    widget::{column, container, text},
};
use rustc_hash::FxHashMap;

use crate::{
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome, TagOutcome},
    tag::{
        index::TrackTagIndex,
        models::{Tag, TagGroup, TagId},
    },
    track::models::{Track, TrackId},
    ui::{
        modals::tag_tracks::{
            tag::{get_tag_group_tags, get_tag_index},
            widgets::{header, playback, tag_group_list, tag_list},
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

    current_playback_position: f64,
}

#[derive(Debug, Clone)]
pub enum Message {
    // PlayTrack
    // Resume
    // Pause
    //
    Close,

    SelectTabGroup(usize),
    ToggleTag(TagId),

    Keyboard(keyboard::Event),
    PlaybackScrubbed(f64),
    PlaybackSeeked,
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
    Tag(TagOutcome),
}

impl TagTracksModal {
    pub fn new(track_tagging_queue: Vec<TrackId>) -> Self {
        Self {
            track_tagging_queue,
            track_tagging_queue_cursor: 0,
            tag_groups_cursor: 0,

            current_playback_position: 0.0,
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        tags: &[Tag],
        tag_groups: &[TagGroup],
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
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
            Message::PlaybackScrubbed(position) => {
                self.current_playback_position = position;
            }
            Message::PlaybackSeeked => {}
            Message::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Character(character),
                repeat: false,
                ..
            }) if let Some(character) = character.chars().next()
                && let Some(track_id) = self
                    .track_tagging_queue
                    .get(self.track_tagging_queue_cursor)
                && let Some(tag_group) = tag_groups.get(self.tag_groups_cursor)
                && let Some(tag_index) = get_tag_index(&character)
                && let Some(tag) = get_tag_group_tags(tags, tag_group.id).get(tag_index) =>
            {
                outcomes.push(Outcome::Tag(TagOutcome::ToggleTag(*track_id, tag.id)));
            }
            Message::ToggleTag(_) | Message::Keyboard(_) => {}
        }

        (task, outcomes)
    }

    pub fn on_event(&mut self, _event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(
        &self,
        theme: &Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
        track_tag_index: &'a TrackTagIndex,
    ) -> Element<'a, Message, Theme, Renderer> {
        let width = 1000.0;
        let height = 770.0;

        let current_tagging_track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)
            .unwrap();
        let track = tracks.get(current_tagging_track_id);
        let track_number = self.track_tagging_queue_cursor + 1;
        let track_total = self.track_tagging_queue.len();
        let track_tags = track_tag_index.get_track_tags(*current_tagging_track_id);

        container(
            column![
                header(theme, track, track_number, track_total),
                vertical_separator(),
                playback(
                    theme,
                    tracks,
                    Some(current_tagging_track_id),
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
                container(text("Keyboard controls"))
                    .height(140.0)
                    .width(Length::Fill),
                vertical_separator(),
                container(text("Footer")).height(84.0).width(Length::Fill),
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
