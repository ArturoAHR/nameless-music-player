use iced::{
    Border, Color, Shadow, Vector,
    border::Radius,
    widget::pick_list::{Catalog, Status, Style},
};

use crate::ui::theme::{
    Theme,
    color::{mix, with_alpha},
};

pub type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(|theme, _status| Style {
            background: theme.palette.surface.into(),
            border: Border::default(),
            handle_color: theme.palette.border,
            placeholder_color: theme.palette.text_muted,
            text_color: theme.palette.text,
        })
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
