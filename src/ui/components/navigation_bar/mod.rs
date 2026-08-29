use iced::{
    Element, Length, Padding, Renderer, Task, alignment,
    widget::{Space, button, container, row},
};
use tracing::instrument;

use crate::{
    event::Event,
    outcome::ModalOutcome,
    ui::{
        components::navigation_bar::navigation_bar_menu::main_menu_dropdown,
        theme::Theme,
        widgets::icons::{self, icon},
    },
};

pub mod handler;
pub mod navigation_bar_menu;

#[derive(Debug)]
pub struct NavigationBar {}

#[derive(Debug, Clone)]
pub enum Message {
    SelectedScanDirectoryOption,
    SelectedEditMenuManageTagsOption,
    OpenAdvancedSearchModal,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Modal(ModalOutcome),
    OpenSelectDirectoryDialog,
}

impl NavigationBar {
    #[instrument(skip(self), level = "debug")]
    pub fn update(&mut self, event: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match event {
            Message::SelectedScanDirectoryOption => {
                outcomes.push(Outcome::OpenSelectDirectoryDialog);
            }
            Message::SelectedEditMenuManageTagsOption => {
                outcomes.push(Outcome::Modal(ModalOutcome::OpenManageTagsModal));
            }
            Message::OpenAdvancedSearchModal => {
                outcomes.push(Outcome::Modal(ModalOutcome::OpenAdvancedSearchModal));
            }
        }

        (task, outcomes)
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(&'a self, theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
        container(
            row![
                main_menu_dropdown(theme),
                Space::new().width(Length::Fill),
                button(icon(icons::SEARCH)).on_press(Message::OpenAdvancedSearchModal)
            ]
            .align_y(alignment::Vertical::Center)
            .padding(Padding::from([0.0, theme.sizes.space.lg]))
            .height(Length::Fill),
        )
        .height(Length::Fixed(theme.sizes.component.nav_bar_height))
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette.surface_sunken.into()),
            ..container::Style::default()
        })
        .into()
    }
}
