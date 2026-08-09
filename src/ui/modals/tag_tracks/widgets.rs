use iced::{
    Element, Length, Padding, Renderer,
    widget::{Space, button, column, container, row, text},
};

use crate::{
    tag::models::TagGroup,
    track::models::Track,
    ui::{
        modals::tag_tracks::Message,
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
    .padding(Padding::from([20.0, 28.0]))
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
                .style(style)
                .into()
        })
        .collect();

    container(
        column![
            text("TAG GROUP"),
            row(tag_group_toggle_buttons).spacing(theme.sizes.space.md)
        ]
        .spacing(theme.sizes.space.xl),
    )
    .height(140.0)
    .padding(Padding::from([20.0, 28.0]))
    .width(Length::Fill)
    .into()
}
