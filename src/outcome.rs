use iced::Task;
use tracing::{error, instrument, warn};

use crate::{
    app::{App, Message, PlaybackOwner},
    error::AppError,
    outcome::TagOutcome::SaveTags,
    playback::{controller::PlaybackControllerStatus, queue::entry::PlaybackQueueEntryId},
    tag::{
        models::TagId,
        repository::{delete_track_tag, insert_track_tag},
    },
    track::models::TrackId,
    ui::components::playback_bar::PlaybackBarStatus,
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
    OpenManageTagsModal,
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
        post_seek_status: Option<PlaybackControllerStatus>,
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
                self.seek_timestamp(timestamp, post_seek_status)?;
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
                if !matches!(self.current_playback_owner, PlaybackOwner::PlaybackBar) {
                    if let Some(current_playing_track_id) =
                        self.playback_bar.current_playing_track_id
                    {
                        task = self.play_track_at_timestamp(
                            current_playing_track_id,
                            self.playback_bar.current_position as u64,
                        )?;

                        if matches!(self.playback_bar.status, PlaybackBarStatus::Paused) {
                            self.playback_controller.pause()?;
                        }
                    } else if matches!(
                        self.playback_controller.status,
                        PlaybackControllerStatus::Playing,
                    ) {
                        self.playback_controller.pause()?;
                    }

                    self.current_playback_owner = PlaybackOwner::PlaybackBar;
                }

                task = task.chain(self.modal_controller.close_modal());
            }
            ModalOutcome::OpenTagTracksModal(track_tagging_queue) => {
                let Some(first_track_id) = track_tagging_queue.first() else {
                    error!(
                        "Provided track tagging queue was empty, tagging modal won't be opened."
                    );

                    return Ok(task);
                };

                self.current_playback_owner = PlaybackOwner::TagTrackModal;

                task = self.play_track(*first_track_id)?;

                self.modal_controller
                    .open_tag_tracks_modal(track_tagging_queue);
            }
            ModalOutcome::OpenManageTagsModal => {
                self.modal_controller.open_manage_tags_modal();
            }
        }

        Ok(task)
    }

    #[instrument(skip(self))]
    pub fn handle_tag_outcome(&mut self, outcome: TagOutcome) -> Result<Task<Message>, AppError> {
        let mut task = Task::none();

        match outcome {
            TagOutcome::ToggleTag(track_id, tag_id) => {
                let tag_track_exists = self.track_tag_index.exists(track_id, tag_id);

                self.track_tag_index.toggle_track_tag(track_id, tag_id);

                // Instead of toggling at the database level we explicitly either delete or insert
                // to ensure consistency with the in memory track tag index
                let pool = self.pool.clone();
                task = Task::perform(
                    async move {
                        if tag_track_exists {
                            delete_track_tag(pool, track_id, tag_id).await
                        } else {
                            insert_track_tag(pool, track_id, tag_id).await
                        }
                    },
                    Message::ToggledTrackTag,
                );
            }
            SaveTags => {
                warn!("Saving current track tag index is still not implemented");
            }
        }

        Ok(task)
    }
}
