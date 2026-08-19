use iced::{
    Element, Length, Padding, Renderer, Task,
    widget::{column, container, row},
};
use tracing::instrument;

use crate::{
    event::Event,
    outcome::{ModalOutcome, TagOutcome},
    tag::models::{Tag, TagGroup, TagGroupId, TagId},
    ui::{
        modals::manage_tags::widgets::{footer, header, tag_group_pane, tag_group_tags_pane},
        theme::{Theme, catalog},
        widgets::separator::{horizontal_separator, vertical_separator},
    },
};

pub mod handler;
pub mod widgets;

#[derive(Default)]
pub struct ManageTagsModal {
    selected_tag_group_id: Option<TagGroupId>,
    new_tag_group_name_input_text: String,
    new_tag_name_input_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Close,
    SelectTagGroup(TagGroupId),
    NewTagGroupNameInputTextChanged(String),
    AddNewTagGroup,
    RemoveTagGroup(TagGroupId),
    NewTagNameInputTextChanged(String),
    AddNewTag,
    RemoveTag(TagId),
}

pub enum Outcome {
    Modal(ModalOutcome),
    Tag(TagOutcome),
}

impl ManageTagsModal {
    pub fn new() -> Self {
        Self {
            selected_tag_group_id: None,
            new_tag_group_name_input_text: String::new(),
            new_tag_name_input_text: String::new(),
        }
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
            Message::SelectTagGroup(tag_group_id) => {
                self.selected_tag_group_id = Some(tag_group_id);
            }
            Message::NewTagGroupNameInputTextChanged(new_tag_group_name_input_text) => {
                self.new_tag_group_name_input_text = new_tag_group_name_input_text;
            }
            Message::NewTagNameInputTextChanged(new_tag_name_input_text) => {
                self.new_tag_name_input_text = new_tag_name_input_text;
            }
            Message::AddNewTag if let Some(selected_tag_group_id) = self.selected_tag_group_id => {
                outcomes.push(Outcome::Tag(TagOutcome::AddNewTag(
                    selected_tag_group_id,
                    self.new_tag_name_input_text.clone(),
                )));

                self.new_tag_name_input_text.clear();
            }
            Message::AddNewTagGroup => {
                outcomes.push(Outcome::Tag(TagOutcome::AddNewTagGroup(
                    self.new_tag_group_name_input_text.clone(),
                )));

                self.new_tag_group_name_input_text.clear();
            }
            Message::AddNewTag => {
                // Not reachable
            }

            Message::RemoveTagGroup(tag_group_id) => {
                outcomes.push(Outcome::Tag(TagOutcome::RemoveTagGroup(tag_group_id)));
            }
            Message::RemoveTag(tag_id) => {
                outcomes.push(Outcome::Tag(TagOutcome::RemoveTag(tag_id)));
            }
        }

        (task, outcomes)
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        Task::none()
    }

    #[instrument(skip_all, level = "debug")]
    pub fn view<'a>(
        &self,
        theme: &Theme,
        tags: &'a [Tag],
        tag_groups: &'a [TagGroup],
    ) -> Element<'a, Message, Theme, Renderer> {
        let tag_group = self
            .selected_tag_group_id
            .and_then(|selected_tag_group_id| {
                tag_groups
                    .iter()
                    .find(|tag_group| tag_group.id == selected_tag_group_id)
            });
        let tag_group_tags = tags.iter().filter(|&tag| {
            self.selected_tag_group_id
                .is_some_and(|selected_tag_group_id| tag.tag_group_id == selected_tag_group_id)
        });

        container(column![
            header(theme, tag_groups),
            vertical_separator(),
            row![
                tag_group_pane(theme, tag_groups, &self.new_tag_group_name_input_text),
                horizontal_separator(),
                tag_group_tags_pane(
                    theme,
                    tag_group,
                    tag_group_tags,
                    &self.new_tag_name_input_text
                )
            ]
            .height(Length::Fill)
            .width(Length::Fill),
            vertical_separator(),
            footer(theme)
        ])
        .width(890.0)
        .height(660.0)
        // Offsets inner containers so they don't overlap modal container border.
        .padding(Padding::from(1.0))
        .style(catalog::container::modal)
        .into()
    }
}
