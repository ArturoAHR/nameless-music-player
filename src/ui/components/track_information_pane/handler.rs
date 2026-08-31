use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{self, components::track_information_pane::Message, theme::Theme},
};

impl App {
    pub fn view_track_information_pane(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.track_information_pane
            .view(&self.theme, &self.tracks)
            .map(ui::Message::TrackInformationPane)
            .map(app::Message::Ui)
    }

    pub fn handle_track_information_pane(&mut self, message: Message) -> Task<app::Message> {
        let (task, _outcomes) = self.track_information_pane.update(message);

        task.map(ui::Message::TrackInformationPane)
            .map(app::Message::Ui)
    }

    pub fn notify_track_information_pane(&mut self, event: &Event) -> Task<app::Message> {
        self.track_information_pane
            .on_event(event, &self.playback_owner)
            .map(ui::Message::TrackInformationPane)
            .map(app::Message::Ui)
    }
}
