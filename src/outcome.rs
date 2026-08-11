use iced::Task;
use tracing::{instrument, warn};

use crate::{
    app::{App, Message},
    error::AppError,
    outcome::TagOutcome::SaveTags,
    playback::{controller::PlaybackControllerStatus, queue::entry::PlaybackQueueEntryId},
    tag::models::TagId,
    track::models::TrackId,
};

#[derive(Debug, Clone)]
pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
    Tag(TagOutcome),
}

#[derive(Debug, Clone)]
pub enum ModalOutcome {
    CloseModal,
    OpenTagTracksModal(Vec<TrackId>),
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
    PlayQueueEntry(PlaybackQueueEntryId),
    CycleRepeatMode,
    CycleOrder,
    QueueNext(Vec<TrackId>),
}

#[derive(Debug, Clone)]
pub enum TagOutcome {
    ToggleTag(TrackId, TagId),
    SaveTags,
}

impl App {
    pub fn handle_outcome(&mut self, outcome: Outcome) -> Task<Message> {
        let outcome_task = match outcome {
            Outcome::Playback(outcome) => self.handle_playback_outcome(outcome),
            Outcome::Modal(outcome) => self.handle_modal_outcome(outcome),
            Outcome::Tag(outcome) => self.handle_tag_outcome(outcome),
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
                self.playback_generation_threshold =
                    self.playback_controller.get_audio_engine_generation();

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
            }
            PlaybackOutcome::PlayNext => {
                let next_track_id = self.playback_queue.next();

                if let Some(next_track_id) = next_track_id {
                    task = self.play_track(next_track_id)?;
                }
            }
            PlaybackOutcome::PlayPrevious => {
                let previous_track_id = self.playback_queue.previous();

                if let Some(previous_track_id) = previous_track_id {
                    task = self.play_track(previous_track_id)?;
                }
            }
            PlaybackOutcome::PlayQueueEntry(entry_id) => {
                if let Some((queue_entry_index, playback_queue_entry_track_id)) = self
                    .playback_queue
                    .entries
                    .iter()
                    .enumerate()
                    .find(|(_, entry)| entry.id == entry_id)
                    .map(|(index, entry)| (index, entry.track_id))
                {
                    self.playback_queue.set_cursor(queue_entry_index);

                    task = self.play_track(playback_queue_entry_track_id)?;
                }
            }
            PlaybackOutcome::CycleRepeatMode => {
                self.playback_queue.cycle_repeat_mode();
            }
            PlaybackOutcome::CycleOrder => {
                self.playback_queue.cycle_queue_order();
            }
            PlaybackOutcome::QueueNext(queued_track_ids) => {
                for queued_track_id in queued_track_ids {
                    if self.playback_queue.entries.is_empty() {
                        self.playback_queue
                            .start(self.displayed_track_ids.clone(), Some(queued_track_id));

                        self.current_playing_track_id = Some(queued_track_id);
                    } else {
                        self.playback_queue.insert_next(queued_track_id);
                    }
                }
            }
        }

        Ok(task)
    }

    #[instrument(skip(self))]
    pub fn handle_modal_outcome(
        &mut self,
        outcome: ModalOutcome,
    ) -> Result<Task<Message>, AppError> {
        let mut task = Task::none();

        match outcome {
            ModalOutcome::CloseModal => {
                task = self.modal_controller.close_modal();
            }
            ModalOutcome::OpenTagTracksModal(track_tagging_queue) => {
                self.modal_controller
                    .open_tag_tracks_modal(track_tagging_queue);
            }
        }

        Ok(task)
    }

    #[instrument(skip(self))]
    pub fn handle_tag_outcome(&mut self, outcome: TagOutcome) -> Result<Task<Message>, AppError> {
        let task = Task::none();

        match outcome {
            TagOutcome::ToggleTag(track_id, tag_id) => {
                self.track_tag_index.toggle_track_tag(track_id, tag_id);
            }
            SaveTags => {
                warn!("Saving current track tag index is still not implemented");
            }
        }

        Ok(task)
    }
}
