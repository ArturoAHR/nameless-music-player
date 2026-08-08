use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App},
    event::Event,
    ui::{self, modals::Message, theme::Theme},
};

impl App {
    pub fn view_modal(&self) -> Option<Element<'_, app::Message, Theme, Renderer>> {
        let modal = self.modal_controller.view(&self.theme)?;

        Some(modal.map(ui::Message::Modal).map(app::Message::Ui))
    }

    pub fn handle_modal(&mut self, message: Message) -> Task<app::Message> {
        let (task, _outcomes) = self.modal_controller.update(message);

        task.map(ui::Message::Modal).map(app::Message::Ui)
    }

    pub fn notify_modal(&mut self, event: &Event) -> Task<app::Message> {
        self.modal_controller
            .on_event(event)
            .map(ui::Message::Modal)
            .map(app::Message::Ui)
    }
}
