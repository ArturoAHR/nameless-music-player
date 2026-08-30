use iced::{
    Element, Font, Length, Padding, Renderer, alignment,
    widget::{Space, button, center, column, container, row, slider, text},
};
use rustc_hash::FxHashMap;
use std::iter;

use crate::{
    tag::models::{Tag, TagGroup, TagId},
    track::{
        models::{Track, TrackId},
        utils::get_track_duration_label,
    },
    ui::{
        modals::tag_tracks::{Message, tag::get_tag_keys},
        theme::{Theme, catalog},
        utils::label::format_duration,
        widgets::icons::{self, icon},
    },
};

pub fn header<'a>(
    theme: &Theme,
    track: Option<&'a Track>,
    track_number: usize,
    track_total: usize,
) -> Element<'a, Message, Theme, Renderer> {
    let title = track
        .and_then(|track| track.title.as_deref())
        .unwrap_or("Missing title");
    let artist = track
        .and_then(|track| track.artist.as_deref())
        .unwrap_or("Unknown");
    let count = format!("Track {track_number} of {track_total} selected");

    container(row![
        column![
            text(title).size(theme.sizes.font.h2),
            text(artist).size(theme.sizes.font.h3),
            text(count).size(theme.sizes.font.body)
        ]
        .spacing(theme.sizes.space.lg),
        Space::new().width(Length::Fill),
        button(icon(icons::CLOSE))
            .on_press(Message::Close)
            .style(catalog::button::clear_icon_button)
    ])
    .height(128.0)
    .width(Length::Fill)
    .padding(Padding::from([
        theme.sizes.space.xxl,
        theme.sizes.space.xxxxl,
    ]))
    .style(catalog::container::modal_header)
    .into()
}

#[allow(clippy::implicit_hasher)]
pub fn playback<'a>(
    theme: &Theme,
    tracks: &FxHashMap<TrackId, Track>,
    current_tagging_track_id: Option<&TrackId>,
    current_position: f64,
) -> Element<'a, Message, Theme, Renderer> {
    let mut total_frames = 1.0;
    let mut current_frames = 0.0;

    let (track_duration_timestamp, current_position_timestamp) = current_tagging_track_id
        .and_then(|track_id| tracks.get(track_id))
        .map_or_else(
            || ("0:00".to_owned(), "0:00".to_owned()),
            |track| {
                total_frames = track.frames as f64;
                current_frames = current_position;

                (
                    get_track_duration_label(track),
                    format_duration((current_frames / track.sample_rate as f64).floor() as u64),
                )
            },
        );

    container(
        column![
            row![
                text(current_position_timestamp)
                    .size(theme.sizes.font.small)
                    .color(theme.palette.text_subtle),
                Space::new().width(Length::Fill),
                text(track_duration_timestamp)
                    .size(theme.sizes.font.small)
                    .color(theme.palette.text_subtle)
            ],
            slider(
                0.0..=total_frames,
                current_frames,
                Message::PlaybackScrubbed
            )
            .on_release(Message::PlaybackSeeked)
        ]
        .spacing(theme.sizes.space.md),
    )
    .padding(Padding::from([
        theme.sizes.space.xxl,
        theme.sizes.space.xxxxl,
    ]))
    .height(Length::Shrink)
    .width(Length::Fill)
    .style(catalog::container::background_surface_raised)
    .into()
}

pub fn tag_group_list<'a>(
    theme: &Theme,
    tag_groups: &'a [TagGroup],
    tag_groups_cursor: usize,
) -> Element<'a, Message, Theme, Renderer> {
    let tag_group_toggle_buttons: Vec<Element<'a, Message, Theme, Renderer>> = tag_groups
        .iter()
        .enumerate()
        .map(|(index, tag_group)| {
            let style = if index == tag_groups_cursor {
                catalog::button::active_toggle
            } else {
                catalog::button::toggle
            };

            button(text(&tag_group.name))
                .on_press(Message::SelectTabGroup(index))
                .padding(Padding::from([theme.sizes.space.lg, theme.sizes.space.xl]))
                .style(style)
                .into()
        })
        .collect();

    container(
        column![
            text("TAG GROUP")
                .size(theme.sizes.font.small)
                .color(theme.palette.text_subtle),
            row(tag_group_toggle_buttons)
                .spacing(theme.sizes.space.md)
                .wrap()
        ]
        .spacing(theme.sizes.space.xl),
    )
    .padding(Padding::from([
        theme.sizes.space.xxl,
        theme.sizes.space.xxxxl,
    ]))
    .height(Length::Shrink)
    .width(Length::Fill)
    .into()
}

