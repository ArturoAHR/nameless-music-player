use iced::Task;

use crate::{
    tag::models::{Tag, TagGroup},
    ui::modals::{
        self, AppModal, ModalController,
        manage_tags::{ManageTagsModal, Message, Outcome},
    },
};

impl ModalController {
    pub fn handle_manage_tags_modal(
        &mut self,
        message: Message,
        tags: &[Tag],
        tag_groups: &[TagGroup],
    ) -> (Task<modals::Message>, Vec<modals::Outcome>) {
        let Some(AppModal::ManageTags(manage_tags_modal)) = self.modal.as_mut() else {
            return (Task::none(), Vec::new());
        };

        let (task, outcomes) = manage_tags_modal.update(message, tags, tag_groups);

        let modal_task = task.map(modals::Message::ManageTagsModal);

        let outcomes = outcomes
            .into_iter()
            .map(|outcome| match outcome {
                Outcome::Modal(outcome) => modals::Outcome::Modal(outcome),
                Outcome::Tag(outcome) => modals::Outcome::Tag(outcome),
            })
            .collect();

        (modal_task, outcomes)
    }

    pub fn open_manage_tags_modal(&mut self) {
        self.modal = Some(AppModal::ManageTags(ManageTagsModal::new()));
    }
}
