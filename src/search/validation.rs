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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_valid_search_condition_group_with_one_condition() {
        let mut search_condition_group = SearchConditionGroup::default();

        search_condition_group
            .conditions
            .push(SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: Some(1) },
            ));

        assert!(search_condition_group.validate());
    }

    #[test]
    fn should_validate_valid_search_condition_group_with_multiple_conditions() {
        let mut search_condition_group = SearchConditionGroup::default();

        search_condition_group.conditions.extend([
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(1) }),
            SearchCondition::Statement(SearchConditionStatement::DoesNotHaveTag {
                tag_id: Some(3),
            }),
        ]);

        assert!(search_condition_group.validate());
    }

    #[test]
    fn should_validate_valid_search_condition_group_with_nested_conditions() {
        let mut search_condition_group = SearchConditionGroup::default();

        let mut nested_search_condition_group = SearchConditionGroup::default();

        nested_search_condition_group.conditions.extend([
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(1) }),
            SearchCondition::Statement(SearchConditionStatement::DoesNotHaveTag {
                tag_id: Some(3),
            }),
        ]);

        search_condition_group
            .conditions
            .extend([SearchCondition::Group(nested_search_condition_group)]);

        assert!(search_condition_group.validate());
    }

    #[test]
    fn should_validate_invalid_search_condition_group_with_no_conditions() {
        let search_condition_group = SearchConditionGroup::default();

        assert!(!search_condition_group.validate());
    }

    #[test]
    fn should_validate_invalid_search_condition_group_with_one_invalid_condition() {
        let mut search_condition_group = SearchConditionGroup::default();

        search_condition_group
            .conditions
            .push(SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: None },
            ));

        assert!(!search_condition_group.validate());
    }

    #[test]
    fn should_validate_invalid_search_condition_group_with_multiple_conditions() {
        let mut search_condition_group = SearchConditionGroup::default();

        search_condition_group.conditions.extend([
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(1) }),
            SearchCondition::Statement(SearchConditionStatement::DoesNotHaveTag { tag_id: None }),
        ]);

        assert!(!search_condition_group.validate());
    }

    #[test]
    fn should_validate_invalid_search_condition_group_with_nested_conditions() {
        let mut search_condition_group = SearchConditionGroup::default();

        let mut nested_search_condition_group = SearchConditionGroup::default();

        nested_search_condition_group.conditions.extend([
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(1) }),
            SearchCondition::Statement(SearchConditionStatement::DoesNotHaveTag { tag_id: None }),
        ]);

        search_condition_group
            .conditions
            .extend([SearchCondition::Group(nested_search_condition_group)]);

        assert!(!search_condition_group.validate());
    }

    #[test]
    fn should_validate_valid_has_tag_search_condition_statement() {
        let search_condition_statement = SearchConditionStatement::HasTag { tag_id: Some(1) };

        assert!(search_condition_statement.validate());
    }

    #[test]
    fn should_validate_invalid_has_tag_search_condition_statement() {
        let search_condition_statement = SearchConditionStatement::HasTag { tag_id: None };

        assert!(!search_condition_statement.validate());
    }

    #[test]
    fn should_validate_valid_does_not_have_tag_search_condition_statement() {
        let search_condition_statement =
            SearchConditionStatement::DoesNotHaveTag { tag_id: Some(1) };

        assert!(search_condition_statement.validate());
    }

    #[test]
    fn should_validate_invalid_does_not_have_tag_search_condition_statement() {
        let search_condition_statement = SearchConditionStatement::DoesNotHaveTag { tag_id: None };

        assert!(!search_condition_statement.validate());
    }
}
