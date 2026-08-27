use iced::{
    Element, Length, Renderer,
    widget::{Space, button, column, container, right, row, text},
};

use crate::{
    search::models::SearchConditionGroup,
    ui::{
        modals::advanced_search::Message,
        theme::{Theme, catalog},
        widgets::icons::{self, icon},
    },
};

pub fn header<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    container(row![
        text("Advanced search"),
        Space::new().width(Length::Fill),
        button(icon(icons::CLOSE))
            .on_press(Message::Close)
            .style(catalog::button::clear_icon_button)
    ])
    .padding([theme.sizes.space.xxxl, theme.sizes.space.xxl])
    .height(60.0)
    .width(Length::Fill)
    .into()
}

pub fn body<'a>(
    theme: &Theme,
    criteria: &SearchConditionGroup,
) -> Element<'a, Message, Theme, Renderer> {
    container(search_condition_group_form(theme, criteria))
        .height(372.0)
        .width(Length::Fill)
        .into()
}

pub fn search_condition_group_form<'a>(
    theme: &Theme,
    search_condition_group: &SearchConditionGroup,
) -> Element<'a, Message, Theme, Renderer> {
    container(column![]).into()
}

pub fn footer<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    container(right(
        row![
            button("Cancel").style(catalog::button::modal_footer_button),
            button("Search").style(catalog::button::modal_footer_button)
        ]
        .spacing(theme.sizes.space.md),
    ))
    .padding([theme.sizes.space.lg, theme.sizes.space.xxl])
    .into()
}
