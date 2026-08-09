use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{
        self,
        modals::{Message, Outcome},
        theme::Theme,
    },
};

impl App {
    pub fn view_modal(&self) -> Option<Element<'_, app::Message, Theme, Renderer>> {
        let modal = self.modal_controller.view(&self.theme, &self.tracks)?;

        Some(modal.map(ui::Message::Modal).map(app::Message::Ui))
    }

    pub fn handle_modal(&mut self, message: Message) -> Task<app::Message> {
        let (task, outcomes) = self
            .modal_controller
            .update(message, &self.tags, &self.tag_groups);

        let component_task = task.map(ui::Message::Modal).map(app::Message::Ui);

        if outcomes.is_empty() {
            return component_task;
        }

        let mut tasks = vec![component_task];

        for outcome in outcomes {
            let outcome = match outcome {
                Outcome::Playback(outcome) => app::Outcome::Playback(outcome),
                Outcome::Modal(outcome) => app::Outcome::Modal(outcome),
                Outcome::Tag(outcome) => app::Outcome::Tag(outcome),
            };

            let outcome_task = self.handle_outcome(outcome);

            tasks.push(outcome_task);
        }

        Task::batch(tasks)
    }

    pub fn notify_modal(&mut self, event: &Event) -> Task<app::Message> {
        self.modal_controller
            .on_event(event)
            .map(ui::Message::Modal)
            .map(app::Message::Ui)
    }
}
