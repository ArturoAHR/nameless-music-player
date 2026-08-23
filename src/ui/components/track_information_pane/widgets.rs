use iced::{
    Element, Length, Renderer,
    widget::{column, text},
};

use crate::{
    track::{models::Track, utils::get_track_duration_label},
    ui::{components::track_information_pane::Message, theme::Theme},
};

pub fn track_information<'a>(
    theme: &Theme,
    track: &'a Track,
) -> Element<'a, Message, Theme, Renderer> {
    let title = track.title.as_deref().unwrap_or("Untitled");
    let artist = track.artist.as_deref().unwrap_or("Unknown");
    let album = track.album.as_deref();
    let year = track.year;
    let file_extension = track.file_format.to_ascii_uppercase();
    let sample_rate_khz = format!("{:.1} kHz", track.sample_rate as f32 / 1000.0);
    let bitrate_kbps = track
        .bitrate_kbps
        .map(|bitrate_kbps| format!("{bitrate_kbps}k"));
    let channels = match track.channels {
        1 => "Mono",
        2 => "Stereo",
        _ => "Unsupported",
    };
    let duration = get_track_duration_label(track);

    let mut secondary_track_information = vec![text(artist).size(theme.sizes.font.body).into()];

    if let Some(album) = album {
        secondary_track_information.push(text(album).size(theme.sizes.font.body).into());
    }

    if let Some(year) = year {
        secondary_track_information.push(text(year).size(theme.sizes.font.body).into());
    }

    let track_file_information = bitrate_kbps.map_or_else(
        || format!("{file_extension} {sample_rate_khz}, {channels}, {duration}"),
        |bitrate_kbps| {
            format!("{file_extension} {sample_rate_khz}, {bitrate_kbps}, {channels}, {duration}")
        },
    );

    column![
        text(title),
        column(secondary_track_information).spacing(theme.sizes.space.md),
        text(track_file_information)
            .size(theme.sizes.font.small)
            .color(theme.palette.text_muted)
    ]
    .width(Length::Fill)
    .spacing(theme.sizes.space.lg)
    .into()
}
