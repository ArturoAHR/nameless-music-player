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
        vec![],
    ))
    .height(372.0)
    .width(Length::Fill)
    .into()
}

pub fn search_condition_group_form<'a>(
    theme: &Theme,
    search_condition_group: &SearchConditionGroup,
    tag_options: &'a [PickListOption<TagId>],
    index_path: Vec<usize>,
) -> Element<'a, Message, Theme, Renderer> {
    let mut search_condition_group_control_row_elements = vec![
        button(text("+ Condition")).into(),
        button(text("+ Group")).into(),
    ];

    if !index_path.is_empty() {
        search_condition_group_control_row_elements.push(button(icon(icons::CLOSE)).into());
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
    index_path: Vec<usize>,
) -> Element<'a, Message, Theme, Renderer> {
    let index_path: Arc<[usize]> = Arc::from(index_path);

    let statement_form = match search_condition_statement {
        SearchConditionStatement::HasTag { tag_id }
        | SearchConditionStatement::DoesNotHaveTag { tag_id } => {
            let index_path = index_path.clone();

            pick_list(
                tag_options,
                tag_id.and_then(|tag_id| {
                    tag_options
                        .iter()
                        .find(|tag_option| tag_option.value == tag_id)
                }),
                move |tag_option| {
                    Message::SelectConditionStatementTag(tag_option.value, index_path.clone())
                },
            )
        }
    };

    container(row![
        pick_list(
            SearchConditionStatementKind::VARIANTS,
            Some(search_condition_statement.kind()),
            move |search_condition_statement_kind| {
                Message::SelectConditionStatement(
                    search_condition_statement_kind,
                    index_path.clone(),
                )
            },
        ),
        statement_form
    ])
    .into()
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
