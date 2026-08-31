use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{self, components::status_bar::Message, theme::Theme},
};

impl App {
    pub fn view_status_bar(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.status_bar
            .view(&self.theme, &self.displayed_track_ids, &self.status)
            .map(ui::Message::StatusBar)
            .map(app::Message::Ui)
    }

    pub fn handle_status_bar(&mut self, event: Message) -> Task<app::Message> {
        let (task, _outcomes) = self.status_bar.update(event);

        task.map(ui::Message::StatusBar).map(app::Message::Ui)
    }

    pub fn notify_status_bar(&mut self, event: &Event) -> Task<app::Message> {
        self.status_bar
            .on_event(event)
            .map(ui::Message::StatusBar)
            .map(app::Message::Ui)
    }
}
