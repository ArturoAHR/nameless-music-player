use iced::{
    Background, Border,
    border::Radius,
    widget::button::{Catalog, Status, Style},
};

use crate::ui::theme::{Theme, color::lighten};

pub type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| {
            let background = match status {
                Status::Active => theme.palette.surface_raised,
                Status::Pressed => theme.palette.surface_overlay,
                Status::Disabled => theme.palette.surface_sunken,
                Status::Hovered => theme.palette.hover,
            };

            Style {
                background: Some(background.into()),
                text_color: theme.palette.text,
                border: Border {
                    color: theme.palette.border,
                    width: theme.sizes.border.width,
                    radius: theme.sizes.border.radius_sm.into(),
                },
                ..Style::default()
            }
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn menu_option(theme: &Theme, status: Status) -> Style {
    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => None,
        Status::Pressed => Some(lighten(theme.palette.hover, 0.1).into()),
        Status::Hovered => Some(theme.palette.hover.into()),
    };

    let text_color = match status {
        Status::Active | Status::Hovered | Status::Pressed => theme.palette.text,
        Status::Disabled => theme.palette.text_muted,
    };

    Style {
        background,
        text_color,
        border: Border {
            radius: Radius::from(theme.sizes.border.radius_sm),
            ..Border::default()
        },
        ..Style::default()
    }
}
