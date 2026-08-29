use std::sync::Arc;

use iced::{Element, Renderer, Task, widget::column};
use itertools::Itertools;
use tracing::warn;

use crate::{
    event::Event,
    outcome::{ModalOutcome, TrackListOutcome},
    search::models::{
        SearchCondition, SearchConditionGroup, SearchConditionGroupOperator,
        SearchConditionStatement, SearchConditionStatementKind,
    },
    tag::models::{Tag, TagGroup, TagId},
    ui::{
        modals::advanced_search::widgets::{body, footer, header},
        theme::Theme,
        widgets::{modal::modal_container, separator::vertical_separator},
    },
};

pub mod handler;
pub mod widgets;

#[derive(Debug)]
pub struct AdvancedSearchModal {
    criteria: SearchConditionGroup,
    tag_options: Vec<PickListOption<TagId>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Close,
    Search,
    SelectConditionStatementTag(TagId, Arc<[usize]>),
    SelectConditionStatement(SearchConditionStatementKind, Arc<[usize]>),
    SelectGroupOperator(SearchConditionGroupOperator, Arc<[usize]>),
    AddCondition(Arc<[usize]>),
    AddSubgroup(Arc<[usize]>),
    RemoveStatement(Arc<[usize]>),
    RemoveGroup(Arc<[usize]>),
}

pub enum Outcome {
    Modal(ModalOutcome),
    TrackList(TrackListOutcome),
}

// TODO: Move this struct to a more generic location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickListOption<T: PartialEq> {
    label: String,
    value: T,
}

impl<T> std::fmt::Display for PickListOption<T>
where
    T: PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

impl AdvancedSearchModal {
    pub fn new(criteria: SearchConditionGroup, tags: &[Tag], tag_groups: &[TagGroup]) -> Self {
        let tag_options = tag_groups
            .iter()
            .flat_map(|tag_group| {
                tags.iter()
                    .filter_map(|tag| {
                        if tag.tag_group_id == tag_group.id {
                            Some(PickListOption {
                                label: format!("{} - {}", tag_group.name, tag.name),
                                value: tag.id,
                            })
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .collect();

        Self {
            criteria,
            tag_options,
        }
    }

    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Close => {
                outcomes.push(Outcome::Modal(ModalOutcome::CloseModal));
            }
            Message::Search => {
                outcomes.push(Outcome::TrackList(TrackListOutcome::AdvancedSearch(
                    self.criteria.clone(),
                )));
            }
            Message::AddCondition(index_path) if index_path.is_empty() => {
                self.criteria.conditions.push(SearchCondition::Statement(
                    SearchConditionStatement::HasTag { tag_id: None },
                ));
            }
            Message::AddSubgroup(index_path) if index_path.is_empty() => {
                self.criteria
                    .conditions
                    .push(SearchCondition::Group(SearchConditionGroup::default()));
            }

            Message::AddCondition(index_path)
                if let Some(SearchCondition::Group(search_condition_group)) =
                    self.criteria.get_mut(&index_path) =>
            {
                search_condition_group
                    .conditions
                    .push(SearchCondition::Statement(
                        SearchConditionStatement::HasTag { tag_id: None },
                    ));
            }
            Message::AddSubgroup(index_path)
                if let Some(SearchCondition::Group(search_condition_group)) =
                    self.criteria.get_mut(&index_path) =>
            {
                search_condition_group
                    .conditions
                    .push(SearchCondition::Group(SearchConditionGroup::default()));
            }
            Message::RemoveGroup(index_path) => {
                let removed_group = self.criteria.remove(&index_path);

                if removed_group.is_none() {
                    warn!("Could not remove group at index path: {index_path:?}");
                }
            }
            Message::RemoveStatement(index_path) => {
                let removed_statement = self.criteria.remove(&index_path);

                if removed_statement.is_none() {
                    warn!("Could not remove statement at index path: {index_path:?}");
                }
            }
            Message::SelectConditionStatement(search_condition_statement_kind, index_path)
                if let Some(SearchCondition::Statement(search_condition_statement)) =
                    self.criteria.get_mut(&index_path) =>
            {
                *search_condition_statement = search_condition_statement_kind.statement();
            }
            Message::SelectConditionStatementTag(selected_tag_id, index_path)
                if let Some(SearchCondition::Statement(search_condition_statement)) =
                    self.criteria.get_mut(&index_path) =>
            {
                match search_condition_statement {
                    SearchConditionStatement::HasTag { tag_id }
                    | SearchConditionStatement::DoesNotHaveTag { tag_id } => {
                        *tag_id = Some(selected_tag_id);
                    }
                }
            }
            Message::SelectGroupOperator(search_condition_group_operator, index_path)
                if index_path.is_empty() =>
            {
                self.criteria.operator = search_condition_group_operator;
            }
            Message::SelectGroupOperator(search_condition_group_operator, index_path)
                if let Some(SearchCondition::Group(search_condition_group)) =
                    self.criteria.get_mut(&index_path) =>
            {
                search_condition_group.operator = search_condition_group_operator;
            }
            _ => {
                warn!("Unsupported operation: {message:?}");
            }
        }

        (task, outcomes)
    }

    pub fn on_event(&mut self, _event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self, theme: &Theme) -> Element<'_, Message, Theme, Renderer> {
        modal_container(column![
            header(theme),
            vertical_separator(),
            body(theme, &self.criteria, &self.tag_options),
            vertical_separator(),
            footer(theme)
        ])
        .width(500.0)
        .height(600.0)
        .into()
    }
}
