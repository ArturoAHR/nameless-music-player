use iced::Task;

use crate::{
    track::models::TrackId,
    ui::modals::{
        self, AppModal, ModalController,
        tag_tracks::{Message, TagTracksModal},
    },
};

impl ModalController {
    pub fn handle_tag_tracks_modal(
        &mut self,
        message: Message,
    ) -> (Task<modals::Message>, Vec<modals::Outcome>) {
        let Some(AppModal::TagTracks(tag_tracks_modal)) = self.current_modal.as_mut() else {
            return (Task::none(), Vec::new());
        };

        let (task, _outcomes) = tag_tracks_modal.update(message);

        let modal_task = task.map(modals::Message::TagTracksModal);

        (modal_task, Vec::new())
    }

    pub fn open_tag_tracks_modal(&mut self, track_tagging_queue: Vec<TrackId>) {
        self.current_modal = Some(AppModal::TagTracks(TagTracksModal::new(
            track_tagging_queue,
        )));
    }
}
