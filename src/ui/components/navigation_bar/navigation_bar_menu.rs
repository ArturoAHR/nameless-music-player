use iced::{Length, Padding, widget::button};

use crate::ui::{
    components::navigation_bar::Message,
    theme::{Theme, catalog},
    widgets::{
        icons::{self, icon},
        menu::{
            DropdownMenu, DropdownMenuToggle, dropdown_menu, dropdown_menu_grouping_option,
            dropdown_menu_option, dropdown_toggle,
        },
    },
};

pub fn main_menu_dropdown<'a>(theme: &Theme) -> DropdownMenuToggle<'a, Message> {
    dropdown_toggle(
        theme,
        button(
            icon(icons::MENU)
                .size(theme.sizes.font.h2)
                .color(theme.palette.text),
        )
        .padding(Padding::default())
        .style(catalog::button::clear_icon_button),
        main_menu(theme),
    )
}

pub fn main_menu<'a>(theme: &Theme) -> DropdownMenu<'a, Message> {
    dropdown_menu(
        theme,
        vec![
            dropdown_menu_grouping_option(theme, "File", file_menu(theme)),
            dropdown_menu_grouping_option(theme, "Edit", edit_menu(theme)),
            // TODO: Add back these options when they are functional
            // dropdown_menu_option(theme, "View", None),
            // dropdown_menu_option(theme, "Controls", None),
            // dropdown_menu_option(theme, "Help", None),
        ],
    )
    .width(Length::Fixed(100.0))
}

pub fn file_menu<'a>(theme: &Theme) -> DropdownMenu<'a, Message> {
    dropdown_menu(
        theme,
        vec![
            // TODO: Add back this option when it is functional
            // dropdown_menu_option(theme, "Add new files to library", None),
            dropdown_menu_option(
                theme,
                "Scan folder for new files",
                Some(Message::SelectedScanDirectoryOption),
            ),
        ],
    )
    .width(220.0)
    .offset(12.0)
}

pub fn edit_menu<'a>(theme: &Theme) -> DropdownMenu<'a, Message> {
    dropdown_menu(
        theme,
        vec![dropdown_menu_option(theme, "Manage Tags", None)],
    )
    .width(160.0)
    .offset(12.0)
}
