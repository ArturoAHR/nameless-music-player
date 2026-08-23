use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{
        self,
        components::explorer_pane::{Message, Outcome},
        theme::Theme,
    },
};

impl App {
    pub fn view_explorer_pane(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.explorer_pane
            .view(
                &self.theme,
                &self.tags,
                &self.tag_groups,
                &self.track_list,
                &self.track_tag_index,
            )
            .map(ui::Message::ExplorerPane)
            .map(app::Message::Ui)
    }

    pub fn handle_explorer_pane(&mut self, message: Message) -> Task<app::Message> {
        let (task, outcomes) = self.explorer_pane.update(message);
        let component_task = task.map(ui::Message::ExplorerPane).map(app::Message::Ui);

        if outcomes.is_empty() {
            return component_task;
        }

        let mut tasks = vec![component_task];

        for outcome in outcomes {
            let outcome = match outcome {
                Outcome::TrackList(outcome) => app::Outcome::TrackList(outcome),
            };

            let outcome_task = self.handle_outcome(outcome);

            tasks.push(outcome_task);
        }

        Task::batch(tasks)
    }

    pub fn notify_explorer_pane(&mut self, event: &Event) -> Task<app::Message> {
        self.explorer_pane
            .on_event(event)
            .map(ui::Message::ExplorerPane)
            .map(app::Message::Ui)
    }
}
