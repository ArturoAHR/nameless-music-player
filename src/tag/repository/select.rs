use sea_query::{Asterisk, Expr, ExprTrait, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;

use crate::{
    error::AppError,
    tag::models::{Tag, TagGroup, TagGroupIden, TagIden, TrackTag, TrackTagIden},
};

pub async fn get_tags(pool: SqlitePool) -> Result<Vec<Tag>, AppError> {
    let (sql, values) = Query::select()
        .column(Asterisk)
        .from(TagIden::Table)
        .and_where(Expr::col(TagIden::DeletedAt).is_null())
        .build_sqlx(SqliteQueryBuilder);

    let tags = sqlx::query_as_with::<_, Tag, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(tags)
}

pub async fn get_tag_groups(pool: SqlitePool) -> Result<Vec<TagGroup>, AppError> {
    let (sql, values) = Query::select()
        .column(Asterisk)
        .from(TagGroupIden::Table)
        .and_where(Expr::col(TagGroupIden::DeletedAt).is_null())
        .build_sqlx(SqliteQueryBuilder);

    let tag_groups = sqlx::query_as_with::<_, TagGroup, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(tag_groups)
}

pub async fn get_track_tags(pool: SqlitePool) -> Result<Vec<TrackTag>, AppError> {
    let (sql, values) = Query::select()
        .column(Asterisk)
        .from(TrackTagIden::Table)
        .build_sqlx(SqliteQueryBuilder);

    let track_tags = sqlx::query_as_with::<_, TrackTag, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(track_tags)
}
