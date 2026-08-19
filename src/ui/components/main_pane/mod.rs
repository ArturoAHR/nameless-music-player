use iced::{
    Element, Length, Renderer, Task, alignment,
    widget::{Space, column, container, text},
};
use iced_aw::ContextMenu;
use iced_palace::widget::ellipsized_text;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::instrument;

use crate::{
    event::Event,
    outcome::{ModalOutcome, PlaybackOutcome},
    track::models::{Track, TrackId},
    ui::{
        theme::{Theme, catalog},
        utils::label::format_duration,
        widgets::{
            icons::{self, icon},
            menu::menu_option,
            table::{self, table},
        },
    },
};

pub mod handler;

#[derive(Debug, Default)]
pub struct MainPane {
    pub selected_track_ids: FxHashSet<i64>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TrackRowDoubleClicked(TrackId),
    TrackRowSelected(FxHashSet<TrackId>),
    ColumnHeaderCellClicked(TrackTableColumn),
    QueueNext,
    TagSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrackTableColumn {
    NowPlaying,
    Title,
    Artist,
    Duration,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Playback(PlaybackOutcome),
    Modal(ModalOutcome),
}

#[derive(Debug)]
pub struct MainPaneUpdateContext {}

impl MainPane {
    #[instrument(skip(self), level = "debug")]
    pub fn update(
        &mut self,
        event: Message,
        displayed_track_ids: &Vec<TrackId>,
    ) -> (Task<Message>, Vec<Outcome>) {
        let task = Task::none();
        let mut outcomes = Vec::new();

        match event {
            Message::TrackRowDoubleClicked(track_id) => {
                outcomes.push(Outcome::Playback(PlaybackOutcome::StartQueue(track_id)));
            }
            Message::TrackRowSelected(selected_track_ids) => {
                self.selected_track_ids = selected_track_ids.into_iter().collect();
            }
            Message::ColumnHeaderCellClicked(_column_id) => {}
            Message::QueueNext => {
                if !self.selected_track_ids.is_empty() {
                    let queued_track_ids = displayed_track_ids
                        .iter()
                        .filter(|track_id| self.selected_track_ids.contains(track_id))
                        .copied()
                        .collect();

                    outcomes.push(Outcome::Playback(PlaybackOutcome::QueueNext(
                        queued_track_ids,
                    )));
                }
            }
            Message::TagSelection => {
                if !self.selected_track_ids.is_empty() {
                    let track_tagging_queue = displayed_track_ids
                        .iter()
                        .filter(|track_id| self.selected_track_ids.contains(track_id))
                        .copied()
                        .collect();

                    outcomes.push(Outcome::Modal(ModalOutcome::OpenTagTracksModal(
                        track_tagging_queue,
                    )));
                }
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
        _theme: &'a Theme,
        tracks: &'a FxHashMap<TrackId, Track>,
        displayed_track_ids: &Vec<TrackId>,
        current_playing_track_id: Option<&TrackId>,
    ) -> Element<'a, Message, Theme, Renderer> {
        let current_playing_track_id = current_playing_track_id.copied().unwrap_or(-1);

        let columns = vec![
            table::column(TrackTableColumn::NowPlaying, None, move |track: &Track| {
                if track.id == current_playing_track_id {
                    Element::from(icon(icons::PLAY))
                } else {
                    Element::from(Space::new())
                }
            })
            .width(30.0),
            table::column(
                TrackTableColumn::Artist,
                Some(text("Artist").into()),
                |track: &Track| {
                    ellipsized_text(track.artist.as_deref().unwrap_or("Unknown"))
                        .wrapping(text::Wrapping::None)
                },
            )
            .width(200.0)
            .resizable(true),
            table::column(
                TrackTableColumn::Title,
                Some(text("Title").into()),
                |track: &Track| {
                    ellipsized_text(track.title.as_deref().unwrap_or("Untitled"))
                        .wrapping(text::Wrapping::None)
                },
            )
            .width(200.0)
            .resizable(true),
            table::column(
                TrackTableColumn::Duration,
                Some(text("Duration").into()),
                |track: &Track| {
                    ellipsized_text(format_duration(
                        track.frames as u64 / track.sample_rate as u64,
                    ))
                    .wrapping(text::Wrapping::None)
                },
            )
            .width(50.0)
            .resizable(true)
            .align_x(alignment::Horizontal::Right),
        ];

        container(ContextMenu::new(
            table(
                columns,
                displayed_track_ids
                    .iter()
                    .filter_map(|track_id| tracks.get(track_id))
                    .collect(),
            )
            .selected_rows(&self.selected_track_ids)
            .on_row_select(Message::TrackRowSelected)
            .on_row_double_click(Message::TrackRowDoubleClicked)
            .on_header_cell_click(Message::ColumnHeaderCellClicked),
            || {
                if self.selected_track_ids.is_empty() {
                    Space::new().into()
                } else {
                    container(
                        column![
                            menu_option("Queue Next", Some(Message::QueueNext)),
                            menu_option("Tag Selection", Some(Message::TagSelection))
                        ]
                        .width(Length::Fill),
                    )
                    .width(180.0)
                    .padding(6.0)
                    .style(catalog::container::context_menu)
                    .into()
                }
            },
        ))
        .height(Length::Fill)
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette.surface.into()),
            ..container::Style::default()
        })
        .into()
    }
}
