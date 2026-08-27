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

impl SearchConditionStatement {
    pub fn kind(&self) -> SearchConditionStatementKind {
        match self {
            Self::HasTag { tag_id: _ } => SearchConditionStatementKind::HasTag,
            Self::DoesNotHaveTag { tag_id: _ } => SearchConditionStatementKind::DoesNotHaveTag,
        }
    }
}

impl SearchConditionGroup {
    pub fn get(&self, index_path: &[usize]) -> Option<&SearchCondition> {
        let [index, index_path @ ..] = index_path else {
            return None;
        };

        let condition = self.conditions.get(*index)?;
        match condition {
            SearchCondition::Statement(_) | SearchCondition::Group(_) if index_path.is_empty() => {
                Some(condition)
            }
            SearchCondition::Group(group) => group.get(index_path),
            SearchCondition::Statement(_) => None,
        }
    }

    pub fn get_mut(&mut self, index_path: &[usize]) -> Option<&mut SearchCondition> {
        let [index, index_path @ ..] = index_path else {
            return None;
        };

        let condition = self.conditions.get_mut(*index)?;
        match condition {
            SearchCondition::Statement(_) | SearchCondition::Group(_) if index_path.is_empty() => {
                Some(condition)
            }
            SearchCondition::Group(group) => group.get_mut(index_path),
            SearchCondition::Statement(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_matches;

    use super::*;

    #[test]
    fn should_get_statement() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: None },
            )],
        };

        assert_matches!(
            search_condition_group.get(&[0]).unwrap(),
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None })
        );
    }

    #[test]
    fn should_get_group() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![],
            })],
        };

        assert_matches!(
            search_condition_group.get(&[0]).unwrap(),
            SearchCondition::Group(SearchConditionGroup { operator: SearchConditionGroupOperator::And, conditions }) if conditions.is_empty()
        );
    }

    #[test]
    fn should_get_nested_statement() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![SearchCondition::Statement(
                    SearchConditionStatement::HasTag { tag_id: None },
                )],
            })],
        };

        assert_matches!(
            search_condition_group.get(&[0, 0]).unwrap(),
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None })
        );
    }

    #[test]
    fn should_get_nested_group() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        assert_matches!(
            search_condition_group.get(&[1, 0]).unwrap(),
            SearchCondition::Group(SearchConditionGroup { operator: SearchConditionGroupOperator::And, conditions })
                if conditions.len() == 1
        );
    }

    #[test]
    fn should_not_get_non_existent_value() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        assert_matches!(search_condition_group.get(&[2]), None);
    }

    #[test]
    fn should_not_get_non_existent_nested_value() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        assert_matches!(search_condition_group.get(&[0, 0]), None);
    }

    #[test]
    fn should_not_get_value_with_empty_index_path() {
        let search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: None },
            )],
        };

        assert_matches!(search_condition_group.get(&[]), None);
    }

    #[test]
    fn should_get_mutable_statement() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: None },
            )],
        };

        let statement = search_condition_group.get_mut(&[0]).unwrap();

        assert_matches!(
            statement,
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None })
        );

        let SearchCondition::Statement(statement) = statement else {
            panic!("Expected statement, got {statement:?} instead")
        };

        match statement {
            SearchConditionStatement::HasTag { tag_id }
            | SearchConditionStatement::DoesNotHaveTag { tag_id } => {
                *tag_id = Some(6);
            }
        }
    }

    #[test]
    fn should_get_mutable_group() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![],
            })],
        };

        let group = search_condition_group.get_mut(&[0]).unwrap();

        assert_matches!(
            group,
            SearchCondition::Group(SearchConditionGroup { operator: SearchConditionGroupOperator::And, conditions }) if conditions.is_empty()
        );

        let SearchCondition::Group(group) = group else {
            panic!("Expected group, got {group:?} instead");
        };

        group.conditions.push(SearchCondition::Statement(
            SearchConditionStatement::HasTag { tag_id: None },
        ));
    }

    #[test]
    fn should_get_mutable_nested_statement() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![SearchCondition::Statement(
                    SearchConditionStatement::HasTag { tag_id: None },
                )],
            })],
        };

        let nested_statement = search_condition_group.get_mut(&[0, 0]).unwrap();

        assert_matches!(
            nested_statement,
            SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None })
        );

        let SearchCondition::Statement(nested_statement) = nested_statement else {
            panic!("Expected nested statement, got {nested_statement:?} instead")
        };

        match nested_statement {
            SearchConditionStatement::HasTag { tag_id }
            | SearchConditionStatement::DoesNotHaveTag { tag_id } => {
                *tag_id = Some(6);
            }
        }
    }

    #[test]
    fn should_get_mutable_nested_group() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        let nested_group = search_condition_group.get_mut(&[1, 0]).unwrap();

        assert_matches!(
            nested_group,
            SearchCondition::Group(SearchConditionGroup { operator: SearchConditionGroupOperator::And, conditions })
                if conditions.len() == 1
        );

        let SearchCondition::Group(nested_group) = nested_group else {
            panic!("Expected nested group: got {nested_group:?} instead");
        };

        nested_group.conditions.push(SearchCondition::Statement(
            SearchConditionStatement::HasTag { tag_id: None },
        ));
    }

    #[test]
    fn should_not_get_mutable_non_existent_value() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        assert_matches!(search_condition_group.get_mut(&[2]), None);
    }

    #[test]
    fn should_not_get_mutable_non_existent_nested_value() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: None }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![SearchCondition::Group(SearchConditionGroup {
                        operator: SearchConditionGroupOperator::And,
                        conditions: vec![SearchCondition::Statement(
                            SearchConditionStatement::HasTag { tag_id: None },
                        )],
                    })],
                }),
            ],
        };

        assert_matches!(search_condition_group.get_mut(&[0, 0]), None);
    }

    #[test]
    fn should_not_get_mutable_value_with_empty_index_path() {
        let mut search_condition_group = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: None },
            )],
        };

        assert_matches!(search_condition_group.get_mut(&[]), None);
    }
}
