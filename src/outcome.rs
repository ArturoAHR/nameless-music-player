use std::sync::Arc;

use iced::Task;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{
    app::{App, Message},
    constants::PLAYBACK_QUEUE_LENGTH,
    error::AppError,
    event::Event,
    playback::{
        self,
        controller::PlaybackControllerStatus,
        queue::{
            PlaybackQueueOrder,
            algorithm::{PlaybackQueueRandomShuffleAlgorithm, PlaybackQueueSequentialAlgorithm},
        },
    },
    track::models::TrackId,
};

#[derive(Debug, Clone)]
pub enum Outcome {
    Playback(PlaybackOutcome),
}

#[derive(Debug, Clone)]
pub enum PlaybackOutcome {
    Resume,
    Pause,
    Stop,
    Play(TrackId),
    StartQueue(TrackId),
    Seek {
        timestamp: u64,
        post_seek_status: PlaybackControllerStatus,
    },
    PlayNext,
    PlayPrevious,
    CycleRepeatMode,
    CycleOrder,
}

impl App {
    pub fn handle_outcome(&mut self, outcome: Outcome) -> Task<Message> {
        let outcome_task = match outcome {
            Outcome::Playback(outcome) => self.handle_playback_outcome(outcome),
        };

        match outcome_task {
            Ok(outcome_task) => outcome_task,
            Err(_error) => Task::none(), // TODO: Add error notification system
        }
    }

    #[instrument(skip(self))]
    fn handle_playback_outcome(
        &mut self,
        outcome: PlaybackOutcome,
    ) -> Result<Task<Message>, AppError> {
        let mut task = Task::none();

        match outcome {
            PlaybackOutcome::Resume => {
                self.playback_controller.resume()?;
            }
            PlaybackOutcome::Stop => {
                self.playback_controller.stop()?;
            }
            PlaybackOutcome::Pause => {
                self.playback_controller.pause()?;
            }
            PlaybackOutcome::Seek {
                post_seek_status,
                timestamp,
            } => {
                self.playback_controller.seek(timestamp)?;

                match post_seek_status {
                    PlaybackControllerStatus::Playing => self.playback_controller.resume()?,
                    PlaybackControllerStatus::Stopped => self.playback_controller.pause()?,
                }
            }

            PlaybackOutcome::Play(track_id) => task = self.play_track(track_id)?,
            PlaybackOutcome::StartQueue(track_id) => {
                task = self.play_track(track_id)?;

                self.playback_queue
                    .start(self.displayed_track_ids.clone(), Some(track_id));

                task = task.chain(Task::done(Message::PlaybackQueue(
                    playback::queue::Message::RegenerateNextTracks,
                )));
            }
            PlaybackOutcome::PlayNext => {
                let next_track_id = self.playback_queue.go_to_next();

                if let Some(next_track_id) = next_track_id {
                    task = self.play_track(next_track_id)?;
                }

                if self.playback_queue.get_remaining_tracks() <= PLAYBACK_QUEUE_LENGTH {
                    task = task.chain(Task::done(Message::PlaybackQueue(
                        playback::queue::Message::GenerateNextTracks,
                    )));
                } else {
                    task = task.chain(self.broadcast(Event::QueueChanged(
                        self.playback_queue.get_queue_entries(PLAYBACK_QUEUE_LENGTH),
                    )));
                }
            }
            PlaybackOutcome::PlayPrevious => {
                let previous_track_id = self.playback_queue.go_to_previous();

                if let Some(previous_track_id) = previous_track_id {
                    task = self.play_track(previous_track_id)?;
                }
            }
            PlaybackOutcome::CycleRepeatMode => {
                self.playback_repeat_mode = self.playback_repeat_mode.next();
            }
            PlaybackOutcome::CycleOrder => {
                self.playback_queue_order = self.playback_queue_order.next();

                match self.playback_queue_order {
                    PlaybackQueueOrder::Sequential => {
                        let algorithm = PlaybackQueueSequentialAlgorithm::new(&self.playback_queue);

                        self.playback_queue_algorithm = Arc::new(Mutex::new(Box::new(algorithm)));
                    }

                    PlaybackQueueOrder::Shuffle => {
                        let algorithm = PlaybackQueueRandomShuffleAlgorithm::default();

                        self.playback_queue_algorithm = Arc::new(Mutex::new(Box::new(algorithm)));
                    }
                }

                task = Task::done(Message::PlaybackQueue(
                    playback::queue::Message::RegenerateNextTracks,
                ));
            }
        }

        Ok(task)
    }
}
