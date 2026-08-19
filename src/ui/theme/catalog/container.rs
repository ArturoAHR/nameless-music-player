use iced::{
    Border, Color, Shadow, Vector,
    border::Radius,
    widget::container::{Catalog, Style},
};

use crate::ui::theme::{
    Theme,
    color::{mix, with_alpha},
};

pub type StyleFn<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn background_surface_raised(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.surface_raised.into()),
        ..Style::default()
    }
}

pub fn context_menu(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.surface_raised.into()),
        text_color: None,
        border: Border {
            color: theme.palette.border,
            width: theme.sizes.border.width,
            radius: Radius::from(theme.sizes.border.radius_md),
        },
        shadow: Shadow {
            color: Color::BLACK,
            blur_radius: theme.sizes.border.radius_md,
            offset: Vector::new(theme.sizes.space.sm, theme.sizes.space.sm),
        },
        ..Style::default()
    }
}

pub fn modal(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.surface.into()),
        text_color: None,
        border: Border {
            color: theme.palette.border,
            width: theme.sizes.border.width,
            radius: Radius::from(theme.sizes.border.radius_lg),
        },
        shadow: Shadow {
            color: Color::BLACK,
            blur_radius: theme.sizes.border.radius_xxxl,
            offset: Vector::new(theme.sizes.space.md, theme.sizes.space.md),
        },
        ..Style::default()
    }
}

pub fn modal_backdrop(_theme: &Theme) -> Style {
    Style {
        background: Some(with_alpha(Color::BLACK, 0.8).into()),
        ..Style::default()
    }
}

pub fn modal_header(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.surface_raised.into()),
        border: Border {
            radius: Radius::default().top(theme.sizes.border.radius_lg),
            ..Border::default()
        },
        ..Style::default()
    }
}

pub fn separator(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.border.into()),
        ..Style::default()
    }
}

pub fn badge(theme: &Theme) -> Style {
    Style {
        background: Some(theme.palette.surface.into()),
        border: Border {
            color: theme.palette.border,
            width: 1.0,
            radius: Radius::from(theme.sizes.border.radius_md),
        },
        ..Style::default()
    }
}

pub fn active_badge(theme: &Theme) -> Style {
    Style {
        background: Some(mix(theme.palette.surface, theme.palette.accent, 0.2).into()),
        border: Border {
            color: theme.palette.accent,
            width: 1.0,
            radius: Radius::from(theme.sizes.border.radius_md),
        },
        ..Style::default()
    }
}
