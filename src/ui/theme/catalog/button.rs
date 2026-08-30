use iced::{
    Background, Border,
    border::Radius,
    widget::button::{Catalog, Status, Style},
};

use crate::ui::theme::{
    Theme,
    color::{darken, lighten, mix},
};

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

pub fn toggle(theme: &Theme, status: Status) -> Style {
    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => Some(theme.palette.surface.into()),
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
            radius: Radius::from(theme.sizes.border.radius_lg),
            color: theme.palette.border,
            width: 1.0,
        },
        ..Style::default()
    }
}

pub fn active_toggle(theme: &Theme, status: Status) -> Style {
    let base_color = mix(theme.palette.surface, theme.palette.accent, 0.1);

    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => Some(base_color.into()),
        Status::Pressed => Some(lighten(base_color, 0.2).into()),
        Status::Hovered => Some(lighten(base_color, 0.1).into()),
    };

    let text_color = match status {
        Status::Active | Status::Hovered | Status::Pressed => theme.palette.text_selected,
        Status::Disabled => mix(theme.palette.text_selected, theme.palette.text_muted, 0.1),
    };

    Style {
        background,
        text_color,
        border: Border {
            radius: Radius::from(theme.sizes.border.radius_lg),
            color: theme.palette.accent,
            width: 1.0,
        },
        ..Style::default()
    }
}

pub fn modal_footer_button(theme: &Theme, status: Status) -> Style {
    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => Some(theme.palette.surface_raised.into()),
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
            radius: Radius::from(theme.sizes.border.radius_lg),
            color: theme.palette.border,
            width: 1.0,
        },
        ..Style::default()
    }
}

pub fn clear_icon_button(theme: &Theme, status: Status) -> Style {
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
            radius: Radius::from(theme.sizes.border.radius_round),
            ..Border::default()
        },
        ..Style::default()
    }
}

pub fn accent_icon_button(theme: &Theme, status: Status) -> Style {
    let text_color = match status {
        Status::Active => theme.palette.text,
        Status::Hovered => theme.palette.text_selected,
        Status::Pressed => theme.palette.accent,
        Status::Disabled => theme.palette.text_muted,
    };

    Style {
        background: None,
        text_color,
        ..Style::default()
    }
}

pub fn explorer_pane_dropdown_controller(theme: &Theme, status: Status) -> Style {
    let text_color = match status {
        Status::Active => theme.palette.text_muted,
        Status::Hovered => lighten(theme.palette.text_muted, 0.1),
        Status::Pressed => lighten(theme.palette.text_muted, 0.2),
        Status::Disabled => darken(theme.palette.text_muted, 0.3),
    };

    Style {
        text_color,
        ..Style::default()
    }
}

pub fn explorer_pane_option(theme: &Theme, status: Status) -> Style {
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
        ..Style::default()
    }
}

pub fn active_explorer_pane_option(theme: &Theme, status: Status) -> Style {
    let base_color = mix(theme.palette.surface, theme.palette.accent, 0.1);

    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => Some(base_color.into()),
        Status::Pressed => Some(lighten(base_color, 0.2).into()),
        Status::Hovered => Some(lighten(base_color, 0.1).into()),
    };

    let text_color = match status {
        Status::Active | Status::Hovered | Status::Pressed => theme.palette.text_selected,
        Status::Disabled => mix(theme.palette.text_selected, theme.palette.text_muted, 0.1),
    };

    Style {
        background,
        text_color,
        ..Style::default()
    }
}

pub fn outline(theme: &Theme, status: Status) -> Style {
    let background: Option<Background> = match status {
        Status::Active | Status::Disabled => None,
        Status::Pressed => Some(lighten(theme.palette.hover, 0.1).into()),
        Status::Hovered => Some(theme.palette.hover.into()),
    };

    let text_color = match status {
        Status::Active | Status::Hovered | Status::Pressed => theme.palette.text_muted,
        Status::Disabled => darken(theme.palette.text_muted, 0.1),
    };

    Style {
        background,
        text_color,
        border: Border {
            radius: Radius::from(theme.sizes.border.radius_md),
            color: theme.palette.border,
            width: 1.0,
        },
        ..Style::default()
    }
}
