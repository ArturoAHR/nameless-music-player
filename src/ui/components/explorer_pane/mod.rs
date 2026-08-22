use iced::{
    Element, Length, Renderer, Task,
    widget::{button, column, container, row, scrollable, text},
};
use rustc_hash::FxHashSet;
use tracing::instrument;

use crate::{
    event::Event,
    outcome::TrackListOutcome,
    tag::models::{Tag, TagGroup, TagGroupId, TagId},
    ui::{
        theme::{Theme, catalog},
        widgets::{
            icons::{self, icon},
            separator::vertical_separator,
        },
    },
};

pub mod handler;

#[derive(Debug, Default)]
pub struct ExplorerPane {
    tag_groups_expanded: FxHashSet<TagGroupId>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectedMainLibrary,
    SelectedTag(TagId),
    ToggleTagGroup(TagGroupId),
}

#[derive(Debug, Clone)]
pub enum Outcome {
    TrackList(TrackListOutcome),
}

impl ExplorerPane {
    #[instrument(skip(self), level = "debug")]
    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::SelectedMainLibrary => {
                outcomes.push(Outcome::TrackList(
                    TrackListOutcome::DisplayMainLibraryTrackList,
                ));
            }
            Message::SelectedTag(tag_id) => {
                outcomes.push(Outcome::TrackList(TrackListOutcome::DisplayTagTrackList(
                    tag_id,
                )));
            }
            Message::ToggleTagGroup(tag_group_id) => {
                if self.tag_groups_expanded.contains(&tag_group_id) {
                    self.tag_groups_expanded.remove(&tag_group_id);
                } else {
                    self.tag_groups_expanded.insert(tag_group_id);
                }
            }
        }

        (task, outcomes)
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
            button(row![icon(icons::MUSICAL_NOTE), text("Music")])
                .on_press(Message::SelectedMainLibrary)
                .into(),
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
                    let mut dropdown_elements: Vec<Element<'a, Message, Theme, Renderer>> = vec![
                        button(row![icon(icons::CHEVRON_DOWN), text(&tag_group.name)])
                            .on_press(Message::ToggleTagGroup(tag_group.id))
                            .into(),
                    ];

                    // Add check to conditionally add tags below.
                    if self.tag_groups_expanded.contains(&tag_group.id) {
                        dropdown_elements.extend(tag_group_tags.iter().map(|tag_group_tag| {
                            button(row![icon(icons::TAG), text(&tag_group_tag.name)])
                                .on_press(Message::SelectedTag(tag_group_tag.id))
                                .into()
                        }));
                    }

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
