use iced::{
    Element, Font, Length, Padding, Renderer, alignment,
    widget::{Space, button, center, column, container, row, text},
};

use crate::{
    tag::models::{Tag, TagGroup, TagId},
    track::models::Track,
    ui::{
        modals::tag_tracks::{Message, tag::get_tag_keys},
        theme::{Theme, catalog},
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
        button(icon(icons::CLOSE)).on_press(Message::Close)
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
    let selected_tag_group = tag_groups.get(tag_groups_cursor).unwrap();
    let tag_group_tags = tags
        .iter()
        .filter(|tag| tag.tag_group_id == selected_tag_group.id);
    let mut tag_characters = get_tag_keys();

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
            text(&selected_tag_group.name),
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
