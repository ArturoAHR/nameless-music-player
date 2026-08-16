use iced::{
    Element, Length, Padding, Renderer,
    widget::{Space, button, column, container, row, text},
};

use crate::{
    tag::models::TagGroup,
    ui::{
        modals::manage_tags::Message,
        theme::{Theme, catalog},
        widgets::icons::{self, icon},
    },
};

pub fn header<'a>(theme: &Theme, tag_groups: &[TagGroup]) -> Element<'a, Message, Theme, Renderer> {
    let title = "Manage tags";
    let subtitle = match tag_groups.len() {
        0 => "No tag groups".to_owned(),
        1 => "1 tag group".to_owned(),
        tag_groups_len => format!("{tag_groups_len} tag groups"),
    };

    container(row![
        column![
            text(title).size(theme.sizes.font.h2),
            text(subtitle)
                .size(theme.sizes.font.small)
                .color(theme.palette.text_muted),
        ]
        .spacing(theme.sizes.space.lg),
        Space::new().width(Length::Fill),
        button(icon(icons::CLOSE))
            .on_press(Message::Close)
            .style(catalog::button::clear_icon_button)
    ])
    .height(84.0)
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.xl, theme.sizes.space.xxl]))
    .style(catalog::container::modal_header)
    .into()
}
