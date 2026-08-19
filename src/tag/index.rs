use std::collections::hash_map::Entry;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    tag::models::{TagId, TrackTag},
    track::models::TrackId,
};

#[derive(Debug, Default)]
pub struct TrackTagIndex {
    tags_by_track: FxHashMap<TrackId, Vec<TagId>>,
    tracks_by_tag: FxHashMap<TagId, FxHashSet<TrackId>>,
}

impl TrackTagIndex {
    pub fn new(track_tag_rows: Vec<TrackTag>) -> Self {
        let mut tracks_by_tag: FxHashMap<TagId, FxHashSet<TrackId>> = FxHashMap::default();
        let mut tags_by_track: FxHashMap<TrackId, Vec<TagId>> = FxHashMap::default();

        for track_tag_row in track_tag_rows {
            tracks_by_tag
                .entry(track_tag_row.tag_id)
                .or_default()
                .insert(track_tag_row.track_id);

            tags_by_track
                .entry(track_tag_row.track_id)
                .or_default()
                .push(track_tag_row.tag_id);
        }

        Self {
            tags_by_track,
            tracks_by_tag,
        }
    }

    pub fn get_track_tags(&self, track_id: TrackId) -> Option<&Vec<TagId>> {
        self.tags_by_track.get(&track_id)
    }

    pub fn get_tag_tracks(&self, tag_id: TagId) -> Option<&FxHashSet<TrackId>> {
        self.tracks_by_tag.get(&tag_id)
    }

    pub fn tag_track(&mut self, track_id: TrackId, tag_id: TagId) {
        self.tracks_by_tag
            .entry(tag_id)
            .or_default()
            .insert(track_id);

        self.tags_by_track.entry(track_id).or_default().push(tag_id);
    }

    pub fn untag_track(&mut self, track_id: TrackId, tag_id: TagId) {
        let track_tags = self.tags_by_track.entry(track_id).and_modify(|track_tags| {
            if let Some(track_tag_index) = track_tags
                .iter()
                .position(|&track_tag_id| track_tag_id == tag_id)
            {
                track_tags.remove(track_tag_index);
            }
        });

        if let Entry::Occupied(track_tags) = track_tags
            && track_tags.get().is_empty()
        {
            self.tags_by_track.remove(&track_id);
        }

        let tag_tracks = self.tracks_by_tag.entry(tag_id).and_modify(|tag_tracks| {
            tag_tracks.remove(&track_id);
        });

        if let Entry::Occupied(tag_tracks) = tag_tracks
            && tag_tracks.get().is_empty()
        {
            self.tracks_by_tag.remove(&tag_id);
        }
    }

    pub fn toggle_track_tag(&mut self, track_id: TrackId, tag_id: TagId) {
        if self.exists(track_id, tag_id) {
            self.untag_track(track_id, tag_id);
        } else {
            self.tag_track(track_id, tag_id);
        }
    }

    pub fn exists(&self, track_id: TrackId, tag_id: TagId) -> bool {
        self.tracks_by_tag
            .get(&tag_id)
            .is_some_and(|tag_tracks| tag_tracks.contains(&track_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_track_tag_exists(index: &TrackTagIndex, track_id: TrackId, tag_id: TagId) {
        assert!(
            index.exists(track_id, tag_id),
            "Index didn't find inserted track tag."
        );

        assert!(
            index
                .get_tag_tracks(tag_id)
                .expect("index didn't find tag tracks")
                .contains(&track_id),
            "Index did not insert track id to tracks_by_id index."
        );

        assert!(
            index
                .get_track_tags(track_id)
                .expect("index didn't find track tags")
                .contains(&tag_id),
            "Index did not insert tag id to tags_by_track index."
        );
    }

    fn assert_no_track_tag_exists(index: &TrackTagIndex, track_id: TrackId, tag_id: TagId) {
        assert!(
            !index.exists(track_id, tag_id),
            "Index found inserted the track tag when it shouldn't be there."
        );

        assert!(
            index
                .get_tag_tracks(tag_id)
                .is_none_or(|tag_tracks| !tag_tracks.contains(&track_id)),
            "Index found inserted the track tag in the tracks_by_id index when it shouldn't be there."
        );

        assert!(
            index
                .get_track_tags(track_id)
                .is_none_or(|track_tags| !track_tags.contains(&tag_id)),
            "Index found inserted the track tag in the tags_by_id index when it shouldn't be there."
        );
    }

    #[test]
    fn should_get_existing_track_tag() {
        let mut index = TrackTagIndex::new(vec![]);

        let track_id = 1;
        let tag_id = 2;

        index
            .tracks_by_tag
            .entry(tag_id)
            .or_default()
            .insert(track_id);

        index
            .tags_by_track
            .entry(track_id)
            .or_default()
            .push(tag_id);

        assert_track_tag_exists(&index, track_id, tag_id);
    }

    #[test]
    fn should_not_get_existing_track_tag() {
        let index = TrackTagIndex::new(vec![]);

        let track_id = 1;
        let tag_id = 2;

        assert_no_track_tag_exists(&index, track_id, tag_id);
    }

    #[test]
    fn should_initialize_with_track_tags() {
        let mut track_tag_rows: Vec<TrackTag> = Vec::new();

        for track_id in 0..10 {
            for tag_id in 0..track_id {
                track_tag_rows.push(TrackTag {
                    id: track_tag_rows.len() as i64,
                    tag_id,
                    track_id,
                    created_at: 0,
                });
            }
        }

        let index = TrackTagIndex::new(track_tag_rows);

        for track_id in 0..10 {
            for tag_id in 0..track_id {
                assert_track_tag_exists(&index, track_id, tag_id);
            }
        }

        assert_no_track_tag_exists(&index, 1, 10);
    }

    #[test]
    fn should_toggle_track_tags() {
        let mut index = TrackTagIndex::new(vec![]);

        let track_id = 1;
        let tag_id = 2;

        index.toggle_track_tag(track_id, tag_id);

        assert_track_tag_exists(&index, track_id, tag_id);

        index.toggle_track_tag(track_id, tag_id);

        assert_no_track_tag_exists(&index, track_id, tag_id);
    }
}
