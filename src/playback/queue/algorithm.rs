use async_trait::async_trait;
use itertools::Either;
use rand::seq::{IteratorRandom, SliceRandom};
use rustc_hash::FxHashSet;

use crate::{
    playback::queue::{PlaybackQueue, PlaybackQueueError, PlaybackRepeatMode, QueueTrackId},
    track::models::TrackId,
    traits::Identifiable,
};

#[async_trait]
pub trait PlaybackQueueAlgorithm {
    fn reset(&mut self, playback_queue: &PlaybackQueue);

    async fn generate_next_tracks(
        &mut self,
        playback_queue: PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError>;
    fn take_next_tracks(&mut self, amount: Option<usize>) -> Vec<TrackId>;
    fn get_next_tracks(&self, amount: Option<usize>) -> Vec<TrackId>;
}

#[derive(Default)]
pub struct PlaybackQueueSequentialAlgorithm {
    next_track_ids: Vec<TrackId>,
    cursor: usize,
}

// TODO: Make sequential playing order more intuitive, overriding previous playback_queue contents so it
// is truly sequential when playing previous tracks
impl PlaybackQueueSequentialAlgorithm {
    pub fn new(playback_queue: &PlaybackQueue) -> Self {
        Self {
            next_track_ids: Vec::new(),
            cursor: Self::get_cursor_position(playback_queue),
        }
    }

    pub fn get_cursor_position(playback_queue: &PlaybackQueue) -> usize {
        let mut queue_system_track_ids = playback_queue
            .queue_track_ids
            .iter()
            .enumerate()
            .filter(|&(index, queue_track_id)| {
                matches!(queue_track_id, QueueTrackId::System(_)) && index <= playback_queue.cursor
            })
            .map(|(_, queue_track_id)| *queue_track_id.id())
            .rev();

        loop {
            let Some(queue_system_track_id) = queue_system_track_ids.next() else {
                break 0;
            };

            let Some(position) = playback_queue
                .track_pool
                .iter()
                .position(|&track_id| track_id == queue_system_track_id)
            else {
                continue;
            };

            break position + 1;
        }
    }
}

#[async_trait]
impl PlaybackQueueAlgorithm for PlaybackQueueSequentialAlgorithm {
    fn reset(&mut self, playback_queue: &PlaybackQueue) {
        self.next_track_ids.clear();
        self.cursor = Self::get_cursor_position(playback_queue);
    }

    async fn generate_next_tracks(
        &mut self,
        playback_queue: PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError> {
        let track_pool = if repeat_mode.is_repeating() {
            Either::Left(playback_queue.track_pool.iter().cycle())
        } else {
            Either::Right(playback_queue.track_pool.iter())
        };

        let next_track_ids: Vec<TrackId> =
            track_pool.skip(self.cursor).take(amount).copied().collect();

        self.cursor += next_track_ids.len();

        if repeat_mode.is_repeating() {
            self.cursor %= playback_queue.track_pool.len();
        }

        self.next_track_ids.extend(&next_track_ids);

        Ok(next_track_ids.len())
    }

    fn take_next_tracks(&mut self, amount: Option<usize>) -> Vec<TrackId> {
        let range = if let Some(amount) = amount
            && amount <= self.next_track_ids.len()
        {
            0..amount
        } else {
            0..self.next_track_ids.len()
        };

        self.next_track_ids.drain(range).collect()
    }

    fn get_next_tracks(&self, amount: Option<usize>) -> Vec<TrackId> {
        let range = if let Some(amount) = amount
            && amount <= self.next_track_ids.len()
        {
            0..amount
        } else {
            0..self.next_track_ids.len()
        };

        self.next_track_ids[range].to_vec()
    }
}

#[derive(Debug, Default)]
pub struct PlaybackQueueRandomShuffleAlgorithm {
    next_track_ids: Vec<TrackId>,
}

#[async_trait]
impl PlaybackQueueAlgorithm for PlaybackQueueRandomShuffleAlgorithm {
    fn reset(&mut self, _playback_queue: &PlaybackQueue) {
        self.next_track_ids.clear();
    }

    async fn generate_next_tracks(
        &mut self,
        playback_queue: PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError> {
        let mut rng = rand::rng();

        let mut next_track_ids: Vec<TrackId> = if repeat_mode.is_repeating() {
            let recently_played_tracks: FxHashSet<TrackId> = if let Some(queue_skip) =
                playback_queue
                    .queue_track_ids
                    .len()
                    .checked_sub(playback_queue.track_pool.len() / 2)
            {
                playback_queue
                    .queue_track_ids
                    .iter()
                    .skip(queue_skip)
                    .map(Identifiable::id)
                    .copied()
                    .collect()
            } else {
                playback_queue
                    .queue_track_ids
                    .iter()
                    .map(Identifiable::id)
                    .copied()
                    .collect()
            };

            playback_queue
                .track_pool
                .iter()
                .filter(|&track_id| !recently_played_tracks.contains(track_id))
                .copied()
                .sample(&mut rng, amount)
        } else {
            let already_played_track_ids: FxHashSet<TrackId> = playback_queue
                .queue_track_ids
                .iter()
                .map(Identifiable::id)
                .copied()
                .collect();

            playback_queue
                .track_pool
                .iter()
                .filter(|&track_id| !already_played_track_ids.contains(track_id))
                .sample(&mut rng, amount)
                .into_iter()
                .copied()
                .collect()
        };

        next_track_ids.shuffle(&mut rng);

        self.next_track_ids.extend(&next_track_ids);

        Ok(next_track_ids.len())
    }

    fn take_next_tracks(&mut self, amount: Option<usize>) -> Vec<TrackId> {
        let range = if let Some(amount) = amount
            && amount <= self.next_track_ids.len()
        {
            0..amount
        } else {
            0..self.next_track_ids.len()
        };

        self.next_track_ids.drain(range).collect()
    }

    fn get_next_tracks(&self, amount: Option<usize>) -> Vec<TrackId> {
        let range = if let Some(amount) = amount
            && amount <= self.next_track_ids.len()
        {
            0..amount
        } else {
            0..self.next_track_ids.len()
        };

        self.next_track_ids[range].to_vec()
    }
}
