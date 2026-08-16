use iced::{
    Element, Length, Padding, Renderer, Task,
    widget::{Space, column, container},
};
use tracing::instrument;

use crate::{
    event::Event,
    outcome::ModalOutcome,
    tag::models::{Tag, TagGroup},
    ui::theme::{Theme, catalog},
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
        let width = 1000.0;

        container(Space::new())
            .width(width)
            .height(Length::Shrink)
            // Offsets inner containers so they don't overlap modal container border.
            .padding(Padding::from(1.0))
            .style(catalog::container::modal)
            .into()
    }
}
