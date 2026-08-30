use iced::{Element, Renderer, Task, keyboard};
use rustc_hash::FxHashMap;
use tracing::instrument;

use crate::{
    app::{self, PlaybackOwner},
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome, TagOutcome, TrackListOutcome},
    playback::controller::PlaybackControllerStatus,
    tag::{
        index::TrackTagIndex,
        models::{Tag, TagGroup},
    },
    track::models::{Track, TrackId},
    ui::{
        modals::{
            advanced_search::AdvancedSearchModal, manage_tags::ManageTagsModal,
            tag_tracks::TagTracksModal,
        },
        theme::Theme,
    },
};

pub mod advanced_search;
pub mod handler;
pub mod manage_tags;
pub mod tag_tracks;

pub enum AppModal {
    ManageTags(ManageTagsModal),
    TagTracks(TagTracksModal),
    AdvancedSearch(AdvancedSearchModal),
}

#[derive(Default)]
pub struct ModalController {
    modal: Option<AppModal>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Keyboard(keyboard::Event),

    OpenManageTagsModal,
    OpenTagTracksModal(Vec<TrackId>),
    CloseModal,
    ManageTagsModal(manage_tags::Message),
    TagTracksModal(tag_tracks::Message),
    AdvancedSearchModal(advanced_search::Message),
}

pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
    Tag(TagOutcome),
    TrackList(TrackListOutcome),
}

impl ModalController {
    #[instrument(skip_all, level = "debug")]
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
            Message::OpenManageTagsModal => {
                self.open_manage_tags_modal();
            }
            Message::OpenTagTracksModal(track_tagging_queue) => {
                self.open_tag_tracks_modal(track_tagging_queue);
            }
            Message::CloseModal => {
                self.modal = None;
            }
            Message::Keyboard(event) if let Some(AppModal::TagTracks(_)) = self.modal.as_ref() => {
                (task, outcomes) = self.handle_tag_tracks_modal(
                    tag_tracks::Message::Keyboard(event),
                    tracks,
                    tags,
                    tag_groups,
                    playback_controller_status,
                );
            }
            Message::Keyboard(_) => {}
            Message::ManageTagsModal(message) => {
                (task, outcomes) = self.handle_manage_tags_modal(message, tags, tag_groups);
            }
            Message::TagTracksModal(message) => {
                (task, outcomes) = self.handle_tag_tracks_modal(
                    message,
                    tracks,
                    tags,
                    tag_groups,
                    playback_controller_status,
                );
            }
            Message::AdvancedSearchModal(message) => {
                (task, outcomes) = self.handle_advanced_search_modal(message);
            }
        }

        (task, outcomes)
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(
        &mut self,
        event: &Event,
        current_playback_owner: &PlaybackOwner,
    ) -> Task<Message> {
        let mut task = Task::none();

        let Some(current_modal) = self.modal.as_mut() else {
            return task;
        };

        match current_modal {
            AppModal::ManageTags(manage_tags_modal) => {
                task = manage_tags_modal
                    .on_event(event)
                    .map(Message::ManageTagsModal);
            }
            AppModal::TagTracks(tag_tracks_modal) => {
                task = tag_tracks_modal
                    .on_event(event, current_playback_owner)
                    .map(Message::TagTracksModal);
            }
            AppModal::AdvancedSearch(advanced_search_modal) => {
                task = advanced_search_modal
                    .on_event(event)
                    .map(Message::AdvancedSearchModal);
            }
        }

        task
    }

    #[instrument(skip_all, level = "debug")]
    pub fn view<'a>(
        &'a self,
        theme: &Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
        track_tag_index: &'a TrackTagIndex,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        match self.modal.as_ref()? {
            AppModal::ManageTags(manage_tags_modal) => Some(
                manage_tags_modal
                    .view(theme, tags, tag_groups)
                    .map(Message::ManageTagsModal),
            ),
            AppModal::TagTracks(tag_tracks_modal) => Some(
                tag_tracks_modal
                    .view(theme, tracks, tags, tag_groups, track_tag_index)
                    .map(Message::TagTracksModal),
            ),
            AppModal::AdvancedSearch(advanced_search_modal) => Some(
                advanced_search_modal
                    .view(theme)
                    .map(Message::AdvancedSearchModal),
            ),
        }
    }

    pub fn close_modal(&mut self) -> Task<app::Message> {
        self.modal = None;

        // TODO: Add saving current track tags index to the database on closing tag tracks modal.

        Task::none()
    }

    pub fn is_modal_active(&self) -> bool {
        self.modal.is_some()
    }
}
