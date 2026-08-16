use iced::{Background, Border, Color, Shadow, Vector, border::Radius};
use iced_aw::style::{
    Status,
    menu_bar::{Catalog, Style},
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
        bar_background: Color::TRANSPARENT.into(),
        bar_border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },

        menu_background: theme.palette.surface_raised.into(),
        menu_border: Border {
            color: theme.palette.border,
            width: theme.sizes.border.width,
            radius: Radius::from(theme.sizes.border.radius_md),
        },
        menu_shadow: Shadow {
            color: Color::BLACK,
            blur_radius: theme.sizes.border.radius_md,
            offset: Vector::new(theme.sizes.space.sm, theme.sizes.space.sm),
        },

        path: Background::Color(theme.palette.hover),
        path_border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: theme.sizes.border.radius_sm.into(),
        },

        ..Style::default()
    }
}
