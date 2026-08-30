use iced::{
    Alignment, Element, Length, Padding, Renderer, Task,
    widget::{container, row},
};
use rustc_hash::FxHashMap;
use tracing::{instrument, trace};

use crate::{
    app::PlaybackOwner,
    event::Event,
    outcome::PlaybackOutcome,
    playback::{controller::PlaybackControllerStatus, queue::PlaybackQueue},
    track::models::{Track, TrackId},
    ui::{
        components::playback_bar::widgets::{
            playback_controls, playing_track_progress_bar, queue_controls, volume_bar,
        },
        theme::{Theme, catalog},
    },
};

pub mod handler;
pub mod widgets;

#[derive(Debug)]
pub struct PlaybackBar {
    pub playback_position: f64,
    pub playing_track_id: Option<TrackId>,

    pub status: PlaybackBarStatus,

    pub volume_percentage: u8,
    pub muted: bool,
}

#[derive(Debug)]
pub enum PlaybackBarStatus {
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub enum Message {
    Pause,
    Resume,
    PlayNext,
    PlayPrevious,
    Scrubbed(f64),
    Seeked,
    ChangeVolumePercentage(u8),
    MutePlayback,
    CycleRepeatMode,
    CycleQueueOrder,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Playback(PlaybackOutcome),
}

/*
 * TODO:
 * - Fix icons with consistent design.
 */
impl PlaybackBar {
    pub fn new() -> Self {
        Self {
            status: PlaybackBarStatus::Playing,
            playback_position: 0.0,
            playing_track_id: None,

            muted: false,
            volume_percentage: 50,
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub fn update(
        &mut self,
        message: Message,
        playback_controller_status: &PlaybackControllerStatus,
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match message {
            Message::Scrubbed(position) => {
                self.playback_position = position;

                if matches!(
                    playback_controller_status,
                    PlaybackControllerStatus::Playing
                ) {
                    outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
                }
            }

            Message::Seeked => {
                let pre_seek_status = match self.status {
                    PlaybackBarStatus::Playing => PlaybackControllerStatus::Playing,
                    PlaybackBarStatus::Paused => PlaybackControllerStatus::Stopped,
                };

                outcomes.push(Outcome::Playback(PlaybackOutcome::Seek {
                    timestamp: self.playback_position.round() as u64,
                    post_seek_status: Some(pre_seek_status),
                }));
            }
            Message::Resume if self.playing_track_id.is_some() => {
                self.status = PlaybackBarStatus::Playing;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Resume));
            }
            Message::Pause if self.playing_track_id.is_some() => {
                self.status = PlaybackBarStatus::Paused;

                outcomes.push(Outcome::Playback(PlaybackOutcome::Pause));
            }
            Message::PlayNext => outcomes.push(Outcome::Playback(PlaybackOutcome::PlayNext)),
            Message::PlayPrevious => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::PlayPrevious));
            }

            // TODO: Add playback outcome to change volume
            Message::ChangeVolumePercentage(volume_percentage) => {
                self.volume_percentage = volume_percentage;

                outcomes.push(Outcome::Playback(PlaybackOutcome::SetVolumePercentage(
                    volume_percentage,
                )));
            }
            // TODO: Add playback outcome to change volume
            Message::MutePlayback => {
                self.muted = !self.muted;

                outcomes.push(Outcome::Playback(PlaybackOutcome::SetVolumePercentage(
                    if self.muted {
                        0
                    } else {
                        self.volume_percentage
                    },
                )));
            }
            // TODO: Wire these two changes in upper for queue functionality
            Message::CycleRepeatMode => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::CycleRepeatMode));
            }
            Message::CycleQueueOrder => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::CycleOrder));
            }

            Message::Resume | Message::Pause => {}
        }

        (task, outcomes)
    }

    #[allow(clippy::single_match)]
    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event, playback_owner: &PlaybackOwner) -> Task<Message> {
        let task = Task::none();

        if !matches!(playback_owner, PlaybackOwner::PlaybackBar) {
            trace!("Playback Bar does not own the playback currently, ignoring event");

            return task;
        }

        match event {
            Event::AttemptedPlayingTrack => {
                self.status = PlaybackBarStatus::Playing;

                self.playback_position = 0.0;
            }
            Event::PlaybackProgressed(position) => {
                self.playback_position = *position;
            }
            Event::ActiveTrackChanged(track_id) => {
                self.playing_track_id = *track_id;
            }
            _ => {}
        }

        task
    }

    pub fn view<'a>(
        &'a self,
        theme: &Theme,
        tracks: &FxHashMap<TrackId, Track>,
        playback_queue: &PlaybackQueue,
    ) -> Element<'a, Message, Theme, Renderer> {
        container(
            row![
                playback_controls(theme, &self.status),
                playing_track_progress_bar(
                    theme,
                    tracks,
                    self.playing_track_id.as_ref(),
                    self.playback_position
                ),
                volume_bar(theme, self.volume_percentage, self.muted),
                queue_controls(theme, playback_queue),
            ]
            .align_y(Alignment::Center)
            .spacing(theme.sizes.space.xxxxl),
        )
        .height(Length::Fixed(theme.sizes.component.playback_bar_height))
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .padding(Padding::from([0.0, theme.sizes.space.xxxl]))
        .style(catalog::container::background_surface_raised)
        .into()
    }
}

impl Default for PlaybackBar {
    fn default() -> Self {
        Self::new()
    }
}
