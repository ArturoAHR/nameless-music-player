use iced::{
    Element, Length, Renderer,
    widget::{center, container, opaque, stack},
};

use crate::ui::theme::{Theme, catalog::container::modal_backdrop};

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
                .style(modal_backdrop),
        ));
    }

    stack(elements).into()
}
