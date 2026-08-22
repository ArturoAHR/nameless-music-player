use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{
        self,
        components::main_pane::{Message, Outcome},
        theme::Theme,
    },
};

impl App {
    pub fn view_main_pane(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.main_pane
            .view(
                &self.theme,
                &self.tracks,
                &self.displayed_track_ids,
                self.playback_bar.playing_track_id.as_ref(),
            )
            .map(ui::Message::MainPane)
            .map(app::Message::Ui)
    }

    pub fn handle_main_pane(&mut self, message: Message) -> Task<app::Message> {
        let (task, outcomes) = self.main_pane.update(message, &self.displayed_track_ids);
        let component_task = task.map(ui::Message::MainPane).map(app::Message::Ui);

        if outcomes.is_empty() {
            return component_task;
        }

        let mut tasks = vec![component_task];

        for outcome in outcomes {
            let outcome = match outcome {
                Outcome::Playback(playback_outcome) => app::Outcome::Playback(playback_outcome),
                Outcome::Modal(modal_outcome) => app::Outcome::Modal(modal_outcome),
            };

            let outcome_task = self.handle_outcome(outcome);

            tasks.push(outcome_task);
        }

        Task::batch(tasks)
    }

    pub fn notify_main_pane(&mut self, event: &Event) -> Task<app::Message> {
        self.main_pane
            .on_event(event)
            .map(ui::Message::MainPane)
            .map(app::Message::Ui)
    }
}
