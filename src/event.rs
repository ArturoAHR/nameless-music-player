use iced::Task;

use crate::{
    app::{App, Message},
    constants::PLAYBACK_QUEUE_LENGTH,
    playback::queue::PlaybackQueueEntry,
    track::models::Track,
};

#[derive(Debug, Clone)]
pub enum Event {
    AttemptedPlayingTrack,
    ActiveTrackChanged(Box<Option<Track>>),
    StartedPlayback,
    StoppedPlayback,
    EndOfTrack,
    QueueChanged(Vec<PlaybackQueueEntry>),
}

impl App {
    #[allow(clippy::needless_pass_by_value)]
    pub fn broadcast(&mut self, event: Event) -> Task<Message> {
        Task::batch(vec![
            self.notify_explorer_pane(&event),
            self.notify_main_pane(&event),
            self.notify_navigation_bar(&event),
            self.notify_playback_bar(&event),
            self.notify_queue_pane(&event),
            self.notify_status_bar(&event),
            self.notify_track_information_pane(&event),
        ])
    }

    pub fn broadcast_queue_changed(&mut self) -> Task<Message> {
        self.broadcast(Event::QueueChanged(
            self.playback_queue.get_queue_entries(PLAYBACK_QUEUE_LENGTH),
        ))
    }
}
