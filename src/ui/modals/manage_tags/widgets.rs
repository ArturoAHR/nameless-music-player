use iced::{
    Element, Length, Padding, Renderer, alignment,
    widget::{Space, button, center, column, container, right, row, scrollable, text, text_input},
};
use iced_palace::widget::ellipsized_text;

use crate::{
    tag::models::{Tag, TagGroup},
    ui::{
        modals::manage_tags::Message,
        theme::{Theme, catalog},
        widgets::{
            icons::{self, icon},
            separator::vertical_separator,
        },
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

pub fn tag_group_pane<'a>(
    theme: &Theme,
    tag_groups: &'a [TagGroup],
    new_tag_group_name: &str,
) -> Element<'a, Message, Theme, Renderer> {
    column![
        tag_group_list(theme, tag_groups),
        vertical_separator(),
        tag_group_input(theme, new_tag_group_name)
    ]
    .height(Length::Fill)
    .width(Length::FillPortion(1))
    .into()
}

pub fn tag_group_list<'a>(
    theme: &Theme,
    tag_groups: &'a [TagGroup],
) -> Element<'a, Message, Theme, Renderer> {
    let tag_group_list_elements: Vec<Element<'a, Message, Theme, Renderer>> = tag_groups
        .iter()
        .map(|tag_group| {
            button(row![
                ellipsized_text(&tag_group.name),
                Space::new().width(Length::Fill),
                button(icon(icons::CLOSE).size(theme.sizes.font.caption))
                    .on_press(Message::RemoveTagGroup(tag_group.id))
                    .padding(Padding::from([theme.sizes.space.sm, theme.sizes.space.md]))
                    .style(catalog::button::clear_icon_button)
            ])
            .width(Length::Fill)
            .padding(Padding::from(theme.sizes.space.lg))
            .on_press(Message::SelectTagGroup(tag_group.id))
            .style(catalog::button::menu_option)
            .into()
        })
        .collect();

    column![
        text("TAG GROUPS")
            .size(theme.sizes.font.body)
            .color(theme.palette.text_muted),
        scrollable(column(tag_group_list_elements))
    ]
    .spacing(theme.sizes.space.lg)
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.xl, theme.sizes.space.xxl]))
    .into()
}

pub fn tag_group_input<'a>(
    theme: &Theme,
    new_tag_group_name: &str,
) -> Element<'a, Message, Theme, Renderer> {
    container(
        column![
            text("New group")
                .size(theme.sizes.font.small)
                .color(theme.palette.text_muted),
            text_input("Group name", new_tag_group_name)
                .on_input(Message::NewTagGroupNameInputTextChanged)
                .on_submit(Message::AddNewTagGroup)
                .padding(Padding::from(theme.sizes.space.lg))
        ]
        .spacing(theme.sizes.space.md),
    )
    .height(104.0)
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.xl, theme.sizes.space.xxl]))
    .into()
}

pub fn tag_group_tags_pane<'a>(
    theme: &Theme,
    tag_group: Option<&'a TagGroup>,
    tag_group_tags: impl Iterator<Item = &'a Tag>,
    new_tag_name: &str,
) -> Element<'a, Message, Theme, Renderer> {
    let Some(tag_group) = tag_group else {
        return center(
            column![
                text("No tags to display").size(theme.sizes.font.h2),
                text("Create or select a tag group to manage its tags")
                    .size(theme.sizes.font.small)
                    .color(theme.palette.text_muted)
            ]
            .spacing(theme.sizes.space.lg)
            .align_x(alignment::Horizontal::Center),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(2))
        .into();
    };

    column![
        tag_group_tags_list(theme, tag_group, tag_group_tags),
        vertical_separator(),
        tag_groups_tags_input(theme, new_tag_name)
    ]
    .width(Length::FillPortion(2))
    .padding(Padding::from([theme.sizes.space.xl, theme.sizes.space.xxl]))
    .into()
}

pub fn tag_group_tags_list<'a>(
    theme: &Theme,
    tag_group: &'a TagGroup,
    tag_group_tags: impl Iterator<Item = &'a Tag>,
) -> Element<'a, Message, Theme, Renderer> {
    let tag_group_tags_elements: Vec<Element<'a, Message, Theme, Renderer>> = tag_group_tags
        .map(|tag| {
            button(
                row![
                    text(&tag.name),
                    button(icon(icons::CLOSE).size(theme.sizes.font.caption))
                        .on_press(Message::RemoveTag(tag.id))
                        .padding(Padding::from([theme.sizes.space.sm, theme.sizes.space.md]))
                        .style(catalog::button::clear_icon_button)
                ]
                .spacing(theme.sizes.space.md),
            )
            .padding(Padding::from(theme.sizes.space.md))
            .into()
        })
        .collect();

    let list_title = &tag_group.name;
    let list_subtitle = format!("{} / 36 tags", tag_group_tags_elements.len());

    column![
        column![
            text(list_title),
            text(list_subtitle)
                .size(theme.sizes.font.small)
                .color(theme.palette.text_muted)
        ]
        .spacing(theme.sizes.space.md),
        row(tag_group_tags_elements)
            .spacing(theme.sizes.space.md)
            .wrap()
    ]
    .spacing(theme.sizes.space.lg)
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

pub fn tag_groups_tags_input<'a>(
    theme: &Theme,
    new_tag_name: &str,
) -> Element<'a, Message, Theme, Renderer> {
    container(
        column![
            text("New tag")
                .size(theme.sizes.font.small)
                .color(theme.palette.text_muted),
            text_input("Tag name", new_tag_name)
                .on_input(Message::NewTagNameInputTextChanged)
                .on_submit(Message::AddNewTag)
                .padding(Padding::from(theme.sizes.space.lg))
        ]
        .spacing(theme.sizes.space.md),
    )
    .padding(Padding::default().top(theme.sizes.space.xl))
    .height(88.0)
    .width(Length::Fill)
    .into()
}

pub fn footer<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    right(
        button(text("Done"))
            .on_press(Message::Close)
            .padding(Padding::from([
                theme.sizes.space.xl,
                theme.sizes.space.xxxl,
            ]))
            .style(catalog::button::modal_footer_button),
    )
    .align_y(alignment::Vertical::Center)
    .height(84.0)
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.xl, theme.sizes.space.xxl]))
    .style(catalog::container::modal_footer)
    .into()
}
