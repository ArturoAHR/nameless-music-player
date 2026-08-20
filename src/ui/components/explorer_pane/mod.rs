use iced::{
    Element, Length, Renderer, Task,
    widget::{button, column, container, row, scrollable, text},
};
use tracing::instrument;

use crate::{
    event::Event,
    tag::models::{Tag, TagGroup},
    ui::{
        theme::{Theme, catalog},
        widgets::{
            icons::{self, icon},
            separator::vertical_separator,
        },
    },
};

pub mod handler;

#[derive(Debug)]
pub struct ExplorerPane {}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub enum Outcome {}

impl ExplorerPane {
    #[instrument(skip(self), level = "debug")]
    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        (Task::none(), vec![])
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(
        &'a self,
        _theme: &Theme,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
    ) -> Element<'a, Message, Theme, Renderer> {
        let mut pane_elements: Vec<Element<'a, Message, Theme, Renderer>> = vec![
            container(text("Library")).into(),
            button(row![icon(icons::MUSICAL_NOTE), text("Music")]).into(),
            vertical_separator(),
        ];

        let tag_groups_with_tags: Vec<(&TagGroup, Vec<&Tag>)> = tag_groups
            .iter()
            .map(|tag_group| {
                (
                    tag_group,
                    tags.iter()
                        .filter(|tag| tag.tag_group_id == tag_group.id)
                        .collect(),
                )
            })
            .collect();

        pane_elements.extend(
            tag_groups_with_tags
                .iter()
                .map(|(tag_group, tag_group_tags)| {
                    let mut dropdown_elements: Vec<Element<'a, Message, Theme, Renderer>> =
                        vec![button(row![icon(icons::CHEVRON_DOWN), text(&tag_group.name)]).into()];

                    // Add check to conditionally add tags below.
                    dropdown_elements.extend(tag_group_tags.iter().map(|tag_group_tag| {
                        button(row![icon(icons::TAG), text(&tag_group_tag.name)]).into()
                    }));

                    column(dropdown_elements).into()
                }),
        );

        container(scrollable(column(pane_elements)))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(catalog::container::background_surface_raised)
            .into()
    }
}
