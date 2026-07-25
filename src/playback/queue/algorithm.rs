use async_trait::async_trait;
use itertools::Either;
use rand::seq::{IndexedRandom, IteratorRandom};

use crate::{
    playback::queue::{PlaybackQueue, PlaybackQueueError, PlaybackRepeatMode, QueueTrackId},
    track::models::TrackId,
    traits::Identifiable,
};

#[async_trait]
pub trait PlaybackQueueAlgorithm {
    fn initialize(&mut self, playback_queue: &PlaybackQueue, track_pool: Vec<TrackId>);
    fn update_track_pool(&mut self, track_pool: Vec<TrackId>);

    async fn generate_next_tracks(
        &mut self,
        playback_queue: &PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError>;
    fn take_next_tracks(&mut self, amount: Option<usize>) -> Vec<TrackId>;
    fn get_next_tracks(&self, amount: Option<usize>) -> Vec<TrackId>;
}

#[derive(Default)]
pub struct PlaybackQueueSequentialAlgorithm {
    next_track_ids: Vec<TrackId>,
    track_pool: Vec<TrackId>,
    cursor: usize,
}

#[async_trait]
impl PlaybackQueueAlgorithm for PlaybackQueueSequentialAlgorithm {
    fn initialize(&mut self, playback_queue: &PlaybackQueue, track_pool: Vec<TrackId>) {
        self.next_track_ids.clear();
        self.track_pool = track_pool;

        let mut queue_system_track_ids = playback_queue
            .queue_track_ids
            .iter()
            .enumerate()
            .filter(|&(index, queue_track_id)| {
                matches!(queue_track_id, QueueTrackId::System(_)) && index <= playback_queue.cursor
            })
            .map(|(_, queue_track_id)| *queue_track_id.id())
            .rev();

        self.cursor = loop {
            let Some(queue_system_track_id) = queue_system_track_ids.next() else {
                break 0;
            };

            let Some(position) = self
                .track_pool
                .iter()
                .position(|&track_id| track_id == queue_system_track_id)
            else {
                continue;
            };

            break position + 1;
        };
    }

    fn update_track_pool(&mut self, track_pool: Vec<TrackId>) {
        self.track_pool = track_pool;
    }

    async fn generate_next_tracks(
        &mut self,
        _playback_queue: &PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError> {
        let track_pool = if repeat_mode.is_repeating() {
            Either::Left(self.track_pool.iter().cycle())
        } else {
            Either::Right(self.track_pool.iter())
        };

        let next_track_ids: Vec<TrackId> =
            track_pool.skip(self.cursor).take(amount).copied().collect();

        self.cursor += next_track_ids.len();

        if repeat_mode.is_repeating() {
            self.cursor %= self.track_pool.len();
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

pub struct PlaybackQueueRandomShuffleAlgorithm {
    next_track_ids: Vec<TrackId>,
    track_pool: Vec<TrackId>,
}

#[async_trait]
impl PlaybackQueueAlgorithm for PlaybackQueueRandomShuffleAlgorithm {
    fn initialize(&mut self, _playback_queue: &PlaybackQueue, track_pool: Vec<TrackId>) {
        self.next_track_ids.clear();
        self.track_pool = track_pool;
    }

    fn update_track_pool(&mut self, track_pool: Vec<TrackId>) {
        self.track_pool = track_pool;
    }

    async fn generate_next_tracks(
        &mut self,
        playback_queue: &PlaybackQueue,
        repeat_mode: PlaybackRepeatMode,
        amount: usize,
    ) -> Result<usize, PlaybackQueueError> {
        let next_track_ids: Vec<TrackId> = if repeat_mode.is_repeating() {
            self.track_pool
                .sample(&mut rand::rng(), amount)
                .copied()
                .collect()
        } else {
            self.track_pool
                .iter()
                .filter(|&track_id| {
                    playback_queue
                        .queue_track_ids
                        .contains(&From::<TrackId>::from(*track_id))
                })
                .sample(&mut rand::rng(), amount)
                .into_iter()
                .copied()
                .collect()
        };

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
