use std::path::{Path, PathBuf};

use iced_split::{horizontal_split, vertical_split};
use rustc_hash::FxHashMap;
use sqlx::SqlitePool;

use iced::{
    Element, Length, Size, Subscription, Task,
    time::{every, milliseconds},
    widget::{column, container},
    window,
};
use tracing::{error, info, instrument};

use crate::{
    app::Message::LoadTracks,
    constants::CURRENT_PLAYBACK_POSITION_POLL_INTERVAL_MS,
    error::AppError,
    library::scanner::scan_files_in_directory,
    playback::{
        self,
        controller::{PlaybackController, PlaybackControllerStatus},
        pipeline::thread::AudioPipelineThreadEvent,
        queue::PlaybackQueue,
    },
    subscriptions::{
        audio_device::watch_default_device,
        audio_pipeline_thread_events::audio_pipeline_thread_events,
    },
    tag::{
        models::{Tag, TagGroup},
        repository::{TagLibrary, load_tag_library},
    },
    track::{
        models::{Track, TrackId},
        repository::get_tracks,
    },
    ui::{
        self,
        components::{
            explorer_pane::ExplorerPane, main_pane::MainPane, navigation_bar::NavigationBar,
            playback_bar::PlaybackBar, queue_pane::QueuePane, status_bar::StatusBar,
            track_information_pane::TrackInformationPane,
        },
        handler::PaneSplit,
        theme::Theme,
    },
};

pub use crate::outcome::Outcome;

pub struct App {
    pub pool: SqlitePool,
    pub ui_scale: f32,
    pub theme: Theme,
    pub status: AppStatus,
    /// Tracks master list contains all the tracks, derived projections take
    pub tracks: FxHashMap<TrackId, Track>,
    pub tags: Vec<Tag>,
    pub tag_groups: Vec<TagGroup>,
    pub displayed_track_ids: Vec<TrackId>,
    pub current_playing_track_id: Option<TrackId>,

    pub window_size: Size,
    pub main_window_id: Option<window::Id>,

    pub pane_split_ratio: PaneSplitPositions,

    pub playback_controller: PlaybackController,
    pub playback_queue: PlaybackQueue,

    pub navigation_bar: NavigationBar,
    pub explorer_pane: ExplorerPane,
    pub main_pane: MainPane,
    pub queue_pane: QueuePane,
    pub track_information_pane: TrackInformationPane,
    pub status_bar: StatusBar,
    pub playback_bar: PlaybackBar,
}

#[derive(Debug)]
pub enum AppStatus {
    Idle,
    // TODO: Add progress with count
    AddingTracks,
    // TODO: Add optional error data
    FinishedAddingTracks,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadTracks,
    LoadedTracks(Result<Vec<Track>, AppError>),
    LoadTagLibrary,
    LoadedTagLibrary(Result<TagLibrary, AppError>),
    ScanDirectory(Option<Vec<PathBuf>>),
    ScannedDirectory(Result<(), AppError>),

    AudioPipelineEventChannelReady(
        iced::futures::channel::mpsc::UnboundedSender<AudioPipelineThreadEvent>,
    ),

    Ui(ui::Message),

    PlaybackController(playback::controller::Message),
    PlaybackQueue(playback::queue::Message),
}

pub struct PaneSplitPositions {
    pub explorer_main: f64,
    pub main_queue: f64,
    pub queue_track_information: f64,
}

impl App {
    #[instrument(skip(pool, playback_controller))]
    pub fn new(
        pool: SqlitePool,
        theme: Theme,
        ui_scale: f32,
        playback_controller: PlaybackController,
    ) -> (Self, Task<Message>) {
        info!("Setting up App instance.");

        (
            Self {
                pool,
                theme,
                ui_scale,
                status: AppStatus::Idle,
                tracks: FxHashMap::default(),
                tags: Vec::new(),
                tag_groups: Vec::new(),
                displayed_track_ids: Vec::new(),
                current_playing_track_id: None,

                window_size: Size::default(),
                main_window_id: None,

                pane_split_ratio: PaneSplitPositions {
                    explorer_main: 0.2,
                    main_queue: 0.7,
                    queue_track_information: 0.8,
                },

                playback_controller,
                playback_queue: PlaybackQueue::default(),

                navigation_bar: NavigationBar {},
                explorer_pane: ExplorerPane {},
                main_pane: MainPane::default(),
                queue_pane: QueuePane::default(),
                track_information_pane: TrackInformationPane {},
                status_bar: StatusBar {},
                playback_bar: PlaybackBar::new(),
            },
            Task::batch([
                Task::done(Message::LoadTracks),
                Task::done(Message::LoadTagLibrary),
                window::latest().and_then(|window_id| {
                    Task::batch([
                        Task::done(Message::Ui(ui::Message::GetWindowId(window_id))),
                        window::size(window_id)
                            .map(|size| Message::Ui(ui::Message::WindowResized(None, size))),
                    ])
                }),
            ]),
        )
    }

    pub fn title(&self) -> String {
        String::from("Soundlore")
    }

