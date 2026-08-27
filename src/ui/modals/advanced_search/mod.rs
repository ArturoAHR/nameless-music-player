use iced::{Element, Renderer, Task, widget::column};

use crate::{
    event::Event,
    outcome::ModalOutcome,
    search::models::{
        SearchCondition, SearchConditionGroup, SearchConditionGroupOperator,
        SearchConditionStatement,
    },
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Close,
    Search,
}

pub enum Outcome {
    Modal(ModalOutcome),
}

impl AdvancedSearchModal {
    pub fn new(criteria: SearchConditionGroup) -> Self {
        Self { criteria }
    }

    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        (Task::none(), Vec::new())
    }

    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(&self, theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
        modal_container(column![
            header(theme),
            vertical_separator(),
            body(theme, &self.criteria),
            vertical_separator(),
            footer(theme)
        ])
        .width(500.0)
        .height(600.0)
        .into()
    }
}

impl Default for AdvancedSearchModal {
    fn default() -> Self {
        Self {
            criteria: SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![SearchCondition::Statement(
                    SearchConditionStatement::HasTag { tag_id: None },
                )],
            },
        }
    }
}
