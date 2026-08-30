use iced::{
    Element, Length, Renderer, alignment,
    widget::{Button, Space, button, row, text},
};
use iced_aw::{Menu, MenuBar, menu::Item};
use iced_palace::widget::ellipsized_text;

use crate::ui::{
    theme::{Theme, catalog},
    widgets::icons::{self, icon},
};

pub type DropdownMenuToggle<'a, Message> = MenuBar<'a, Message, Theme, Renderer>;
pub type DropdownMenuItem<'a, Message> = Item<'a, Message, Theme, Renderer>;
pub type DropdownMenu<'a, Message> = Menu<'a, Message, Theme, Renderer>;

pub fn dropdown_toggle<'a, Message>(
    _theme: &Theme,
    toggle: impl Into<Element<'a, Message, Theme, Renderer>>,
    menu: DropdownMenu<'a, Message>,
) -> DropdownMenuToggle<'a, Message> {
    MenuBar::new(vec![Item::with_menu(toggle, menu)])
}

pub fn dropdown_menu<'a, Message>(
    _theme: &Theme,
    items: Vec<DropdownMenuItem<'a, Message>>,
) -> DropdownMenu<'a, Message> {
    Menu::new(items).offset(8.0).spacing(2.0)
}

pub fn dropdown_menu_item<'a, Message>(
    _theme: &Theme,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> DropdownMenuItem<'a, Message> {
    Item::new(content)
}

pub fn dropdown_menu_option<'a, Message: Clone + 'a>(
    theme: &Theme,
    text_label: &'static str,
    event: Option<Message>,
) -> DropdownMenuItem<'a, Message> {
    Item::new(
        button(ellipsized_text(text_label).color(theme.palette.text))
            .on_press_maybe(event)
            .width(Length::Fill)
            .style(catalog::button::menu_option),
    )
    .close_on_click(true)
}

pub fn dropdown_submenu<'a, Message>(
    _theme: &Theme,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    submenu: DropdownMenu<'a, Message>,
) -> DropdownMenuItem<'a, Message> {
    Item::with_menu(content, submenu)
}

pub fn dropdown_menu_grouping_option<'a, Message: Clone + 'a>(
    theme: &Theme,
    text_label: &'static str,
    submenu: DropdownMenu<'a, Message>,
) -> DropdownMenuItem<'a, Message> {
    Item::with_menu(
        row![
            button(ellipsized_text(text_label).color(theme.palette.text))
                .width(Length::Fill)
                .style(catalog::button::menu_option),
            Space::new().width(Length::Fill),
            icon(icons::CHEVRON_RIGHT).color(theme.palette.text)
        ]
        .align_y(alignment::Vertical::Center),
        submenu,
    )
}

pub fn menu_option<'a, Message>(
    text_label: &'static str,
    on_press: Option<Message>,
) -> Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    button(text(text_label))
        .on_press_maybe(on_press)
        .width(Length::Fill)
        .style(catalog::button::menu_option)
}