pub fn tag_list<'a>(
    theme: &Theme,
    tag_groups: &'a [TagGroup],
    tags: &'a [Tag],
    tag_groups_cursor: usize,
    track_tags: Option<&[TagId]>,
) -> Element<'a, Message, Theme, Renderer> {
    let selected_tag_group = tag_groups.get(tag_groups_cursor);
    let mut tag_group_tags: Box<dyn Iterator<Item = &Tag>> = Box::new(iter::empty());
    let mut tag_characters = get_tag_keys();
    let mut selected_tag_group_name = "No tag group selected";

    if let Some(selected_tag_group) = selected_tag_group {
        selected_tag_group_name = &selected_tag_group.name;
        tag_group_tags = Box::new(
            tags.iter()
                .filter(|&tag| tag.tag_group_id == selected_tag_group.id),
        );
    }

    let tag_toggle_buttons: Vec<Element<'a, Message, Theme, Renderer>> = tag_group_tags
        .map(|tag| {
            let active = track_tags.is_some_and(|track_tags| track_tags.contains(&tag.id));

            let button_style = if active {
                catalog::button::active_toggle
            } else {
                catalog::button::toggle
            };

            let badge_style = if active {
                catalog::container::active_badge
            } else {
                catalog::container::badge
            };

            let tag_character = tag_characters.next().unwrap_or('?');

            button(
                row![
                    center(text(tag_character).font(Font::MONOSPACE))
                        .width(30.0)
                        .height(30.0)
                        .style(badge_style),
                    text(&tag.name)
                ]
                .align_y(alignment::Vertical::Center)
                .spacing(theme.sizes.space.lg),
            )
            .on_press(Message::ToggleTag(tag.id))
            .padding(Padding::from([theme.sizes.space.md, theme.sizes.space.lg]))
            .style(button_style)
            .into()
        })
        .collect();

    container(
        column![
            text(selected_tag_group_name),
            row(tag_toggle_buttons).spacing(theme.sizes.space.md).wrap()
        ]
        .spacing(theme.sizes.space.xl),
    )
    .height(Length::Shrink)
    .width(Length::Fill)
    .padding(Padding::from([
        theme.sizes.space.xxl,
        theme.sizes.space.xxxxl,
    ]))
    .into()
}

pub fn footer<'a>(
    theme: &Theme,
    track_tagging_queue: &[TrackId],
    track_tagging_queue_cursor: usize,
) -> Element<'a, Message, Theme, Renderer> {
    container(
        row![
            button(
                row![icon(icons::CHEVRON_LEFT), text!("Previous")]
                    .spacing(theme.sizes.space.lg)
                    .align_y(alignment::Vertical::Bottom)
            )
            .on_press_maybe((track_tagging_queue_cursor > 0).then_some(Message::GoToPreviousTrack))
            .padding(Padding::from([
                theme.sizes.space.lg,
                theme.sizes.space.xxxl,
            ]))
            .style(catalog::button::modal_footer_button),
            Space::new().width(Length::Fill),
            button(
                row![text!("Next"), icon(icons::CHEVRON_RIGHT)]
                    .spacing(theme.sizes.space.lg)
                    .align_y(alignment::Vertical::Bottom)
            )
            .on_press_maybe(
                (track_tagging_queue_cursor < track_tagging_queue.len() - 1)
                    .then_some(Message::GoToNextTrack)
            )
            .padding(Padding::from([
                theme.sizes.space.lg,
                theme.sizes.space.xxxl,
            ]))
            .style(catalog::button::modal_footer_button)
        ]
        .spacing(theme.sizes.space.xl),
    )
    .width(Length::Fill)
    .padding(Padding::from([
        theme.sizes.space.xl,
        theme.sizes.space.xxxxl,
    ]))
    .style(catalog::container::modal_footer)
    .into()
}
