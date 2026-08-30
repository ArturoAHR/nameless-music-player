use std::{iter::once, sync::Arc};

use iced::{
    Element, Length, Padding, Renderer, alignment,
    widget::{Space, button, column, container, pick_list, right, row, scrollable, text},
};
use strum::VariantArray;

use crate::{
    search::models::{
        SearchCondition, SearchConditionGroup, SearchConditionGroupOperator,
        SearchConditionStatement, SearchConditionStatementKind,
    },
    tag::models::TagId,
    ui::{
        modals::advanced_search::{Message, PickListOption},
        theme::{Theme, catalog},
        widgets::{
            icons::{self, icon},
            separator::horizontal_separator,
        },
    },
};

pub fn header<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    container(
        row![
            text("Advanced search").size(theme.sizes.font.h2),
            Space::new().width(Length::Fill),
            button(icon(icons::CLOSE))
                .on_press(Message::Close)
                .style(catalog::button::clear_icon_button)
        ]
        .align_y(alignment::Vertical::Center),
    )
    .padding([theme.sizes.space.xl, theme.sizes.space.xxl])
    .width(Length::Fill)
    .style(catalog::container::modal_header)
    .into()
}

pub fn body<'a>(
    theme: &Theme,
    criteria: &SearchConditionGroup,
    tag_options: &'a [PickListOption<TagId>],
) -> Element<'a, Message, Theme, Renderer> {
    container(scrollable(row![
        search_condition_group_form(theme, criteria, tag_options, Arc::from([]),),
        Space::new().width(theme.sizes.space.lg)
    ]))
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(Padding::from(theme.sizes.space.xxl).right(theme.sizes.space.lg))
    .into()
}

pub fn search_condition_group_form<'a>(
    theme: &Theme,
    search_condition_group: &SearchConditionGroup,
    tag_options: &'a [PickListOption<TagId>],
    index_path: Arc<[usize]>,
) -> Element<'a, Message, Theme, Renderer> {
    let search_condition_group_operator_pick_list = {
        let index_path = Arc::clone(&index_path);

        pick_list(
            SearchConditionGroupOperator::VARIANTS,
            Some(search_condition_group.operator),
            move |search_condition_group_operator| {
                Message::SelectGroupOperator(
                    search_condition_group_operator,
                    Arc::clone(&index_path),
                )
            },
        )
        .padding([theme.sizes.space.md, theme.sizes.space.lg])
        .into()
    };

    let mut search_condition_group_control_row_elements: Vec<
        Element<'a, Message, Theme, Renderer>,
    > = vec![
        search_condition_group_operator_pick_list,
        button(text("+ Condition"))
            .on_press(Message::AddCondition(Arc::clone(&index_path)))
            .padding([theme.sizes.space.md, theme.sizes.space.lg])
            .style(catalog::button::outline)
            .into(),
        button(text("+ Group"))
            .on_press(Message::AddSubgroup(Arc::clone(&index_path)))
            .padding([theme.sizes.space.md, theme.sizes.space.lg])
            .style(catalog::button::outline)
            .into(),
    ];

    if !index_path.is_empty() {
        search_condition_group_control_row_elements.push(
            button(icon(icons::CLOSE))
                .on_press(Message::RemoveGroup(Arc::clone(&index_path)))
                .style(catalog::button::clear_icon_button)
                .into(),
        );
    }

    let mut search_condition_group_form_rows = vec![
        row(search_condition_group_control_row_elements)
            .spacing(theme.sizes.space.lg)
            .align_y(alignment::Vertical::Center)
            .into(),
    ];

    for (index, search_condition) in search_condition_group.conditions.iter().enumerate() {
        let search_condition_index_path = index_path.iter().copied().chain(once(index)).collect();

        search_condition_group_form_rows.push(match search_condition {
            SearchCondition::Group(search_condition_group) => search_condition_group_form(
                theme,
                search_condition_group,
                tag_options,
                search_condition_index_path,
            ),
            SearchCondition::Statement(search_condition_statement) => {
                search_condition_statement_form(
                    theme,
                    search_condition_statement,
                    tag_options,
                    search_condition_index_path,
                )
            }
        });
    }

    let search_condition_group_form =
        column(search_condition_group_form_rows).spacing(theme.sizes.space.xl);

    if index_path.is_empty() {
        return container(search_condition_group_form).into();
    }

    container(
        row![horizontal_separator(), search_condition_group_form]
            .spacing(theme.sizes.space.lg)
            .padding(Padding::default().left(theme.sizes.space.xxl)),
    )
    .height(Length::Shrink)
    .into()
}

pub fn search_condition_statement_form<'a>(
    theme: &Theme,
    search_condition_statement: &SearchConditionStatement,
    tag_options: &'a [PickListOption<TagId>],
    index_path: Arc<[usize]>,
) -> Element<'a, Message, Theme, Renderer> {
    let statement_form = match search_condition_statement {
        SearchConditionStatement::HasTag { tag_id }
        | SearchConditionStatement::DoesNotHaveTag { tag_id } => {
            let index_path = Arc::clone(&index_path);

            let menu_height = (tag_options.len() * 36).min(290);

            dbg!(&menu_height);

            pick_list(
                tag_options,
                tag_id.and_then(|tag_id| {
                    tag_options
                        .iter()
                        .find(|tag_option| tag_option.value == tag_id)
                }),
                move |tag_option| {
                    Message::SelectConditionStatementTag(tag_option.value, Arc::clone(&index_path))
                },
            )
            .width(Length::Fill)
            .menu_height(menu_height as f32)
            .padding([theme.sizes.space.md, theme.sizes.space.lg])
        }
    };

    let remove_button = button(icon(icons::CLOSE))
        .on_press(Message::RemoveStatement(Arc::clone(&index_path)))
        .style(catalog::button::clear_icon_button);

    container(
        row![
            pick_list(
                SearchConditionStatementKind::VARIANTS,
                Some(search_condition_statement.kind()),
                move |search_condition_statement_kind| {
                    Message::SelectConditionStatement(
                        search_condition_statement_kind,
                        Arc::clone(&index_path),
                    )
                },
            )
            .width(216)
            .padding([theme.sizes.space.md, theme.sizes.space.lg]),
            statement_form,
            remove_button,
        ]
        .align_y(alignment::Vertical::Center)
        .spacing(theme.sizes.space.lg),
    )
    .into()
}

pub fn footer<'a>(theme: &Theme) -> Element<'a, Message, Theme, Renderer> {
    container(right(
        row![
            button("Cancel")
                .on_press(Message::Close)
                .style(catalog::button::modal_footer_button),
            button("Search")
                .on_press(Message::Search)
                .style(catalog::button::modal_footer_button)
        ]
        .spacing(theme.sizes.space.md),
    ))
    .padding([theme.sizes.space.lg, theme.sizes.space.xxl])
    .into()
}
