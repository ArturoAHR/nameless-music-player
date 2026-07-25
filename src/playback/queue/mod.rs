use async_trait::async_trait;
use itertools::Either;
use rand::seq::{IndexedRandom, IteratorRandom};
use thiserror::Error;

use crate::{track::models::TrackId, traits::Identifiable, ui::widgets::table::TableRow};

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
    /// Contains the list of tracks from which the queue can pick from, it assumes the list is non-repeating.
    playing_track_list_ids: Vec<TrackId>,

    repeat_mode: PlaybackRepeatMode,
    order: PlaybackQueueOrder,
    shuffle_algorithm: Box<dyn ShuffleAlgorithm + Send>,

    
}

impl PlaybackQueue {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            order: PlaybackQueueOrder::Sequential {
                playing_track_list_cursor: 0,
            },
            playing_track_list_ids: Vec::new(),
            queue_track_ids: Vec::new(),
            repeat_mode: PlaybackRepeatMode::NoRepeat,
            shuffle_algorithm: Box::new(RandomShuffleAlgorithm::new()),
        }
    }

    pub async fn start_queue(
        &mut self,
        playing_track_list_ids: Vec<TrackId>,
        current_playing_track_id: Option<TrackId>,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError> {
        self.playing_track_list_ids = playing_track_list_ids;
        self.cursor = 0;
        self.queue_track_ids.clear();

        if let Some(current_playing_track_id) = current_playing_track_id {
            self.queue_track_ids
                .push(From::<TrackId>::from(current_playing_track_id));
        }

        self.extend(amount).await
    }

    pub fn get_next(&self) -> Option<TrackId> {
        self.queue_track_ids
            .get(self.cursor + 1)
            .map(|queue_track_id| *queue_track_id.id())
    }

    pub fn get_previous(&self) -> Option<TrackId> {
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
        self.cursor = self.cursor.min(self.queue_track_ids.len() - 1);

        Ok(())
    }

    /// Removes the track id relative to the cursor, which current value determines the first
    /// track displayed in the queue pane table.
    pub fn remove_relative_to_cursor(&mut self, index: usize) -> Result<(), PlaybackQueueError> {
        self.remove(self.cursor + index)
    }

    pub fn cycle_repeat_mode(&mut self) {
        self.repeat_mode = self.repeat_mode.next();
    }

    pub fn cycle_queue_order(&mut self) {
        self.order = match self.order {
            PlaybackQueueOrder::Shuffle => {
                // Tries to find the last played track that is from the current playing track list
                // and return its position, if no such track is found it returns 0.
                let mut queue_system_track_ids = self
                    .queue_track_ids
                    .iter()
                    .enumerate()
                    .filter(|&(index, queue_track_id)| {
                        matches!(queue_track_id, QueueTrackId::System(_)) && index <= self.cursor
                    })
                    .map(|(_, queue_track_id)| *queue_track_id.id())
                    .rev();

                let playing_track_list_cursor = loop {
                    let Some(queue_system_track_id) = queue_system_track_ids.next() else {
                        break 0;
                    };

                    let Some(position) = self
                        .playing_track_list_ids
                        .iter()
                        .position(|&track_id| track_id == queue_system_track_id)
                    else {
                        continue;
                    };

                    break position + 1;
                };

                PlaybackQueueOrder::Sequential {
                    playing_track_list_cursor,
                }
            }
            PlaybackQueueOrder::Sequential {
                playing_track_list_cursor: _,
            } => PlaybackQueueOrder::Shuffle,
        }
    }

    pub fn repeat_mode(&self) -> PlaybackRepeatMode {
        self.repeat_mode
    }

    pub fn order(&self) -> PlaybackQueueOrder {
        self.order
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor.min(self.queue_track_ids.len() - 1);
    }

    pub async fn extend(&mut self, amount: usize) -> Result<usize, PlaybackQueueError> {
        let next_track_ids = match self.order {
            PlaybackQueueOrder::Sequential {
                playing_track_list_cursor,
            } => self.get_next_tracks_ids_sequential(playing_track_list_cursor, amount),
            PlaybackQueueOrder::Shuffle => {
                self.shuffle_algorithm
                    .get_next_queue_track_ids(
                        &self.queue_track_ids,
                        &self.playing_track_list_ids,
                        self.repeat_mode,
                        amount,
                    )
                    .await?
            }
        };

        self.queue_track_ids.extend(&next_track_ids);

        Ok(next_track_ids.len())
    }

    pub fn get_next_tracks_ids_sequential(
        &mut self,
        mut playing_track_list_cursor: usize,
        amount: usize,
    ) -> Vec<QueueTrackId> {
        let playing_track_list_ids = if self.repeat_mode.is_repeating() {
            Either::Left(self.playing_track_list_ids.iter().cycle())
        } else {
            Either::Right(self.playing_track_list_ids.iter())
        };

        let next_track_ids: Vec<QueueTrackId> = playing_track_list_ids
            .skip(playing_track_list_cursor)
            .take(amount)
            .copied()
            .map(From::<TrackId>::from)
            .collect();

        playing_track_list_cursor += next_track_ids.len();

        if self.repeat_mode.is_repeating() {
            playing_track_list_cursor %= self.playing_track_list_ids.len();
        }

        self.order = PlaybackQueueOrder::Sequential {
            playing_track_list_cursor,
        };

        next_track_ids
    }

    // Removes the track ids that go after the current track id that are system queued and extends by
    // the given amount.
    pub async fn regenerate(&mut self, amount: usize) -> Result<usize, PlaybackQueueError> {
        let upcoming_user_queued_track_ids: Vec<QueueTrackId> = self
            .queue_track_ids
            .iter()
            .skip(self.cursor + 1)
            .filter(|queue_track_id| matches!(queue_track_id, QueueTrackId::User(_)))
            .copied()
            .collect();

        self.queue_track_ids.truncate(self.cursor + 1);
        self.queue_track_ids.extend(upcoming_user_queued_track_ids);

        self.extend(amount).await
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
    Sequential { playing_track_list_cursor: usize },
    Shuffle,
}

#[async_trait]
pub trait ShuffleAlgorithm {
    async fn get_next_queue_track_ids(
        &mut self,
        current_queue_track_ids: &[QueueTrackId],
        playing_track_list_ids: &[TrackId],
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<Vec<QueueTrackId>, PlaybackQueueError>;
}

pub struct RandomShuffleAlgorithm {}

impl RandomShuffleAlgorithm {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RandomShuffleAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShuffleAlgorithm for RandomShuffleAlgorithm {
    async fn get_next_queue_track_ids(
        &mut self,
        current_queue_track_ids: &[QueueTrackId],
        playing_track_list_ids: &[TrackId],
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<Vec<QueueTrackId>, PlaybackQueueError> {
        if repeat_mode.is_repeating() {
            return Ok(playing_track_list_ids
                .sample(&mut rand::rng(), amount)
                .copied()
                .map(From::<TrackId>::from)
                .collect());
        }

        Ok(playing_track_list_ids
            .iter()
            .filter(|&track_id| current_queue_track_ids.contains(&From::<TrackId>::from(*track_id)))
            .sample(&mut rand::rng(), amount)
            .into_iter()
            .copied()
            .map(From::<TrackId>::from)
            .collect())
    }
}
