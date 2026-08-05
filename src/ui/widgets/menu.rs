use iced::{
    Element, Length, Renderer,
    widget::{Button, button, text},
};
use iced_aw::{Menu, MenuBar, menu::Item};

use crate::ui::theme::{Theme, catalog};

pub type DropdownMenuToggle<'a, M> = MenuBar<'a, M, Theme, Renderer>;
pub type DropdownMenuItem<'a, M> = Item<'a, M, Theme, Renderer>;
pub type DropdownMenu<'a, M> = Menu<'a, M, Theme, Renderer>;

pub fn dropdown_toggle<'a, M>(
    _theme: &Theme,
    toggle: impl Into<Element<'a, M, Theme, Renderer>>,
    menu: DropdownMenu<'a, M>,
) -> DropdownMenuToggle<'a, M> {
    MenuBar::new(vec![Item::with_menu(toggle, menu)])
}

pub fn dropdown_menu<'a, M>(
    _theme: &Theme,
    items: Vec<DropdownMenuItem<'a, M>>,
) -> DropdownMenu<'a, M> {
    Menu::new(items).max_width(220.0).offset(8.0).spacing(2.0)
}

pub fn dropdown_menu_item<'a, M>(
    _theme: &Theme,
    content: impl Into<Element<'a, M, Theme, Renderer>>,
) -> DropdownMenuItem<'a, M> {
    Item::new(content)
}

pub fn dropdown_menu_option<'a, M: Clone + 'a>(
    _theme: &Theme,
    label: impl Into<Element<'a, M, Theme, Renderer>>,
    event: Option<M>,
) -> DropdownMenuItem<'a, M> {
    Item::new(button(label).on_press_maybe(event)).close_on_click(true)
}

pub fn dropdown_submenu<'a, M>(
    _theme: &Theme,
    content: impl Into<Element<'a, M, Theme, Renderer>>,
    submenu: DropdownMenu<'a, M>,
) -> DropdownMenuItem<'a, M> {
    Item::with_menu(content, submenu)
}

pub fn dropdown_menu_grouping_option<'a, M: Clone + 'a>(
    _theme: &Theme,
    label: impl Into<Element<'a, M, Theme, Renderer>>,
    submenu: DropdownMenu<'a, M>,
) -> DropdownMenuItem<'a, M> {
    Item::with_menu(button(label), submenu)
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