    #[instrument(skip(self), level = "debug",
        fields(
            current_track = self
                .current_playing_track_id
                .as_ref()
                .and_then(|track_id| self.tracks.get(track_id))
                .map(|track| {
                    Path::new(&track.file_path)
                        .file_name()
                        .unwrap_or_else(|| track.file_path.as_ref())
                        .to_str()
                })
        )
    )]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let mut task = Task::none();

        match message {
            Message::Ui(message) => task = self.handle_ui(message),

            Message::AudioPipelineEventChannelReady(audio_pipeline_event_sender) => {
                match self
                    .playback_controller
                    .initialize_playback(audio_pipeline_event_sender)
                {
                    Ok(()) => {}
                    Err(error) => error!("Failed to initialize playback: {error}"),
                }
            }

            Message::LoadTracks => {
                let pool = self.pool.clone();

                task = Task::perform(async move { get_tracks(pool).await }, Message::LoadedTracks);
            }
            Message::LoadedTracks(Ok(tracks)) => {
                self.tracks = tracks.into_iter().map(|track| (track.id, track)).collect();

                // TODO: Add loading state to main pane before setting the displayed tracks
                self.displayed_track_ids = self.tracks.keys().copied().collect();

                info!("Tracks loaded successfully");
            }
            Message::LoadedTracks(Err(error)) => {
                error!("Failed to load tracks: {error}");
            }
            Message::LoadTagLibrary => {
                let pool = self.pool.clone();

                task = Task::perform(
                    async { load_tag_library(pool).await },
                    Message::LoadedTagLibrary,
                );
            }
            Message::LoadedTagLibrary(Ok(tag_library)) => {
                let TagLibrary {
                    tags,
                    tag_groups,
                    track_tags: _,
                } = tag_library;

                self.tags = tags;
                self.tag_groups = tag_groups;

                info!("Tag library loaded successfully");
            }
            Message::LoadedTagLibrary(Err(error)) => {
                error!("Failed to load tag library: {error}");
            }
            Message::ScanDirectory(Some(directories)) => {
                let pool = self.pool.clone();
                self.status = AppStatus::AddingTracks;

                task = Task::perform(
                    async move { scan_files_in_directory(pool, directories).await },
                    Message::ScannedDirectory,
                );
            }
            Message::ScanDirectory(None) => {}
            Message::ScannedDirectory(scan_result) => {
                task = match scan_result {
                    Ok(()) => Task::done(LoadTracks),
                    Err(_) => Task::none(),
                };

                self.status = AppStatus::FinishedAddingTracks;
            }

            Message::PlaybackController(message) => task = self.handle_playback_controller(message),
        }

        task
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let navigation_bar = self.view_navigation_bar();

        let explorer_pane = self.view_explorer_pane();

        let main_pane = self.view_main_pane();

        let queue_pane = self.view_queue_pane();

        let track_information_pane = self.view_track_information_pane();

        let status_bar = self.view_status_bar();

        let playback_bar = self.view_playback_bar();

        let queue_track_information_pane_split = horizontal_split(
            queue_pane,
            track_information_pane,
            self.pane_split_ratio.queue_track_information as f32,
            |split_at| {
                Message::Ui(ui::Message::SplitDragged(
                    PaneSplit::QueueTrackInformation,
                    From::<f32>::from(split_at),
                ))
            },
        )
        .handle_width(5.0);

        let main_queue_pane_split = vertical_split(
            main_pane,
            queue_track_information_pane_split,
            self.pane_split_ratio.main_queue as f32,
            |split_at| {
                Message::Ui(ui::Message::SplitDragged(
                    PaneSplit::MainQueue,
                    From::<f32>::from(split_at),
                ))
            },
        )
        .handle_width(5.0);

        let explorer_main_pane_split = vertical_split(
            explorer_pane,
            main_queue_pane_split,
            self.pane_split_ratio.explorer_main as f32,
            |split_at| {
                Message::Ui(ui::Message::SplitDragged(
                    PaneSplit::ExplorerMain,
                    From::<f32>::from(split_at),
                ))
            },
        )
        .handle_width(5.0);

        column![
            navigation_bar,
            container(explorer_main_pane_split)
                .height(Length::Fill)
                .width(Length::Fill),
            status_bar,
            playback_bar
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = vec![
            Subscription::run(watch_default_device),
            Subscription::run(audio_pipeline_thread_events),
        ];

        if matches!(
            self.playback_controller.status,
            PlaybackControllerStatus::Playing
        ) {
            subscriptions.push(
                every(milliseconds(CURRENT_PLAYBACK_POSITION_POLL_INTERVAL_MS)).map(|_| {
                    Message::PlaybackController(
                        playback::controller::Message::PollPlaybackCurrentPlaybackPosition,
                    )
                }),
            );
        }

        subscriptions.push(window::resize_events().map(|(window_id, size)| {
            Message::Ui(ui::Message::WindowResized(Some(window_id), size))
        }));

        Subscription::batch(subscriptions)
    }

    pub fn scale_factor(&self) -> f32 {
        self.ui_scale
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
