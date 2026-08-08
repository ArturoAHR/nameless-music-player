use iced::{Size, Task, window};

use crate::{
    app::{self, App},
    ui::{
        components::{
            explorer_pane, main_pane, navigation_bar, playback_bar, queue_pane, status_bar,
            track_information_pane,
        },
        utils::pane::{are_pane_heights_valid, are_pane_widths_valid},
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    SplitDragged(PaneSplit, f64),
    WindowResized(Option<window::Id>, Size),
    GetWindowId(window::Id),

    NavigationBar(navigation_bar::Message),
    ExplorerPane(explorer_pane::Message),
    MainPane(main_pane::Message),
    QueuePane(queue_pane::Message),
    TrackInformationPane(track_information_pane::Message),
    StatusBar(status_bar::Message),
    PlaybackBar(playback_bar::Message),
}

#[derive(Debug, Clone)]
pub enum PaneSplit {
    /// The split between the explorer pane and main pane.
    ExplorerMain,
    /// The split between the main pane and the column with the queue pane and the track information pane.
    MainQueue,
    /// The split between the queue pane and the track information pane.
    QueueTrackInformation,
}

impl App {
    pub fn handle_ui(&mut self, message: Message) -> Task<app::Message> {
        let mut task = Task::none();

        match message {
            Message::SplitDragged(split, split_ratio) => {
                match split {
                    PaneSplit::ExplorerMain => {
                        // Since the main-queue split is a children of the explorer-main split, we
                        // need to calculate the new ratio of the main-queue split so the split stays
                        // in place.
                        let main_queue_split_ratio = 1.0
                            - (1.0 - self.pane_split_ratio.explorer_main)
                                * (1.0 - self.pane_split_ratio.main_queue)
                                / (1.0 - split_ratio);

                        if are_pane_widths_valid(
                            split_ratio,
                            main_queue_split_ratio,
                            From::<f32>::from(self.window_size.width),
                            From::<f32>::from(self.theme.sizes.component.pane_min_width),
                        ) {
                            self.pane_split_ratio.explorer_main = split_ratio;
                            self.pane_split_ratio.main_queue = main_queue_split_ratio;
                        }
                    }
                    PaneSplit::MainQueue => {
                        if are_pane_widths_valid(
                            self.pane_split_ratio.explorer_main,
                            split_ratio,
                            From::<f32>::from(self.window_size.width),
                            From::<f32>::from(self.theme.sizes.component.pane_min_width),
                        ) {
                            self.pane_split_ratio.main_queue = split_ratio;
                        }
                    }
                    PaneSplit::QueueTrackInformation => {
                        if are_pane_heights_valid(
                            split_ratio,
                            From::<f32>::from(self.window_size.height),
                            From::<f32>::from(self.theme.sizes.component.pane_min_height),
                        ) {
                            self.pane_split_ratio.queue_track_information = split_ratio;
                        }
                    }
                }
            }
            Message::WindowResized(window_id, size) => {
                if window_id.is_none() || window_id == self.main_window_id {
                    self.window_size = size;
                }
            }
            Message::GetWindowId(window_id) => self.main_window_id = Some(window_id),

            Message::NavigationBar(message) => task = self.handle_navigation_bar(message),
            Message::ExplorerPane(message) => task = self.handle_explorer_pane(message),
            Message::MainPane(message) => task = self.handle_main_pane(message),
            Message::QueuePane(message) => task = self.handle_queue_pane(message),
            Message::TrackInformationPane(message) => {
                task = self.handle_track_information_pane(message);
            }
            Message::StatusBar(message) => task = self.handle_status_bar(message),
            Message::PlaybackBar(message) => task = self.handle_playback_bar(message),
        }

        task
    }
}
