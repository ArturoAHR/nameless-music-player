use iced::{
    Element, Length, Renderer, Task,
    widget::{center, column, container, text},
};

use crate::{
    event::Event,
    outcome::PlaybackOutcome,
    track::models::TrackId,
    ui::{
        modals::ModalController,
        theme::{Theme, catalog},
        widgets::separator::vertical_separator,
    },
};

pub mod handler;

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
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalController),
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

    pub fn update(&mut self, _message: Message) -> (Task<Message>, Vec<Outcome>) {
        (Task::none(), Vec::new())
    }

    pub fn on_event(&mut self, _event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(&self, _theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
        let width = 1000.0;
        let height = 770.0;

        container(
            column![
                container(text("Header")).height(128.0).width(Length::Fill),
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
