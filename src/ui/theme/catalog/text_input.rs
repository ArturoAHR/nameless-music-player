use iced::{
    Background, Border,
    border::Radius,
    widget::text_input::{Catalog, Status, Style},
};

use crate::ui::theme::Theme;

pub type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, _status: Status) -> Style {
    Style {
        border: Border {
            color: theme.palette.border,
            radius: Radius::from(theme.sizes.border.radius_md),
            width: 1.0,
        },
        background: Background::Color(theme.palette.surface_raised),
        icon: theme.palette.border,
        placeholder: theme.palette.text_muted,
        selection: theme.palette.selected,
        value: theme.palette.text,
    }
}
