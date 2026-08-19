use iced::{
    Element, Length, Renderer,
    widget::{Space, container},
};

use crate::ui::theme::{Theme, catalog};

pub fn vertical_separator<'a, Message>() -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    container(Space::new())
        .width(Length::Fill)
        .height(1.0)
        .style(catalog::container::separator)
        .into()
}

pub fn horizontal_separator<'a, Message>() -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    container(Space::new())
        .height(Length::Fill)
        .width(1.0)
        .style(catalog::container::separator)
        .into()
}
