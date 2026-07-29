use iced::{
    Element, Length, Renderer, Task,
    widget::{Space, container, text},
};
use iced_palace::widget::ellipsized_text;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::instrument;

use crate::{
    constants::PLAYBACK_QUEUE_LENGTH,
    event::Event,
    playback::queue::{
        PlaybackQueue,
        entry::{PlaybackQueueEntry, PlaybackQueueEntryId},
    },
    track::models::{Track, TrackId},
    ui::{
        theme::Theme,
        widgets::{
            icons::{self, icon},
            table::{column, table},
        },
    },
};

pub mod handler;

#[derive(Debug, Default)]
pub struct QueuePane {
    selected_entries: FxHashSet<PlaybackQueueEntryId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueueTableColumn {
    NowPlaying,
    TrackNumber,
    Title,
    Artist,
}

#[derive(Debug, Clone)]
pub enum Message {
    TrackRowDoubleClicked(PlaybackQueueEntryId),
    TrackRowSelected(FxHashSet<PlaybackQueueEntryId>),
}

#[derive(Debug, Clone)]
pub enum Outcome {}

impl QueuePane {
    #[instrument(skip(self), level = "debug")]
    pub fn update(&mut self, message: Message) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let outcomes = Vec::new();

        match message {
            Message::TrackRowDoubleClicked(_queue_entry) => {
                // TODO: Implement playing moving ahead in the queue.
            }
            Message::TrackRowSelected(selected_entries) => {
                self.selected_entries = selected_entries;
            }
        }

        (task, outcomes)
    }

    #[instrument(skip(self), level = "debug")]
    pub fn on_event(&mut self, event: &Event) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(
        &'a self,
        _theme: &Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
        playback_queue: &'a PlaybackQueue,
    ) -> Element<'a, Message, Theme, Renderer> {
        let columns = vec![
            column(
                QueueTableColumn::NowPlaying,
                None,
                move |entry: &PlaybackQueueEntry| {
                    // TODO: Properly derive queue position
                    if entry.track_id == 0 {
                        icon(icons::PLAY).into()
                    } else {
                        Space::new().into()
                    }
                },
            )
            .width(30.0),
            column(
                QueueTableColumn::TrackNumber,
                Some(text("#").into()),
                |_entry: &PlaybackQueueEntry| text("#"),
            )
            .width(35.0),
            column(
                QueueTableColumn::Title,
                Some(text("Title").into()),
                |entry: &PlaybackQueueEntry| {
                    let track_title = tracks
                        .get(&entry.track_id)
                        .and_then(|track| track.title.as_deref())
                        .unwrap_or("Missing title");

                    ellipsized_text(track_title).wrapping(text::Wrapping::None)
                },
            )
            .width(200.0)
            .resizable(true),
            column(
                QueueTableColumn::Artist,
                Some(text("Artist").into()),
                |entry: &PlaybackQueueEntry| {
                    let track_artist = tracks
                        .get(&entry.track_id)
                        .and_then(|track| track.artist.as_deref())
                        .unwrap_or("Unknown");

                    ellipsized_text(track_artist).wrapping(text::Wrapping::None)
                },
            )
            .width(200.0)
            .resizable(true),
        ];

        let queue_entries = playback_queue
            .entries
            .iter()
            .skip(playback_queue.cursor)
            .take(PLAYBACK_QUEUE_LENGTH)
            .collect();

        container(
            table(columns, queue_entries)
                .selected_rows(&self.selected_entries)
                .on_row_select(Message::TrackRowSelected)
                .on_row_double_click(Message::TrackRowDoubleClicked),
        )
        .height(Length::FillPortion(7))
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette.surface_raised.into()),
            ..container::Style::default()
        })
        .into()
    }
}
