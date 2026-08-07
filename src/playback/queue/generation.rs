use std::collections::VecDeque;

use itertools::Either;
use rand::seq::{IteratorRandom, SliceRandom};
use rustc_hash::FxHashSet;

use crate::{
    playback::queue::{
        PlaybackQueue, PlaybackQueueOrder,
        entry::{PlaybackQueueEntry, PlaybackQueueEntrySource},
    },
    track::models::TrackId,
};

pub enum GenerationDirection {
    Next,
    Previous,
}

impl PlaybackQueue {
    /// Gets the track pool position for sequential generation depending on the direction we are generating:
    ///
    /// - If we are generating the next tracks, we get the last track pool system entry in the queue.
    /// - If we are generating the previous tracks, we get the first track pool system entry in the queue
    pub fn get_track_pool_position(&self, generation_direction: GenerationDirection) -> usize {
        let base_played_system_queue_entries = self
            .entries
            .iter()
            .filter(|entry| matches!(entry.source, PlaybackQueueEntrySource::System))
            .map(|entry| entry.track_id);

        let mut played_system_queue_entries = match generation_direction {
            GenerationDirection::Next => Either::Left(base_played_system_queue_entries.rev()),
            GenerationDirection::Previous => Either::Right(base_played_system_queue_entries),
        };

        loop {
            let Some(played_system_queue_entries) = played_system_queue_entries.next() else {
                break 0;
            };

            let Some(position) = self
                .track_pool
                .iter()
                .position(|&track_id| track_id == played_system_queue_entries)
            else {
                continue;
            };

            break position;
        }
    }

    pub fn generate_next_entries(&mut self, amount: usize) {
        match self.order {
            PlaybackQueueOrder::Sequential => self.generate_next_entries_sequentially(amount),
            PlaybackQueueOrder::Shuffle => self.generate_next_entries_shuffled(amount),
        }
    }

    pub fn generate_previous_entries(&mut self, amount: usize) {
        match self.order {
            PlaybackQueueOrder::Sequential => self.generate_previous_entries_sequentially(amount),
            PlaybackQueueOrder::Shuffle => {}
        }
    }

    pub fn generate_next_entries_sequentially(&mut self, amount: usize) {
        let track_pool_position = self.get_track_pool_position(GenerationDirection::Next);

        let track_pool = if self.repeat_mode.is_repeating() {
            Either::Left(self.track_pool.iter().cycle())
        } else {
            Either::Right(self.track_pool.iter())
        };

        let next_track_ids = track_pool
            .skip(track_pool_position + self.entries.len().min(1))
            .take(amount)
            .copied();

        let next_entries: Vec<PlaybackQueueEntry> = next_track_ids
            .map(|track_id| {
                // Manually increasing `next_entry_id` instead of calling `PlaybackQueue::get_next_entry_id`
                // to prevent borrow checker issues.
                let id = self.next_entry_id;
                self.next_entry_id += 1;

                PlaybackQueueEntry::system(id, track_id)
            })
            .collect();

        self.entries.extend(next_entries);
    }

    pub fn generate_next_entries_shuffled(&mut self, amount: usize) {
        let mut rng = rand::rng();

        let mut next_track_ids: Vec<TrackId> = if self.repeat_mode.is_repeating() {
            let recently_played_tracks: FxHashSet<TrackId> = if let Some(queue_skip) =
                self.entries.len().checked_sub(self.track_pool.len() / 2)
            {
                self.entries
                    .iter()
                    .skip(queue_skip)
                    .map(|entry| entry.track_id)
                    .collect()
            } else {
                self.entries.iter().map(|entry| entry.track_id).collect()
            };

            self.track_pool
                .iter()
                .filter(|&track_id| !recently_played_tracks.contains(track_id))
                .copied()
                .sample(&mut rng, amount)
        } else {
            let already_played_track_ids: FxHashSet<TrackId> =
                self.entries.iter().map(|entry| entry.track_id).collect();

            self.track_pool
                .iter()
                .filter(|&track_id| !already_played_track_ids.contains(track_id))
                .sample(&mut rng, amount)
                .into_iter()
                .copied()
                .collect()
        };

        next_track_ids.shuffle(&mut rng);

        let next_entries: Vec<PlaybackQueueEntry> = next_track_ids
            .into_iter()
            .map(|track_id| {
                let id = self.get_next_entry_id();

                PlaybackQueueEntry::system(id, track_id)
            })
            .collect();

        self.entries.extend(next_entries);
    }

    pub fn generate_previous_entries_sequentially(&mut self, amount: usize) {
        if self.track_pool.is_empty() {
            return;
        }

        let track_pool_position = self.get_track_pool_position(GenerationDirection::Previous);

        let track_pool_position = match track_pool_position.checked_sub(1) {
            Some(position) => position,
            // Wrap around if the repeat mode is a repeating one
            None if self.repeat_mode.is_repeating() => self.track_pool.len().saturating_sub(1),
            None => return,
        };

        let track_pool = if self.repeat_mode.is_repeating() {
            Either::Left(self.track_pool.iter().rev().cycle())
        } else {
            Either::Right(self.track_pool.iter().rev())
        };

        #[allow(clippy::needless_collect)]
        let previous_track_ids_reversed: Vec<TrackId> = track_pool
            .skip(self.track_pool.len() - 1 - track_pool_position)
            .take(amount)
            .copied()
            .collect();

        let previous_entries: Vec<PlaybackQueueEntry> = previous_track_ids_reversed
            .into_iter()
            .map(|track_id| {
                let id = self.get_next_entry_id();

                PlaybackQueueEntry::system(id, track_id)
            })
            .rev()
            .collect();

        if !previous_entries.is_empty() {
            self.cursor += previous_entries.len() + self.entries.len().min(1) - 1;
        }

        let mut entries = VecDeque::from(previous_entries);
        entries.append(&mut self.entries);

        self.entries = entries;
    }
}
