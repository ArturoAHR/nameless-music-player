use iced::{
    Alignment, Element, Font, Length, Renderer, alignment, font,
    widget::{button, column, container, row, slider, text},
};
use iced_palace::widget::ellipsized_text;
use rustc_hash::FxHashMap;

use crate::{
    playback::queue::{PlaybackQueue, PlaybackQueueOrder, PlaybackRepeatMode},
    track::{
        models::{Track, TrackId},
        utils::{get_track_duration_label, get_track_label},
    },
    ui::{
        components::playback_bar::{Message, PlaybackBarStatus},
        theme::{Theme, catalog},
        utils::label::format_duration,
        widgets::icons::{self, icon},
    },
};

pub fn playback_controls<'a>(
    theme: &Theme,
    status: &PlaybackBarStatus,
) -> Element<'a, Message, Theme, Renderer> {
    let play_previous = button(icon(icons::PLAY_PREVIOUS).size(theme.sizes.font.h1))
        .on_press(Message::PlayPrevious)
        .style(catalog::button::accent_icon_button);
    let play_next = button(icon(icons::PLAY_NEXT).size(theme.sizes.font.h1))
        .on_press(Message::PlayNext)
        .style(catalog::button::accent_icon_button);
    let play_button = match status {
        PlaybackBarStatus::Paused => button(icon(icons::PLAY).size(theme.sizes.font.h1))
            .on_press(Message::Resume)
            .style(catalog::button::accent_icon_button),
        PlaybackBarStatus::Playing => button(icon(icons::PAUSE).size(theme.sizes.font.h1))
            .on_press(Message::Pause)
            .style(catalog::button::accent_icon_button),
    };

    row![play_previous, play_button, play_next]
        .spacing(theme.sizes.space.xl)
        .into()
}

pub fn playing_track_progress_bar<'a>(
    theme: &Theme,
    tracks: &FxHashMap<TrackId, Track>,
    playing_track_id: Option<&TrackId>,
    playback_position: f64,
) -> Element<'a, Message, Theme, Renderer> {
    let mut total_frames = 1.0;
    let mut displayed_playback_position = 0.0;

    let mut track_name_label = String::new();
    let (track_duration_timestamp, playback_position_timestamp) = playing_track_id
        .and_then(|track_id| tracks.get(track_id))
        .map_or_else(
            || ("0:00".to_owned(), "0:00".to_owned()),
            |track| {
                total_frames = track.frames as f64;
                displayed_playback_position = playback_position;
                track_name_label = get_track_label(track);

                (
                    get_track_duration_label(track),
                    format_duration(
                        (displayed_playback_position / track.sample_rate as f64).floor() as u64,
                    ),
                )
            },
        );

    let current_time_label = format!("{playback_position_timestamp} / {track_duration_timestamp}");

    column![
        row![
            container(
                ellipsized_text(track_name_label)
                    .font(Font {
                        weight: font::Weight::Bold,
                        ..Font::DEFAULT
                    })
                    .wrapping(text::Wrapping::None),
            )
            .width(Length::Fill),
            text(current_time_label)
                .font(Font {
                    weight: font::Weight::Bold,
                    family: font::Family::Monospace,
                    ..Font::DEFAULT
                })
                .wrapping(text::Wrapping::None),
        ]
        .spacing(theme.sizes.space.xl),
        slider(
            0.0..=total_frames,
            displayed_playback_position,
            Message::Scrubbed
        )
        .on_release(Message::Seeked)
    ]
    .spacing(theme.sizes.space.md)
    .into()
}

pub fn volume_bar<'a>(
    theme: &Theme,
    volume_percentage: u8,
    muted: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let mut volume_percentage = volume_percentage.clamp(0, 100);

    let volume_icon = if muted || volume_percentage == 0 {
        volume_percentage = 0;
        icons::VOLUME_MUTED
    } else {
        icons::VOLUME
    };

    container(
        row![
            button(icon(volume_icon).size(theme.sizes.font.h2))
                .on_press(Message::MutePlayback)
                .width(40)
                .style(catalog::button::accent_icon_button),
            slider(0..=100, volume_percentage, Message::ChangeVolumePercentage,)
        ]
        .width(Length::Fixed(130.0))
        .align_y(Alignment::Center)
        .spacing(10.0),
    )
    .into()
}

pub fn queue_controls<'a>(
    theme: &Theme,
    playback_queue: &PlaybackQueue,
) -> Element<'a, Message, Theme, Renderer> {
    let repeat_mode_icon = match playback_queue.repeat_mode {
        PlaybackRepeatMode::NoRepeat => icons::NO_REPEAT,
        PlaybackRepeatMode::Repeat => icons::REPEAT,
        PlaybackRepeatMode::RepeatOne => icons::REPEAT_ONE,
    };

    let queue_order_icon = match playback_queue.order {
        PlaybackQueueOrder::Sequential => icons::SEQUENTIAL,
        PlaybackQueueOrder::Shuffle => icons::SHUFFLE,
    };

    row![
        button(icon(repeat_mode_icon).size(theme.sizes.font.h2))
            .on_press(Message::CycleRepeatMode)
            .width(40)
            .style(catalog::button::accent_icon_button),
        button(icon(queue_order_icon).size(theme.sizes.font.h2))
            .on_press(Message::CycleQueueOrder)
            .width(40)
            .style(catalog::button::accent_icon_button)
    ]
    .align_y(alignment::Vertical::Center)
    .spacing(theme.sizes.space.lg)
    .into()
}
