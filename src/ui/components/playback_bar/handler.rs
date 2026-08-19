use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{
        self,
        components::playback_bar::{Message, Outcome},
        theme::Theme,
    },
};

impl App {
    pub fn view_playback_bar(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.playback_bar
            .view(&self.theme, &self.tracks, &self.playback_queue)
            .map(ui::Message::PlaybackBar)
            .map(app::Message::Ui)
    }

    pub fn handle_playback_bar(&mut self, message: Message) -> Task<app::Message> {
        let (task, outcomes) = self
            .playback_bar
            .update(message, &self.playback_controller.status);
        let component_task = task.map(ui::Message::PlaybackBar).map(app::Message::Ui);

        if outcomes.is_empty() {
            return component_task;
        }

        let mut tasks = vec![component_task];

        for outcome in outcomes {
            let outcome = match outcome {
                Outcome::Playback(playback_outcome) => app::Outcome::Playback(playback_outcome),
            };

            let outcome_task = self.handle_outcome(outcome);

            tasks.push(outcome_task);
        }

        Task::batch(tasks)
    }

    pub fn notify_playback_bar(&mut self, event: &Event) -> Task<app::Message> {
        self.playback_bar
            .on_event(event, &self.current_playback_owner)
            .map(ui::Message::PlaybackBar)
            .map(app::Message::Ui)
    }
}
