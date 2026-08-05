use iced::{
    Border, Color, Shadow, Vector,
    border::Radius,
    widget::container::{Catalog, Style},
};

use crate::ui::theme::Theme;

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
