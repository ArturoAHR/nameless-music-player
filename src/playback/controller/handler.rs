use iced::Task;
use tracing::{error, instrument};

use crate::{
    app::{self, App, PlaybackOwner},
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
        if audio_engine_generation <= self.playback_generation_threshold {
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
                    AudioPipelineThreadEvent::TrackFinished
                        if matches!(self.current_playback_owner, PlaybackOwner::PlaybackBar) =>
                    {
                        match self.play_next_track() {
                            Ok(event_tasks) => task = event_tasks,
                            Err(error) => {
                                error!("Could not play next track: {error}");
                            }
                        }
                    }
                    AudioPipelineThreadEvent::TrackFinished
                        if matches!(self.current_playback_owner, PlaybackOwner::TagTrackModal)
                            && let Some(track_id) = self.current_playing_track_id =>
                    {
                        match self.play_track(track_id) {
                            Ok(event_tasks) => task = event_tasks,
                            Err(error) => {
                                error!("Could not play next track: {error}");
                            }
                        }
                    }
                    AudioPipelineThreadEvent::ActiveTrackChanged(track_id) => {
                        task = self.broadcast(Event::ActiveTrackChanged(Some(track_id)));
                    }
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

        self.playback_generation_threshold = self.playback_controller.get_audio_engine_generation();

        self.playback_controller.play(track)?;

        self.current_playing_track_id = Some(track_id);

        Ok(event_tasks)
    }

    #[instrument(skip(self))]
    pub fn seek_timestamp(
        &mut self,
        timestamp: u64,
        post_seek_status: PlaybackControllerStatus,
    ) -> Result<(), AppError> {
        self.playback_generation_threshold = self.playback_controller.get_audio_engine_generation();

        self.playback_controller.seek(timestamp)?;

        match post_seek_status {
            PlaybackControllerStatus::Playing => self.playback_controller.resume()?,
            PlaybackControllerStatus::Stopped => self.playback_controller.pause()?,
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn play_track_at_timestamp(
        &mut self,
        track_id: TrackId,
        timestamp: u64,
    ) -> Result<Task<app::Message>, AppError> {
        let track = self
            .tracks
            .get(&track_id)
            .ok_or_else(|| AppError::TrackNotFound {
                id: Some(track_id),
                path: None,
            })?
            .clone();

        let task = self.broadcast(Event::AttemptedPlayingTrackAtDuration(timestamp));

        // Both play and seek increase the generation counter, the play command will emit some
        // Playback Duration Progress events at the start.
        self.playback_generation_threshold =
            self.playback_controller.get_audio_engine_generation() + 1;

        // TODO: Add proper play at timestamp command in audio pipeline
        self.playback_controller.play(track)?;

        self.current_playing_track_id = Some(track_id);

        self.playback_controller.seek(timestamp)?;

        Ok(task)
    }
}
