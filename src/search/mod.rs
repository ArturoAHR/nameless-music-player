use rustc_hash::{FxHashMap, FxHashSet};
use thiserror::Error;
use tracing::instrument;

use crate::{
    search::models::{SearchCondition, SearchConditionGroup, SearchConditionGroupOperator},
    tag::index::TrackTagIndex,
    track::models::{Track, TrackId},
};

pub mod filter;
pub mod models;
pub mod validation;

#[derive(Debug, Error, Clone)]
pub enum SearchError {
    #[error("Invalid search conditions")]
    InvalidSearchConditions,
}

#[instrument(skip(tracks), level = "debug", err)]
pub fn search(
    tracks: &FxHashMap<TrackId, Track>,
    track_tag_index: &TrackTagIndex,
    criteria: SearchConditionGroup,
) -> Result<FxHashSet<TrackId>, SearchError> {
    if !criteria.validate() {
        return Err(SearchError::InvalidSearchConditions);
    }

    let mut result = match criteria.operator {
        SearchConditionGroupOperator::And => tracks.keys().copied().collect(),
        SearchConditionGroupOperator::Or => FxHashSet::default(),
    };

    for condition in criteria.conditions {
        let condition_result = match condition {
            SearchCondition::Statement(statement) => statement.filter(tracks, track_tag_index),
            SearchCondition::Group(group) => Some(search(tracks, track_tag_index, group)?),
        };

        let Some(condition_result) = condition_result else {
            continue;
        };

        match criteria.operator {
            SearchConditionGroupOperator::And => {
                result = result.intersection(&condition_result).copied().collect();
            }
            SearchConditionGroupOperator::Or => {
                result = result.union(&condition_result).copied().collect();
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{search::models::SearchConditionStatement, tag::models::TagId};

    use super::*;

    pub fn create_mock_track(track_id: TrackId) -> Track {
        Track {
            id: track_id,
            ..Track::default()
        }
    }

    pub fn generate_tracks() -> FxHashMap<TrackId, Track> {
        (1..=10)
            .map(|track_id| (track_id, create_mock_track(track_id)))
            .collect()
    }

    pub fn generate_track_tag_index() -> TrackTagIndex {
        let tag_tracks: FxHashMap<TagId, Vec<TrackId>> = [
            (1, vec![1, 2, 3, 4, 5]),
            (2, vec![6, 7, 8, 9, 10]),
            (3, vec![1, 3, 5, 7, 9]),
            (4, vec![2, 4, 6, 8, 10]),
            (5, vec![1]),
            (6, vec![10]),
            (7, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        ]
        .into_iter()
        .collect();

        let mut track_tag_index = TrackTagIndex::new(Vec::new());

        for (tag_id, track_ids) in tag_tracks {
            for track_id in track_ids {
                track_tag_index.tag_track(track_id, tag_id);
            }
        }

        track_tag_index
    }

    fn assert_search_result(search_result: FxHashSet<TrackId>, expected_search_result: &[TrackId]) {
        assert_eq!(
            search_result.len(),
            expected_search_result.len(),
            "\n\n    search_result: {search_result:?}\n    expected_search_result: {expected_search_result:?}\n"
        );

        for expected_search_result_track_id in expected_search_result {
            assert!(
                search_result.contains(expected_search_result_track_id),
                "Search result does not contain expected value: {expected_search_result_track_id}\n\n    search_result: {search_result:?}\n    expected_search_result: {expected_search_result:?}\n"
            );
        }
    }

    #[test]
    fn should_search_with_and_operator_and_a_single_condition() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: Some(1) },
            )],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_search_with_or_operator_and_a_single_condition() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::Or,
            conditions: vec![SearchCondition::Statement(
                SearchConditionStatement::HasTag { tag_id: Some(1) },
            )],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_search_with_and_operator_and_a_several_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(2) }),
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(3) }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[7, 9]);
    }

    #[test]
    fn should_search_with_or_operator_and_a_several_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::Or,
            conditions: vec![
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(2) }),
                SearchCondition::Statement(SearchConditionStatement::HasTag { tag_id: Some(3) }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 3, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn should_search_with_and_operator_and_grouping_exclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::And,
                conditions: vec![
                    SearchCondition::Statement(SearchConditionStatement::HasTag {
                        tag_id: Some(2),
                    }),
                    SearchCondition::Statement(SearchConditionStatement::HasTag {
                        tag_id: Some(3),
                    }),
                ],
            })],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[7, 9]);
    }

    #[test]
    fn should_search_with_or_operator_and_grouping_exclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::Or,
            conditions: vec![SearchCondition::Group(SearchConditionGroup {
                operator: SearchConditionGroupOperator::Or,
                conditions: vec![
                    SearchCondition::Statement(SearchConditionStatement::HasTag {
                        tag_id: Some(2),
                    }),
                    SearchCondition::Statement(SearchConditionStatement::HasTag {
                        tag_id: Some(3),
                    }),
                ],
            })],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 3, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn should_search_with_and_operator_and_several_grouping_exclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(1),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(3),
                        }),
                    ],
                }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(7),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(5),
                        }),
                    ],
                }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1]);
    }

    #[test]
    fn should_search_with_and_operator_and_several_grouping_inclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::And,
            conditions: vec![
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::Or,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(1),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(3),
                        }),
                    ],
                }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::Or,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(7),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(5),
                        }),
                    ],
                }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 2, 3, 4, 5, 7, 9]);
    }

    #[test]
    fn should_search_with_or_operator_and_several_grouping_exclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::Or,
            conditions: vec![
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(1),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(3),
                        }),
                    ],
                }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::And,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(7),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(5),
                        }),
                    ],
                }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 3, 5]);
    }

    #[test]
    fn should_search_with_or_operator_and_several_grouping_inclusive_conditions() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let criteria = SearchConditionGroup {
            operator: SearchConditionGroupOperator::Or,
            conditions: vec![
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::Or,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(1),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(3),
                        }),
                    ],
                }),
                SearchCondition::Group(SearchConditionGroup {
                    operator: SearchConditionGroupOperator::Or,
                    conditions: vec![
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(7),
                        }),
                        SearchCondition::Statement(SearchConditionStatement::HasTag {
                            tag_id: Some(5),
                        }),
                    ],
                }),
            ],
        };

        let search_result = search(&tracks, &track_tag_index, criteria).unwrap();

        assert_search_result(search_result, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
