use iced::{
    Element, Length, Padding, Renderer, alignment,
    widget::{Space, button, column, container, row, text},
};
use iced_palace::widget::ellipsized_text;

use crate::{
    app::TrackList,
    tag::models::{Tag, TagGroup},
    ui::components::explorer_pane::Message,
    ui::{
        theme::Theme,
        widgets::{
            icons::{self, icon},
            separator::vertical_separator,
        },
    },
};

pub fn main_library_section<'a>(
    theme: &Theme,
    track_list: &TrackList,
) -> [Element<'a, Message, Theme, Renderer>; 4] {
    [
        library_label(theme),
        main_library_button(theme, track_list),
        Space::new().height(6).into(),
        vertical_separator(),
    ]
}

pub fn library_label<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    container(
        text("LIBRARY")
            .size(theme.sizes.font.body)
            .color(theme.palette.text_muted),
    )
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.xxl, theme.sizes.space.xl]).bottom(10))
    .into()
}

pub fn main_library_button<'a>(
    theme: &Theme,
    track_list: &TrackList,
) -> Element<'a, Message, Theme, Renderer> {
    button(
        row![
            icon(icons::MUSICAL_NOTE)
                .size(theme.sizes.font.body)
                .height(16)
                .align_y(alignment::Vertical::Top),
            ellipsized_text("Music")
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .on_press(Message::SelectedMainLibrary)
    .height(36)
    .width(Length::Fill)
    .padding([theme.sizes.space.md, theme.sizes.space.xl])
    .into()
}

pub fn tag_group_dropdown<'a>(
    theme: &Theme,
    tag_group: &TagGroup,
    tag_group_tags: &Vec<&'a Tag>,
    track_list: &TrackList,
    is_tag_group_expanded: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let mut dropdown_elements: Vec<Element<'a, Message, Theme, Renderer>> =
        vec![tag_group_dropdown_controller(
            theme,
            tag_group,
            is_tag_group_expanded,
        )];

    if is_tag_group_expanded {
        dropdown_elements.extend(
            tag_group_tags
                .iter()
                .map(|tag_group_tag| tag_group_dropdown_option(theme, tag_group_tag, track_list)),
        );
    }

    column(dropdown_elements).into()
}

pub fn tag_group_dropdown_controller<'a>(
    theme: &Theme,
    tag_group: &TagGroup,
    is_tag_group_expanded: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let chevron = if is_tag_group_expanded {
        icons::CHEVRON_DOWN
    } else {
        icons::CHEVRON_UP
    };

    button(
        row![
            icon(chevron)
                .size(theme.sizes.font.caption)
                .color(theme.palette.text_muted),
            ellipsized_text(tag_group.name.to_uppercase())
                .size(theme.sizes.font.body)
                .color(theme.palette.text_muted)
        ]
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .width(Length::Fill)
    .padding(
        Padding::from([theme.sizes.space.xxl, theme.sizes.space.xl]).bottom(theme.sizes.space.sm),
    )
    .on_press(Message::ToggleTagGroup(tag_group.id))
    .into()
}

pub fn tag_group_dropdown_option<'a>(
    theme: &Theme,
    tag_group_tag: &'a Tag,
    track_list: &TrackList,
) -> Element<'a, Message, Theme, Renderer> {
    button(
        row![
            icon(icons::TAG)
                .size(theme.sizes.font.body)
                .height(16)
                .align_y(alignment::Vertical::Top),
            ellipsized_text(&tag_group_tag.name)
        ]
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .height(36)
    .width(Length::Fill)
    .padding(Padding::from([theme.sizes.space.md, 52.0]).right(theme.sizes.space.sm))
    .on_press(Message::SelectedTag(tag_group_tag.id))
    .into()
}
