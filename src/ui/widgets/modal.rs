use iced::{
    Element, Length, Padding, Renderer,
    widget::{Container, center, container, opaque, stack},
};

use crate::ui::theme::{
    Theme,
    catalog::{self},
};

pub fn modal_context<'a, Message>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    modal_content: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    let mut elements = vec![base.into()];

    if let Some(modal_content) = modal_content {
        elements.push(opaque(
            container(center(opaque(modal_content.into())))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(catalog::container::modal_backdrop),
        ));
    }

    stack(elements).into()
}

pub fn modal_container<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    container(content)
        // Offsets inner containers so they don't overlap modal container border.
        .padding(Padding::from(1.0))
        .style(catalog::container::modal)
}
