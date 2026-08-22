use iced::{
    Element, Length, Padding, Renderer, Task, alignment,
    widget::{Space, button, column, container, row, scrollable, text},
};
use iced_palace::widget::ellipsized_text;
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
        theme: &Theme,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
    ) -> Element<'a, Message, Theme, Renderer> {
        let mut pane_elements: Vec<Element<'a, Message, Theme, Renderer>> = vec![
            container(
                text("LIBRARY")
                    .size(theme.sizes.font.body)
                    .color(theme.palette.text_muted),
            )
            .width(Length::Fill)
            .padding(Padding::from([theme.sizes.space.xxl, theme.sizes.space.xl]).bottom(10))
            .into(),
            button(
                row![
                    icon(icons::MUSICAL_NOTE)
                        .size(theme.sizes.font.body)
                        .height(16)
                        .align_y(alignment::Vertical::Top),
                    ellipsized_text("Music")
                ]
                .align_y(alignment::Vertical::Center)
                .spacing(theme.sizes.space.lg),
            )
            .on_press(Message::SelectedMainLibrary)
            .height(36)
            .width(Length::Fill)
            .padding([theme.sizes.space.md, theme.sizes.space.xl])
            .into(),
            Space::new().height(6).into(),
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
                    let is_tag_group_expanded = self.tag_groups_expanded.contains(&tag_group.id);

                    let chevron = if is_tag_group_expanded {
                        icons::CHEVRON_DOWN
                    } else {
                        icons::CHEVRON_UP
                    };

                    let mut dropdown_elements: Vec<Element<'a, Message, Theme, Renderer>> = vec![
                        button(
                            row![
                                icon(chevron)
                                    .size(theme.sizes.font.caption)
                                    .color(theme.palette.text_muted),
                                ellipsized_text(tag_group.name.to_uppercase())
                                    .size(theme.sizes.font.body)
                                    .color(theme.palette.text_muted)
                            ]
                            .width(Length::Fill)
                            .align_y(alignment::Vertical::Center)
                            .spacing(theme.sizes.space.lg),
                        )
                        .width(Length::Fill)
                        .padding(
                            Padding::from([theme.sizes.space.xxl, theme.sizes.space.xl])
                                .bottom(theme.sizes.space.sm),
                        )
                        .on_press(Message::ToggleTagGroup(tag_group.id))
                        .into(),
                    ];

                    if is_tag_group_expanded {
                        dropdown_elements.extend(tag_group_tags.iter().map(|tag_group_tag| {
                            button(
                                row![
                                    icon(icons::TAG)
                                        .size(theme.sizes.font.body)
                                        .height(16)
                                        .align_y(alignment::Vertical::Top),
                                    ellipsized_text(&tag_group_tag.name)
                                ]
                                .width(Length::Fill)
                                .align_y(alignment::Vertical::Center)
                                .spacing(theme.sizes.space.lg),
                            )
                            .height(36)
                            .width(Length::Fill)
                            .padding(
                                Padding::from([theme.sizes.space.md, 52.0])
                                    .right(theme.sizes.space.sm),
                            )
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
