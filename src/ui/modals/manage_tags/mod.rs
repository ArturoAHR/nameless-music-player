use iced::{
    Element, Length, Padding, Renderer, Task,
    widget::{column, container, row, text},
};
use tracing::instrument;

use crate::{
    event::Event,
    outcome::ModalOutcome,
    tag::models::{Tag, TagGroup},
    ui::{
        theme::{Theme, catalog},
        widgets::separator::{horizontal_separator, vertical_separator},
    },
};

pub mod handler;

#[derive(Default)]
pub struct ManageTagsModal {}

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub enum Outcome {
    Modal(ModalOutcome),
}

impl ManageTagsModal {
    pub fn new() -> Self {
        Self {}
    }

    #[instrument(
        skip(self, tags, tag_groups)
        fields( tags_len = tags.len(), tag_groups_len = tag_groups.len()),
        level = "debug"
    )]
    pub fn update(
        &mut self,
        message: Message,
        tags: &[Tag],
        tag_groups: &[TagGroup],
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Close => outcomes.push(Outcome::Modal(ModalOutcome::CloseModal)),
        }

        (task, outcomes)
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        let task = Task::none();

        task
    }

    #[instrument(skip_all, level = "debug")]
    pub fn view<'a>(
        &self,
        theme: &Theme,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
    ) -> Element<'a, Message, Theme, Renderer> {
        container(column![
            container(text("Header")).height(64.0).width(Length::Fill),
            vertical_separator(),
            row![
                column![
                    container(text("Tag groups list"))
                        .height(Length::Fill)
                        .width(Length::Fill),
                    vertical_separator(),
                    container(text("Tag groups input"))
                        .height(80.0)
                        .width(Length::Fill)
                ],
                horizontal_separator(),
                column![
                    container(text("Tags group tags list"))
                        .height(Length::Fill)
                        .width(Length::Fill),
                    vertical_separator(),
                    container(text("Tag groups tags input"))
                        .height(80.0)
                        .width(Length::Fill)
                ]
            ]
            .height(Length::Fill)
            .width(Length::Fill),
            vertical_separator(),
            container(text("Footer")).height(50.0).width(Length::Fill)
        ])
        .width(670.0)
        .height(500.0)
        // Offsets inner containers so they don't overlap modal container border.
        .padding(Padding::from(1.0))
        .style(catalog::container::modal)
        .into()
    }
}
