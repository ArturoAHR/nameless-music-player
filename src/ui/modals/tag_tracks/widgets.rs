use iced::{
    Element, Length, Padding, Renderer,
    widget::{Space, button, column, container, row, text},
};

use crate::{
    track::models::Track,
    ui::{
        modals::tag_tracks::Message,
        theme::{Theme, catalog},
        widgets::icons::{self, icon},
    },
};

pub fn header<'a>(
    theme: &Theme,
    track: &'a Track,
    track_number: usize,
    track_total: usize,
) -> Element<'a, Message, Theme, Renderer> {
    let title = track.title.as_deref().unwrap_or("Missing title");
    let artist = track.artist.as_deref().unwrap_or("Unknown");
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
