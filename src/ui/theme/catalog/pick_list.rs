use iced::{
    Border,
    border::Radius,
    widget::pick_list::{Catalog, Status, Style},
};

use crate::ui::theme::Theme;

pub type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let mut radius = Radius::from(theme.sizes.border.radius_md);

    if matches!(status, Status::Opened { is_hovered: _ }) {
        radius = radius.bottom(0.0);
    }

    Style {
        background: theme.palette.surface_sunken.into(),
        border: Border {
            color: theme.palette.border,
            width: 1.0,
            radius,
        },
        handle_color: theme.palette.border,
        placeholder_color: theme.palette.text_muted,
        text_color: theme.palette.text,
    }
}
