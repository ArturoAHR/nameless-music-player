use iced::Task;
use tracing::{error, instrument};

use crate::{
    app::{self, App},
    error::AppError,
    event::Event,
    playback::{
        controller::{PlaybackControllerError, PlaybackControllerStatus},
        pipeline::thread::AudioPipelineThreadEvent,
    },
    track::models::TrackId,
};

#[derive(Debug, Clone)]
pub enum Message {
    AudioPipelineEvent(AudioPipelineThreadEvent),
    PendingOutputDeviceChange,
    OutputDeviceChanged,
    OutputDeviceChangeFailed(PlaybackControllerError),
    PollPlaybackCurrentPlaybackPosition,
}

impl App {
    fn get_current_position(&self) -> Option<f64> {
        if matches!(
            self.playback_controller.status,
            PlaybackControllerStatus::Stopped
        ) {
            return None;
        }

        let audio_engine_generation = self.playback_controller.get_audio_engine_generation();
        if audio_engine_generation <= self.playback_bar.current_position_generation_threshold {
            return None;
        }

        let track = self
            .current_playing_track_id
            .and_then(|track_id| self.tracks.get(&track_id))?;

        let output_format = self.playback_controller.output_format.as_ref()?;

        let current_position = self.playback_controller.get_current_track_samples_played() as f64
            * (track.sample_rate as f64 / f64::from(output_format.sample_rate))
            / f64::from(output_format.channels);

        Some(current_position)
    }

    #[instrument(skip(self))]
    pub fn handle_playback_controller(&mut self, message: Message) -> Task<app::Message> {
        let mut task = Task::none();

        match message {
            Message::AudioPipelineEvent(event) => {
                if let Err(error) = self.playback_controller.handle_audio_pipeline_event(&event) {
                    error!("Playback controller failed to handle audio pipeline event: {error}");
                }

                #[allow(clippy::single_match)]
                match event {
                    AudioPipelineThreadEvent::TrackFinished => match self.play_next_track() {
                        Ok(event_tasks) => task = event_tasks,
                        Err(error) => {
                            error!("Could not play next track: {error}");
                        }
                    },
                    _ => {}
                }
            }
            Message::PollPlaybackCurrentPlaybackPosition => {
                if let Some(current_position) = self.get_current_position() {
                    task = self.broadcast(Event::PlaybackProgressed(current_position));
                }
            }
            Message::PendingOutputDeviceChange => {
                task = if let Err(error) = self.playback_controller.build_output() {
                    Task::done(app::Message::PlaybackController(
                        Message::OutputDeviceChangeFailed(error),
                    ))
                } else {
                    Task::done(app::Message::PlaybackController(
                        Message::OutputDeviceChanged,
                    ))
                };
            }
            Message::OutputDeviceChangeFailed(error) => {
                error!("Failed to initialize playback output: {error}");

                // TODO: Display error popup with user friendly message.
            }
            Message::OutputDeviceChanged => {}
        }

        task
    }

    #[instrument(skip(self))]
    pub fn play_track(&mut self, track_id: TrackId) -> Result<Task<app::Message>, AppError> {
        let track = self
            .tracks
            .get(&track_id)
            .ok_or_else(|| AppError::TrackNotFound {
                id: Some(track_id),
                path: None,
            })?
            .clone();

        let event_tasks = self.broadcast(Event::AttemptedPlayingTrack);

        self.playback_controller.play(track)?;

        self.current_playing_track_id = Some(track_id);

        Ok(event_tasks)
    }
}
