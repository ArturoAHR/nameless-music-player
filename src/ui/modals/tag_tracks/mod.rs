use iced::{
    Element, Length, Renderer, Task, keyboard,
    widget::{column, container, text},
};
use rustc_hash::FxHashMap;
use tracing::info;

use crate::{
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome},
    track::models::{Track, TrackId},
    ui::{
        modals::tag_tracks::widgets::header,
        theme::{Theme, catalog},
        widgets::separator::vertical_separator,
    },
};

pub mod handler;
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
    Keyboard(keyboard::Event),
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
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

    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Close => outcomes.push(Outcome::Modal(ModalOutcome::CloseModal)),
            Message::Keyboard(keyboard::Event::KeyPressed {
                key, repeat: false, ..
            }) => {
                info!("{key:?}");
            }
            Message::Keyboard(_) => {}
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
    ) -> Element<'a, Message, Theme, Renderer> {
        let width = 1000.0;
        let height = 770.0;

        let current_tagging_track_id = self
            .track_tagging_queue
            .get(self.track_tagging_queue_cursor)
            .unwrap();
        let track = tracks.get(current_tagging_track_id).unwrap();
        let track_number = self.track_tagging_queue_cursor + 1;
        let track_total = self.track_tagging_queue.len();

        container(
            column![
                header(theme, track, track_number, track_total),
                vertical_separator(),
                container(text("Playback"))
                    .height(100.0)
                    .width(Length::Fill),
                vertical_separator(),
                container(text("Tag Groups"))
                    .height(140.0)
                    .width(Length::Fill),
                vertical_separator(),
                container(text("Tags"))
                    .height(Length::Fill)
                    .width(Length::Fill),
                vertical_separator(),
                container(text("Keyboard controls"))
                    .height(140.0)
                    .width(Length::Fill),
                vertical_separator(),
                container(text("Footer")).height(84.0).width(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(width)
        .height(height)
        .style(catalog::container::modal)
        .into()
    }
}
