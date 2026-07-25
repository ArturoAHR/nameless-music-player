use thiserror::Error;

use crate::{track::models::TrackId, traits::Identifiable, ui::widgets::table::TableRow};

pub mod algorithm;

#[derive(Debug, Error)]
pub enum PlaybackQueueError {
    #[error("Attempted to remove queued track beyond queue length")]
    InvalidQueueRemovePosition,
}

pub struct PlaybackQueue {
    /// Current position in the queue.
    cursor: usize,

    /// Contains the ids of the tracks that are in the queue.
    queue_track_ids: Vec<QueueTrackId>,
}

impl PlaybackQueue {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            queue_track_ids: Vec::new(),
        }
    }

    pub fn start(&mut self, current_playing_track_id: Option<TrackId>) {
        self.cursor = 0;
        self.queue_track_ids.clear();

        if let Some(current_playing_track_id) = current_playing_track_id {
            self.queue_track_ids
                .push(From::<TrackId>::from(current_playing_track_id));
        }
    }

    pub fn get_next(&self) -> Option<TrackId> {
        self.queue_track_ids
            .get(self.cursor + 1)
            .map(|queue_track_id| *queue_track_id.id())
    }

    pub fn get_previous(&self) -> Option<TrackId> {
        if self.cursor == 0 {
            return None;
        }

        self.queue_track_ids
            .get(self.cursor - 1)
            .map(|queue_track_id| *queue_track_id.id())
    }

    pub fn go_to_next(&mut self) -> Option<TrackId> {
        let next_track_id = self
            .queue_track_ids
            .get(self.cursor + 1)
            .map(|queue_track_id| *queue_track_id.id());

        if next_track_id.is_some() {
            self.cursor += 1;
        }

        next_track_id
    }

    pub fn go_to_previous(&mut self) -> Option<TrackId> {
        if self.cursor == 0 {
            return None;
        }

        let previous_track_id = self
            .queue_track_ids
            .get(self.cursor - 1)
            .map(|queue_track_id| *queue_track_id.id());

        if previous_track_id.is_some() {
            self.cursor -= 1;
        }

        previous_track_id
    }

    /// Inserts a track id in the queue at the given index, if the index is out of bounds it
    /// adds the track id at the end.
    pub fn insert(&mut self, index: usize, track_id: TrackId) {
        let insert_index = index.min(self.queue_track_ids.len());

        self.queue_track_ids
            .insert(insert_index, QueueTrackId::User(track_id));
    }

    /// Inserts a track id at the next position in the queue.
    pub fn insert_next(&mut self, track_id: TrackId) {
        let insert_index = (self.cursor + 1).min(self.queue_track_ids.len());

        self.queue_track_ids
            .insert(insert_index, QueueTrackId::User(track_id));
    }

    /// Removes a track id from the queue at the given index, bear in mind this index
    /// can differ from the displayed queue pane table row number, to perform removal relative
    /// to the displayed tracks in the queue pane, use `remove_relative_to_cursor`
    ///
    /// # Errors
    /// This method will fail if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Result<(), PlaybackQueueError> {
        if index >= self.queue_track_ids.len() {
            return Err(PlaybackQueueError::InvalidQueueRemovePosition);
        }

        self.queue_track_ids.remove(index);

        if index < self.cursor {
            self.cursor -= 1;
        }

        Ok(())
    }

    /// Removes the track id relative to the cursor, which current value determines the first
    /// track displayed in the queue pane table.
    pub fn remove_relative_to_cursor(&mut self, index: usize) -> Result<(), PlaybackQueueError> {
        self.remove(self.cursor + index)
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        if self.queue_track_ids.is_empty() {
            self.cursor = 0;
        }

        self.cursor = cursor.min(self.queue_track_ids.len() - 1);
    }

    /// Extends the queue with system queued tracks
    pub fn extend(&mut self, next_track_ids: Vec<TrackId>) {
        let next_track_ids: Vec<QueueTrackId> = next_track_ids
            .into_iter()
            .map(From::<TrackId>::from)
            .collect();

        self.queue_track_ids.extend(next_track_ids);
    }

    /// Removes all the upcoming tracks
    pub fn truncate(&mut self) {
        self.queue_track_ids.truncate(self.cursor + 1);
    }

    pub fn get_queue_entries(&self, amount: usize) -> Vec<PlaybackQueueEntry> {
        self.queue_track_ids
            .iter()
            .skip(self.cursor)
            .take(amount)
            .copied()
            .enumerate()
            .map(|(index, queue_track_id)| {
                PlaybackQueueEntry::from_queue_track_id(index, queue_track_id)
            })
            .collect()
    }
}

impl Default for PlaybackQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueTrackId {
    System(TrackId),
    User(TrackId),
}

impl From<TrackId> for QueueTrackId {
    fn from(track_id: TrackId) -> Self {
        Self::System(track_id)
    }
}

impl Identifiable for QueueTrackId {
    type Identifier = TrackId;

    fn id(&self) -> &Self::Identifier {
        match self {
            Self::System(track_id) | Self::User(track_id) => track_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaybackQueueEntry {
    System(usize, TrackId),
    User(usize, TrackId),
}

impl PlaybackQueueEntry {
    pub fn from_queue_track_id(index: usize, queue_track_id: QueueTrackId) -> Self {
        match queue_track_id {
            QueueTrackId::System(track_id) => Self::System(index, track_id),
            QueueTrackId::User(track_id) => Self::User(index, track_id),
        }
    }
}

impl Identifiable for PlaybackQueueEntry {
    type Identifier = Self;

    fn id(&self) -> &Self::Identifier {
        self
    }
}

impl TableRow for PlaybackQueueEntry {
    fn header_row_id() -> Self::Identifier {
        Self::System(0, -1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackRepeatMode {
    NoRepeat,
    Repeat,
    RepeatOne,
}

impl PlaybackRepeatMode {
    pub fn is_repeating(&self) -> bool {
        matches!(self, Self::RepeatOne | Self::Repeat)
    }

    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            Self::NoRepeat => Self::Repeat,
            Self::Repeat => Self::RepeatOne,
            Self::RepeatOne => Self::NoRepeat,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackQueueOrder {
    Sequential,
    Shuffle,
}

impl PlaybackQueueOrder {
    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            Self::Sequential => Self::Shuffle,
            Self::Shuffle => Self::Sequential,
        }
    }
}
