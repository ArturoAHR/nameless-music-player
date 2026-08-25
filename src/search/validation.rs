use crate::search::models::{SearchCondition, SearchConditionGroup, SearchConditionStatement};

impl SearchConditionGroup {
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Validates that the group is valid to perform a search.
    pub fn validate(&self) -> bool {
        self.conditions.iter().all(SearchCondition::validate) && !self.is_empty()
    }
}

impl SearchCondition {
    /// Validates that the condition is valid to perform a search.
    pub fn validate(&self) -> bool {
        match self {
            Self::Statement(statement) => statement.validate(),
            Self::Group(group) => group.validate(),
        }
    }
}

impl SearchConditionStatement {
    /// Validates that the statement is valid to perform a search.
    pub fn validate(&self) -> bool {
        match self {
            Self::HasTag { tag_id } | Self::DoesNotHaveTag { tag_id } => tag_id.is_some(),
        }
    }
}
