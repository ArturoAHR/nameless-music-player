use iced::{
    Element, Renderer, Task,
    widget::{center, container, text},
};

use crate::{
    event::Event,
    outcome::PlaybackOutcome,
    track::models::TrackId,
    ui::theme::{Theme, catalog},
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
}

pub enum Outcome {
    Playback(PlaybackOutcome),
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
        container(center(text("Hi")))
            .width(100.0)
            .height(80.0)
            .style(catalog::container::context_menu)
            .into()
    }
}
