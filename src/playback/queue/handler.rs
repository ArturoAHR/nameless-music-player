use std::sync::Arc;

use iced::Task;
use tracing::{error, info, instrument, warn};

use crate::{
    app::{self, App},
    constants::PLAYBACK_QUEUE_LENGTH,
    error::AppError,
    event::Event::{self, QueueChanged},
    playback::queue::PlaybackQueueError,
    track::models::TrackId,
};

#[derive(Debug, Clone)]
pub enum Message {
    /// Generates the next tracks based on the current ordering algorithm.
    GenerateNextTracks,
    /// Regenerates the next track ids by removing the next tracks, resetting the order algorithm
    /// and generating them again.
    RegenerateNextTracks,
    FinishedGeneratingNextTracks(Result<Vec<TrackId>, PlaybackQueueError>),
}

impl App {
    #[instrument(
        skip(self),
        fields(
            message,
            playback_repeat_mode = ?self.playback_repeat_mode,
            playback_queue_order = ?self.playback_queue_order,
        )
    )]
    pub fn handle_playback_queue(&mut self, message: Message) -> Task<app::Message> {
        let mut task = Task::none();

        match message {
            Message::GenerateNextTracks => {
                info!("Attempting to generate more tracks for the current queue");

                let playback_queue_algorithm = Arc::clone(&self.playback_queue_algorithm);
                let playback_queue = self.playback_queue.clone();
                let playback_repeat_mode = self.playback_repeat_mode;

                task = Task::perform(
                    async move {
                        let mut playback_queue_algorithm = playback_queue_algorithm.lock().await;

                        let amount = playback_queue_algorithm
                            .generate_next_tracks(
                                playback_queue,
                                playback_repeat_mode,
                                PLAYBACK_QUEUE_LENGTH,
                            )
                            .await?;

                        Ok(playback_queue_algorithm.take_next_tracks(Some(amount)))
                    },
                    Message::FinishedGeneratingNextTracks,
                )
                .map(app::Message::PlaybackQueue);
            }
            Message::RegenerateNextTracks => {
                self.playback_queue.truncate();

                let playback_queue_algorithm = Arc::clone(&self.playback_queue_algorithm);
                let playback_queue = self.playback_queue.clone();
                let playback_repeat_mode = self.playback_repeat_mode;

                task = Task::perform(
                    async move {
                        let mut playback_queue_algorithm = playback_queue_algorithm.lock().await;

                        playback_queue_algorithm.reset(&playback_queue);

                        let amount = playback_queue_algorithm
                            .generate_next_tracks(
                                playback_queue,
                                playback_repeat_mode,
                                PLAYBACK_QUEUE_LENGTH,
                            )
                            .await?;

                        Ok(playback_queue_algorithm.take_next_tracks(Some(amount)))
                    },
                    Message::FinishedGeneratingNextTracks,
                )
                .map(app::Message::PlaybackQueue);
            }
            Message::FinishedGeneratingNextTracks(Ok(next_track_ids)) => {
                info!("Generated {} tracks for queue", next_track_ids.len());

                self.playback_queue.extend(next_track_ids);

                task = self.broadcast(QueueChanged(
                    self.playback_queue.get_queue_entries(PLAYBACK_QUEUE_LENGTH),
                ));
            }
            Message::FinishedGeneratingNextTracks(Err(error)) => {
                error!("Failed to generate next queue tracks: {error} ");
            }
        }

        task
    }

    #[instrument(skip(self))]
    pub fn play_next_track(&mut self) -> Result<Task<app::Message>, AppError> {
        let mut task = if let Some(next_track_id) = self.playback_queue.go_to_next() {
            self.play_track(next_track_id)?
        } else {
            info!("Found no more track in the queue");

            Task::none()
        };

        if self.playback_queue.get_remaining_tracks() <= PLAYBACK_QUEUE_LENGTH {
            task = task.chain(Task::done(app::Message::PlaybackQueue(
                Message::GenerateNextTracks,
            )));
        } else {
            task = task.chain(self.broadcast(Event::QueueChanged(
                self.playback_queue.get_queue_entries(PLAYBACK_QUEUE_LENGTH),
            )));
        }

        Ok(task)
    }
}
