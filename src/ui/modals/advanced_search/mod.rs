use std::sync::Arc;

use iced::{Element, Renderer, Task, widget::column};
use itertools::Itertools;

use crate::{
    event::Event,
    outcome::ModalOutcome,
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
}

// TODO: Move this struct to a more generic location
#[derive(Debug, Clone, PartialEq)]
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
        (Task::none(), Vec::new())
    }

    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
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
