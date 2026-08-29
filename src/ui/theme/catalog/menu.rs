use iced::{
    Border, Color, Shadow, Vector,
    border::Radius,
    widget::overlay::menu::{Catalog, Style},
};

use crate::ui::theme::{
    Theme,
    color::{mix, with_alpha},
};

pub type StyleFn<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(|theme| Style {
            background: theme.palette.surface.into(),
            border: Border::default(),
            selected_background: theme.palette.selected.into(),
            selected_text_color: theme.palette.text_selected,
            shadow: Shadow::default(),
            text_color: theme.palette.text,
        })
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}
