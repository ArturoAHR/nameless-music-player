use iced::{Element, Renderer, Task, keyboard};
use rustc_hash::FxHashMap;

use crate::{
    app::{self},
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome, TagOutcome},
    tag::models::{Tag, TagGroup},
    track::models::{Track, TrackId},
    ui::{modals::tag_tracks::TagTracksModal, theme::Theme},
};

pub mod handler;
pub mod tag_tracks;

pub enum AppModal {
    ManageTags(()),
    TagTracks(TagTracksModal),
}

#[derive(Default)]
pub struct ModalController {
    current_modal: Option<AppModal>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Keyboard(keyboard::Event),

    OpenManageTagsModal,
    OpenTagTracksModal(Vec<TrackId>),
    CloseModal,
    // ManageTagsModal(manage_tags::Message)
    TagTracksModal(tag_tracks::Message),
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
    Tag(TagOutcome),
}

impl ModalController {
    pub fn update(
        &mut self,
        message: Message,
        tags: &[Tag],
        tag_groups: &[TagGroup],
    ) -> (Task<Message>, Vec<Outcome>) {
        let mut task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::OpenManageTagsModal => {
                self.current_modal = Some(AppModal::ManageTags(()));
            }
            Message::OpenTagTracksModal(track_tagging_queue) => {
                self.open_tag_tracks_modal(track_tagging_queue);
            }
            Message::CloseModal => {
                self.current_modal = None;
            }
            Message::Keyboard(event)
                if let Some(AppModal::TagTracks(_)) = self.current_modal.as_ref() =>
            {
                (task, outcomes) = self.handle_tag_tracks_modal(
                    tag_tracks::Message::Keyboard(event),
                    tags,
                    tag_groups,
                );
            }
            Message::Keyboard(_) => {}
            // Message::ManageTagsModal(message) => {
            //     self.handle_manage_tags_modal(message);
            // }
            Message::TagTracksModal(message) => {
                (task, outcomes) = self.handle_tag_tracks_modal(message, tags, tag_groups);
            }
        }

        (task, outcomes)
    }

    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        let mut task = Task::none();

        let Some(current_modal) = self.current_modal.as_mut() else {
            return Task::none();
        };

        match current_modal {
            AppModal::ManageTags(_manage_tags_modal) => {
                // task = manage_tags_modal.on_event(event);
            }
            AppModal::TagTracks(tag_tracks_modal) => {
                task = tag_tracks_modal
                    .on_event(event)
                    .map(Message::TagTracksModal);
            }
        }

        task
    }

    pub fn view<'a>(
        &self,
        theme: &Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        let mut modal = None;

        match self.current_modal.as_ref()? {
            AppModal::ManageTags(_manage_tags_modal) => {
                //modal = manage_tags_modal.view(theme)
            }
            AppModal::TagTracks(tag_tracks_modal) => {
                modal = Some(
                    tag_tracks_modal
                        .view(theme, tracks)
                        .map(Message::TagTracksModal),
                );
            }
        }

        modal
    }

    pub fn close_modal(&mut self) -> Task<app::Message> {
        self.current_modal = None;

        // TODO: Add saving current track tags index to the database on closing tag tracks modal.

        Task::none()
    }

    pub fn is_modal_active(&self) -> bool {
        self.current_modal.is_some()
    }
}
