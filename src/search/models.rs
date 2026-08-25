use crate::tag::models::TagId;

#[derive(Debug, Default)]
pub struct SearchConditionGroup {
    pub operator: SearchConditionGroupOperator,
    pub conditions: Vec<SearchCondition>,
}

#[derive(Debug, Default)]
pub enum SearchConditionGroupOperator {
    #[default]
    And,
    Or,
}

#[derive(Debug)]
pub enum SearchCondition {
    Statement(SearchConditionStatement),
    Group(SearchConditionGroup),
}

#[derive(Debug)]
pub enum SearchConditionStatement {
    HasTag { tag_id: Option<TagId> },
    DoesNotHaveTag { tag_id: Option<TagId> },
}

#[derive(Debug, Default)]
pub enum SearchConditionStatementKind {
    #[default]
    HasTag,
    DoesNotHaveTag,
}

impl SearchConditionStatementKind {
    pub fn statement(&self) -> SearchConditionStatement {
        match self {
            Self::HasTag => SearchConditionStatement::HasTag { tag_id: None },
            Self::DoesNotHaveTag => SearchConditionStatement::DoesNotHaveTag { tag_id: None },
        }
    }
}
