use rustc_hash::{FxHashMap, FxHashSet};
use thiserror::Error;
use tracing::{instrument, warn};

use crate::{
    search::models::{
        SearchCondition, SearchConditionGroup, SearchConditionGroupOperator,
        SearchConditionStatement,
    },
    tag::index::TrackTagIndex,
    track::models::{Track, TrackId},
};

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

impl SearchConditionStatement {
    #[instrument(skip_all, level = "debug")]
    pub fn filter(
        &self,
        tracks: &FxHashMap<TrackId, Track>,
        track_tag_index: &TrackTagIndex,
    ) -> Option<FxHashSet<TrackId>> {
        match self {
            Self::HasTag {
                tag_id: Some(tag_id),
            } => track_tag_index.get_tag_tracks(*tag_id).cloned(),
            Self::DoesNotHaveTag {
                tag_id: Some(tag_id),
            } => {
                let track_ids = tracks.keys();

                let Some(tag_tracks) = track_tag_index.get_tag_tracks(*tag_id) else {
                    return Some(track_ids.copied().collect());
                };

                Some(
                    track_ids
                        .filter(|track_id| !tag_tracks.contains(*track_id))
                        .copied()
                        .collect(),
                )
            }
            Self::HasTag { tag_id: None } | Self::DoesNotHaveTag { tag_id: None } => {
                warn!("Tag search condition statement does not have a tag id.");

                None
            }
        }
    }
}
