use crate::{track::models::TrackId, traits::Identifiable, ui::widgets::table::TableRow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackQueueEntry {
    pub id: u64,
    pub track_id: TrackId,
    pub source: PlaybackQueueEntrySource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackQueueEntrySource {
    System,
    User,
}

impl PlaybackQueueEntry {
    pub(super) fn system(id: PlaybackQueueEntryId, track_id: TrackId) -> Self {
        Self {
            id,
            track_id,
            source: PlaybackQueueEntrySource::System,
        }
    }

    pub(super) fn user(id: PlaybackQueueEntryId, track_id: TrackId) -> Self {
        Self {
            id,
            track_id,
            source: PlaybackQueueEntrySource::User,
        }
    }
}

impl Identifiable for PlaybackQueueEntry {
    type Identifier = u64;

    fn id(&self) -> &Self::Identifier {
        &self.id
    }
}

pub type PlaybackQueueEntryId = <PlaybackQueueEntry as Identifiable>::Identifier;

impl TableRow for PlaybackQueueEntry {
    fn header_row_id() -> Self::Identifier {
        u64::MAX
    }
}
