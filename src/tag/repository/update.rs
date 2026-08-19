use sea_query::{Expr, ExprTrait, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;
use tracing::instrument;

use crate::{
    error::AppError,
    tag::models::{TagGroupId, TagGroupIden, TagId, TagIden},
};

#[instrument(skip(pool))]
pub async fn soft_delete_tag(pool: SqlitePool, tag_id: TagId) -> Result<(), AppError> {
    let (sql, values) = Query::update()
        .values([(TagIden::DeletedAt, Expr::cust("unixepoch()"))])
        .table(TagIden::Table)
        .and_where(Expr::col(TagIden::Id).eq(tag_id))
        .and_where(Expr::col(TagIden::DeletedAt).is_null())
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn soft_delete_tag_group(
    pool: SqlitePool,
    tag_group_id: TagGroupId,
) -> Result<(), AppError> {
    let (sql, values) = Query::update()
        .values([(TagGroupIden::DeletedAt, Expr::cust("unixepoch()"))])
        .table(TagGroupIden::Table)
        .and_where(Expr::col(TagGroupIden::Id).eq(tag_group_id))
        .and_where(Expr::col(TagGroupIden::DeletedAt).is_null())
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}
