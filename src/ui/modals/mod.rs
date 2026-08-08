use iced::{Element, Renderer, Task};

use crate::{app::Outcome, event::Event, track::models::TrackId, ui::theme::Theme};

pub mod handler;

pub enum AppModal {
    ManageTags(()),
    TagTracks(()),
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenManageTagsModal,
    OpenTagTracksModal(Vec<TrackId>),
    CloseModal,
    // ManageTagsModal(manage_tags::Message)
    // TagTracksModal(tag_tracks::Message)
}

#[derive(Default)]
pub struct ModalController {
    current_modal: Option<AppModal>,
}

impl ModalController {
    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let outcome = Vec::new();

        match message {
            Message::OpenManageTagsModal => {
                self.current_modal = Some(AppModal::ManageTags(()));
            }
            Message::OpenTagTracksModal(_) => {
                self.current_modal = Some(AppModal::TagTracks(()));
            }
            Message::CloseModal => {
                self.current_modal = None;
            } //
              // Message::ManageTagsModal(message) => {
              //     self.handle_manage_tags_modal(message);
              // }
              // Message::TagTracksModal(message) => {
              //     self.handle_tag_tracks_modal(message);
              // }
        }

        (task, outcome)
    }

    pub fn on_event(&mut self, _event: &Event) -> Task<Message> {
        let task = Task::none();

        let Some(current_modal) = self.current_modal.as_mut() else {
            return Task::none();
        };

        match current_modal {
            AppModal::ManageTags(_manage_tags_modal) => {
                // task = manage_tags_modal.on_event(event);
            }
            AppModal::TagTracks(_tag_tracks_modal) => {
                // task = tag_tracks_modal.on_event(event);
            }
        }

        task
    }

    pub fn view<'a>(
        &self,
        _theme: &Theme,
        // tracks: &FxHashMap<TrackId, Track>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        let modal = None;

        match self.current_modal.as_ref()? {
            AppModal::ManageTags(_manage_tags_modal) => {
                //modal = manage_tags_modal.view(theme)
            }
            AppModal::TagTracks(_tag_tracks_modal) => {
                //modal = tag_tracks_modal.view(theme, track)
            }
        }

        modal
    }

    pub fn is_modal_active(&self) -> bool {
        self.current_modal.is_some()
    }
}
