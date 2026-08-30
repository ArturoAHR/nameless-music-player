use iced::{Font, Renderer, widget::text::Text};

use crate::ui::theme::Theme;

// music-player-icons.ttf
pub const LOADING: char = '\u{E830}';
pub const PLAY: char = '\u{E805}';
pub const PAUSE: char = '\u{E804}';
pub const STOP: char = '\u{E818}';
pub const PLAY_NEXT: char = '\u{E802}';
pub const PLAY_PREVIOUS: char = '\u{E803}';
pub const REPEAT: char = '\u{E809}';
pub const SHUFFLE: char = '\u{E81B}';
pub const SEQUENTIAL: char = '\u{E801}';
pub const EQUALIZER: char = '\u{E800}';
pub const MENU: char = '\u{E806}';
pub const VOLUME: char = '\u{E816}';
pub const VOLUME_MUTED: char = '\u{E815}';
pub const NO_REPEAT: char = '\u{E81D}';
pub const REPEAT_ONE: char = '\u{E814}';
pub const CLOSE: char = '\u{E810}';
pub const ARROW_LEFT: char = '\u{EF177}';
pub const ARROW_RIGHT: char = '\u{E801}';
pub const CHEVRON_LEFT: char = '\u{E828}';
pub const CHEVRON_RIGHT: char = '\u{E827}';
pub const CHEVRON_UP: char = '\u{E826}';
pub const CHEVRON_DOWN: char = '\u{E829}';
pub const TAG: char = '\u{E817}';
pub const MUSICAL_NOTE: char = '\u{E821}';
pub const SEARCH: char = '\u{E81C}';

pub fn icon<'a>(codepoint: char) -> Text<'a, Theme, Renderer> {
    const ICON_FONT: Font = Font::with_name("music-player-icons");

    iced::widget::text(codepoint).font(ICON_FONT)
}
