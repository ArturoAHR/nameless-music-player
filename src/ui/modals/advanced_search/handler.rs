use iced::Task;

use crate::ui::modals::{
    self, AppModal, ModalController,
    advanced_search::{AdvancedSearchModal, Message, Outcome},
};

impl ModalController {
    pub fn handle_advanced_search_modal(
        &mut self,
        message: Message,
    ) -> (Task<modals::Message>, Vec<modals::Outcome>) {
        let Some(AppModal::AdvancedSearch(advanced_search_modal)) = self.modal.as_mut() else {
            return (Task::none(), Vec::new());
        };

        let (task, outcomes) = advanced_search_modal.update(message);

        let modal_task = task.map(modals::Message::AdvancedSearchModal);

        let outcomes = outcomes
            .into_iter()
            .map(|outcome| match outcome {
                Outcome::Modal(outcome) => modals::Outcome::Modal(outcome),
            })
            .collect();

        (modal_task, outcomes)
    }

    pub fn open_advanced_search_modal(&mut self) {
        self.modal = Some(AppModal::AdvancedSearch(AdvancedSearchModal::default()));
    }
}
