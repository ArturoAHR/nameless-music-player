use iced::{
    Element, Font,
    widget::{text, text::Catalog},
};

// music-player-icons.ttf
pub const LOADING: char = '\u{E830}';
pub const PLAY: char = '\u{E805}';
pub const PAUSE: char = '\u{E804}';
pub const STOP: char = '\u{E807}';
pub const PLAY_NEXT: char = '\u{E802}';
pub const PLAY_PREVIOUS: char = '\u{E803}';
pub const LOOP_TRACKLIST: char = '\u{E809}';
pub const SHUFFLE: char = '\u{E80A}';
pub const NO_SHUFFLE: char = '\u{E801}';
pub const EQUALIZER: char = '\u{E800}';
pub const MENU: char = '\u{E806}';
pub const VOLUME: char = '\u{E808}';
pub const VOLUME_MUTED: char = '\u{E80B}';

pub fn icon<'a, Message, Theme, Renderer>(
    codepoint: char,
) -> impl Into<Element<'a, Message, Theme, Renderer>>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::text::Renderer + 'a,
    <Renderer as iced::advanced::text::Renderer>::Font: From<iced::Font>,
{
    const ICON_FONT: Font = Font::with_name("music-player-icons");

    text(codepoint).font(ICON_FONT)
}
