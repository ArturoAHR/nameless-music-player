use std::{iter::once, sync::Arc};

use iced::{
    Element, Length, Renderer,
    widget::{Space, button, column, container, pick_list, right, row, text},
};
use strum::VariantArray;

use crate::{
    search::models::{
        SearchCondition, SearchConditionGroup, SearchConditionStatement,
        SearchConditionStatementKind,
    },
    tag::models::TagId,
    ui::{
        modals::advanced_search::{Message, PickListOption},
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
    tag_options: &'a [PickListOption<TagId>],
) -> Element<'a, Message, Theme, Renderer> {
    container(search_condition_group_form(
        theme,
        criteria,
        tag_options,
        Arc::from([]),
    ))
    .height(372.0)
    .width(Length::Fill)
    .into()
}

pub fn search_condition_group_form<'a>(
    theme: &Theme,
    search_condition_group: &SearchConditionGroup,
    tag_options: &'a [PickListOption<TagId>],
    index_path: Arc<[usize]>,
) -> Element<'a, Message, Theme, Renderer> {
    let mut search_condition_group_control_row_elements: Vec<
        Element<'a, Message, Theme, Renderer>,
    > = vec![
        button(text("+ Condition"))
            .on_press(Message::AddCondition(Arc::clone(&index_path)))
            .into(),
        button(text("+ Group"))
            .on_press(Message::AddSubgroup(Arc::clone(&index_path)))
            .into(),
    ];

    if !index_path.is_empty() {
        search_condition_group_control_row_elements.push(
            button(icon(icons::CLOSE))
                .on_press(Message::RemoveGroup(Arc::clone(&index_path)))
                .into(),
        );
    }

    let mut search_condition_group_form_rows =
        vec![row(search_condition_group_control_row_elements).into()];

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

    container(column(search_condition_group_form_rows)).into()
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
        }
    };

    let remove_button =
        button(icon(icons::CLOSE)).on_press(Message::RemoveStatement(Arc::clone(&index_path)));

    container(row![
        pick_list(
            SearchConditionStatementKind::VARIANTS,
            Some(search_condition_statement.kind()),
            move |search_condition_statement_kind| {
                Message::SelectConditionStatement(
                    search_condition_statement_kind,
                    Arc::clone(&index_path),
                )
            },
        ),
        statement_form,
        remove_button,
    ])
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
