use iced::{
    Border, Color, Shadow,
    border::Radius,
    widget::{
        container,
        scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style},
    },
};

use crate::ui::theme::{
    Theme,
    color::{darken, lighten},
};

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

pub fn default(theme: &Theme, status: Status) -> Style {
    let base_scroller_color = lighten(theme.palette.surface, 0.1);

    let mut horizontal_scroller_color = base_scroller_color;
    let mut vertical_scroller_color = base_scroller_color;

    match status {
        Status::Active {
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_dragged {
                horizontal_scroller_color = lighten(base_scroller_color, 0.2);
            }

            if is_vertical_scrollbar_dragged {
                vertical_scroller_color = lighten(base_scroller_color, 0.2);
            }

            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_hovered {
                horizontal_scroller_color = lighten(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_hovered {
                vertical_scroller_color = lighten(base_scroller_color, 0.1);
            }

            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
    }

    Style {
        auto_scroll: AutoScroll {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
            icon: Color::TRANSPARENT,
            shadow: Shadow::default(),
        },
        container: container::Style::default(),
        gap: None,
        horizontal_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: horizontal_scroller_color.into(),

                border: Border {
                    width: 1.5,
                    radius: Radius::from(theme.sizes.border.radius_round),
                    ..Border::default()
                },
            },
        },
        vertical_rail: Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: vertical_scroller_color.into(),
                border: Border {
                    width: 1.5,
                    radius: Radius::from(theme.sizes.border.radius_round),
                    ..Border::default()
                },
            },
        },
    }
}

pub fn pane(theme: &Theme, status: Status) -> Style {
    let base_scroller_color = lighten(theme.palette.surface, 0.05);

    let mut horizontal_scroller_color = base_scroller_color;
    let mut vertical_scroller_color = base_scroller_color;

    match status {
        Status::Active {
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_dragged {
                horizontal_scroller_color = lighten(base_scroller_color, 0.2);
            }

            if is_vertical_scrollbar_dragged {
                vertical_scroller_color = lighten(base_scroller_color, 0.2);
            }

            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => {
            if is_horizontal_scrollbar_hovered {
                horizontal_scroller_color = lighten(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_hovered {
                vertical_scroller_color = lighten(base_scroller_color, 0.1);
            }

            if is_horizontal_scrollbar_disabled {
                horizontal_scroller_color = darken(base_scroller_color, 0.1);
            }

            if is_vertical_scrollbar_disabled {
                vertical_scroller_color = darken(base_scroller_color, 0.1);
            }
        }
    }

    Style {
        auto_scroll: AutoScroll {
            background: Color::TRANSPARENT.into(),
            border: Border::default(),
            icon: Color::TRANSPARENT,
            shadow: Shadow::default(),
        },
        container: container::Style::default(),
        gap: None,
        horizontal_rail: Rail {
            background: Some(theme.palette.surface_raised.into()),
            border: Border::default(),
            scroller: Scroller {
                background: horizontal_scroller_color.into(),

                border: Border {
                    width: 1.5,
                    radius: Radius::from(theme.sizes.border.radius_round),
                    ..Border::default()
                },
            },
        },
        vertical_rail: Rail {
            background: Some(theme.palette.surface_raised.into()),
            border: Border::default(),
            scroller: Scroller {
                background: vertical_scroller_color.into(),
                border: Border {
                    width: 1.5,
                    radius: Radius::from(theme.sizes.border.radius_round),
                    ..Border::default()
                },
            },
        },
    }
}
