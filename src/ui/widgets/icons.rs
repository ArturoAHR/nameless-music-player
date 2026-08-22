use iced::{Font, Renderer, widget::text::Text};

use crate::ui::theme::Theme;

// music-player-icons.ttf
pub const LOADING: char = '\u{E830}';
pub const PLAY: char = '\u{E805}';
pub const PAUSE: char = '\u{E804}';
pub const STOP: char = '\u{E807}';
pub const PLAY_NEXT: char = '\u{E802}';
pub const PLAY_PREVIOUS: char = '\u{E803}';
pub const REPEAT: char = '\u{E809}';
pub const SHUFFLE: char = '\u{E80A}';
pub const SEQUENTIAL: char = '\u{E801}';
pub const EQUALIZER: char = '\u{E800}';
pub const MENU: char = '\u{E806}';
pub const VOLUME: char = '\u{E808}';
pub const VOLUME_MUTED: char = '\u{E80B}';
// TODO: Add missing icons to font.
pub const NO_REPEAT: char = '\u{E000}';
pub const REPEAT_ONE: char = '\u{E000}';
pub const CLOSE: char = '\u{E810}';
pub const ARROW_LEFT: char = '\u{EF177}';
pub const ARROW_RIGHT: char = '\u{EF178}';
pub const CHEVRON_LEFT: char = '\u{E80C}';
pub const CHEVRON_RIGHT: char = '\u{E80D}';
pub const CHEVRON_UP: char = '\u{E80E}';
pub const CHEVRON_DOWN: char = '\u{E80F}';
pub const TAG: char = '\u{E812}';
pub const MUSICAL_NOTE: char = '\u{E811}';

pub fn icon<'a>(codepoint: char) -> Text<'a, Theme, Renderer> {
    const ICON_FONT: Font = Font::with_name("music-player-icons");

    iced::widget::text(codepoint).font(ICON_FONT)
}
