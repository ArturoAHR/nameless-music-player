use rustc_hash::{FxHashMap, FxHashSet};
use tracing::{instrument, warn};

use crate::{
    search::models::SearchConditionStatement,
    tag::index::TrackTagIndex,
    track::models::{Track, TrackId},
};

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
            } => track_tag_index
                .get_tag_tracks(*tag_id)
                .cloned()
                .or_else(|| Some(FxHashSet::default())),

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

#[cfg(test)]
mod tests {
    use crate::search::tests::{generate_track_tag_index, generate_tracks};

    use super::*;

    fn assert_filter_result(filter_result: FxHashSet<TrackId>, expected_filter_result: &[TrackId]) {
        assert_eq!(
            filter_result.len(),
            expected_filter_result.len(),
            "\n\n    filter_result: {filter_result:?}\n    expected_filter_result: {expected_filter_result:?}\n"
        );

        for expected_filter_result_track_id in expected_filter_result {
            assert!(
                filter_result.contains(expected_filter_result_track_id),
                "Filter result does not contain expected value: {expected_filter_result_track_id}\n\n    filter_result: {filter_result:?}\n    expected_filter_result: {expected_filter_result:?}\n"
            );
        }
    }

    #[test]
    fn should_filter_has_tag_statement() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::HasTag { tag_id: Some(1) };

        let filter_result = statement.filter(&tracks, &track_tag_index).unwrap();

        assert_filter_result(filter_result, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_filter_has_tag_statement_with_non_existent_tag() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::HasTag { tag_id: Some(0) };

        let filter_result = statement.filter(&tracks, &track_tag_index).unwrap();

        assert_filter_result(filter_result, &[]);
    }

    #[test]
    fn should_filter_does_not_have_tag_statement() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::DoesNotHaveTag { tag_id: Some(1) };

        let filter_result = statement.filter(&tracks, &track_tag_index).unwrap();

        assert_filter_result(filter_result, &[6, 7, 8, 9, 10]);
    }

    #[test]
    fn should_filter_does_not_have_tag_statement_with_non_existent_tag() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::DoesNotHaveTag { tag_id: Some(0) };

        let filter_result = statement.filter(&tracks, &track_tag_index).unwrap();

        assert_filter_result(filter_result, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn should_not_filter_has_tag_statement_missing_tag_id() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::HasTag { tag_id: None };

        assert!(statement.filter(&tracks, &track_tag_index).is_none());
    }

    #[test]
    fn should_not_filter_does_not_have_tag_statement_missing_tag_id() {
        let tracks = generate_tracks();
        let track_tag_index = generate_track_tag_index();

        let statement = SearchConditionStatement::DoesNotHaveTag { tag_id: None };

        assert!(statement.filter(&tracks, &track_tag_index).is_none());
    }
}
