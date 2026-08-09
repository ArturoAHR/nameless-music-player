use iced::Task;

use crate::{
    tag::models::{Tag, TagGroup},
    track::models::TrackId,
    ui::modals::{
        self, AppModal, ModalController,
        tag_tracks::{Message, Outcome, TagTracksModal},
    },
};

impl ModalController {
    pub fn handle_tag_tracks_modal(
        &mut self,
        message: Message,
        tags: &[Tag],
        tag_groups: &[TagGroup],
    ) -> (Task<modals::Message>, Vec<modals::Outcome>) {
        let Some(AppModal::TagTracks(tag_tracks_modal)) = self.current_modal.as_mut() else {
            return (Task::none(), Vec::new());
        };

        let (task, outcomes) = tag_tracks_modal.update(message, tags, tag_groups);

        let modal_task = task.map(modals::Message::TagTracksModal);

        let outcomes = outcomes
            .into_iter()
            .map(|outcome| match outcome {
                Outcome::Playback(outcome) => modals::Outcome::Playback(outcome),
                Outcome::Modal(outcome) => modals::Outcome::Modal(outcome),
                Outcome::Tag(outcome) => modals::Outcome::Tag(outcome),
            })
            .collect();

        (modal_task, outcomes)
    }

    pub fn open_tag_tracks_modal(&mut self, track_tagging_queue: Vec<TrackId>) {
        self.current_modal = Some(AppModal::TagTracks(TagTracksModal::new(
            track_tagging_queue,
        )));
    }
}
