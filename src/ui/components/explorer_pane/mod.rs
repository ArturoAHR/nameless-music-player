use iced::{
    Element, Length, Renderer, Task,
    widget::{Space, column, container, scrollable},
};
use rustc_hash::FxHashSet;
use tracing::instrument;

use crate::{
    app::TrackList,
    event::Event,
    outcome::TrackListOutcome,
    tag::{
        index::TrackTagIndex,
        models::{Tag, TagGroup, TagGroupId, TagId},
    },
    ui::{
        components::explorer_pane::widgets::{main_library_section, tag_group_dropdown},
        theme::{Theme, catalog},
    },
};

pub mod handler;
pub mod widgets;

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
        track_list: &'a TrackList,
        track_tag_index: &'a TrackTagIndex,
    ) -> Element<'a, Message, Theme, Renderer> {
        let mut pane_elements: Vec<Element<'a, Message, Theme, Renderer>> = Vec::new();

        pane_elements.extend(main_library_section(theme, track_list));

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

                    tag_group_dropdown(
                        theme,
                        tag_group,
                        tag_group_tags,
                        track_list,
                        track_tag_index,
                        is_tag_group_expanded,
                    )
                }),
        );

        pane_elements.extend([Space::new().height(theme.sizes.space.lg).into()]);

        container(scrollable(column(pane_elements)).style(catalog::scrollable::pane))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(catalog::container::background_surface_raised)
            .into()
    }
}
