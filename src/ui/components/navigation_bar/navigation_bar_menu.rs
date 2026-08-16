use iced::{Element, Length, Padding, Renderer, widget::button};

use crate::ui::{
    components::navigation_bar::Message,
    theme::{Theme, catalog},
    widgets::{
        icons::{self, icon},
        menu::{
            dropdown_menu, dropdown_menu_grouping_option, dropdown_menu_option, dropdown_toggle,
        },
    },
};

pub fn navigation_bar_menu<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    let dropdown = dropdown_toggle(
        theme,
        button(
            icon(icons::MENU)
                .size(theme.sizes.font.h2)
                .color(theme.palette.text),
        )
        .padding(Padding::default())
        .style(catalog::button::clear_icon_button),
        dropdown_menu(
            theme,
            vec![
                dropdown_menu_grouping_option(
                    theme,
                    "File",
                    dropdown_menu(
                        theme,
                        vec![
                            dropdown_menu_option(theme, "Add new files to library", None),
                            dropdown_menu_option(
                                theme,
                                "Scan folder for new files",
                                Some(Message::SelectedScanDirectoryOption),
                            ),
                        ],
                    )
                    .width(220.0)
                    .offset(12.0),
                ),
                dropdown_menu_option(theme, "Edit", None),
                dropdown_menu_option(theme, "View", None),
                dropdown_menu_option(theme, "Controls", None),
                dropdown_menu_option(theme, "Help", None),
            ],
        )
        .width(Length::Fixed(100.0)),
    );

    dropdown.into()
}
