use iced::{
    Element, Length, Padding, Renderer, alignment,
    widget::{Space, button, column, container, row, text},
};
use iced_palace::widget::ellipsized_text;

use crate::{
    app::TrackList,
    tag::{
        index::TrackTagIndex,
        models::{Tag, TagGroup},
    },
    ui::{
        components::explorer_pane::Message,
        theme::{Theme, catalog},
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
    .padding(Padding::from(theme.sizes.space.xl).bottom(10))
    .into()
}

pub fn main_library_button<'a>(
    theme: &Theme,
    track_list: &TrackList,
) -> Element<'a, Message, Theme, Renderer> {
    button(
        row![
            icon(icons::MUSICAL_NOTE)
                .height(16)
                .align_y(alignment::Vertical::Center),
            ellipsized_text("Music")
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .on_press(Message::SelectedMainLibrary)
    .height(36)
    .width(Length::Fill)
    .padding([theme.sizes.space.md, theme.sizes.space.xl])
    .style(if matches!(track_list, TrackList::MainLibrary) {
        catalog::button::active_explorer_pane_option
    } else {
        catalog::button::explorer_pane_option
    })
    .into()
}

pub fn tag_group_dropdown<'a>(
    theme: &Theme,
    tag_group: &TagGroup,
    tag_group_tags: &[&'a Tag],
    track_list: &TrackList,
    track_tag_index: &TrackTagIndex,
    is_tag_group_expanded: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let mut dropdown_elements: Vec<Element<'a, Message, Theme, Renderer>> =
        vec![tag_group_dropdown_controller(
            theme,
            tag_group,
            tag_group_tags,
            is_tag_group_expanded,
        )];

    if is_tag_group_expanded {
        dropdown_elements.extend(tag_group_tags.iter().map(|tag_group_tag| {
            tag_group_dropdown_option(theme, tag_group_tag, track_list, track_tag_index)
        }));
    }

    column(dropdown_elements).into()
}

pub fn tag_group_dropdown_controller<'a>(
    theme: &Theme,
    tag_group: &TagGroup,
    tag_group_tags: &[&Tag],
    is_tag_group_expanded: bool,
) -> Element<'a, Message, Theme, Renderer> {
    let on_press = (!tag_group_tags.is_empty()).then_some(Message::ToggleTagGroup(tag_group.id));

    let chevron = if is_tag_group_expanded {
        icons::CHEVRON_DOWN
    } else {
        icons::CHEVRON_UP
    };

    button(
        row![
            icon(chevron),
            ellipsized_text(tag_group.name.to_uppercase()).size(theme.sizes.font.body)
        ]
        .width(Length::Fill)
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .width(Length::Fill)
    .padding(Padding::from(theme.sizes.space.xl).bottom(theme.sizes.space.sm))
    .on_press_maybe(on_press)
    .style(catalog::button::explorer_pane_dropdown_controller)
    .into()
}

pub fn tag_group_dropdown_option<'a>(
    theme: &Theme,
    tag_group_tag: &'a Tag,
    track_list: &TrackList,
    track_tag_index: &TrackTagIndex,
) -> Element<'a, Message, Theme, Renderer> {
    let on_press = track_tag_index
        .get_tag_tracks(tag_group_tag.id)
        .is_some_and(|tag_tracks| !tag_tracks.is_empty())
        .then_some(Message::SelectedTag(tag_group_tag.id));

    button(
        row![
            icon(icons::TAG)
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
    .on_press_maybe(on_press)
    .style(
        if let TrackList::Tag(tag_id) = track_list
            && tag_id == &tag_group_tag.id
        {
            catalog::button::active_explorer_pane_option
        } else {
            catalog::button::explorer_pane_option
        },
    )
    .into()
}
