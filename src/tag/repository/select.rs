use sea_query::{Asterisk, Expr, ExprTrait, JoinType, Order, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;
use tracing::instrument;

use crate::{
    error::AppError,
    tag::models::{Tag, TagGroup, TagGroupIden, TagIden, TrackTag, TrackTagIden},
};

#[instrument(skip(pool))]
pub async fn get_tags(pool: SqlitePool) -> Result<Vec<Tag>, AppError> {
    let (sql, values) = Query::select()
        .column((TagIden::Table, Asterisk))
        .from(TagIden::Table)
        .join(
            JoinType::InnerJoin,
            TagGroupIden::Table,
            Expr::col((TagIden::Table, TagIden::TagGroupId))
                .equals((TagGroupIden::Table, TagGroupIden::Id)),
        )
        .and_where(Expr::col((TagIden::Table, TagIden::DeletedAt)).is_null())
        .and_where(Expr::col((TagGroupIden::Table, TagGroupIden::DeletedAt)).is_null())
        .order_by((TagGroupIden::Table, TagGroupIden::Name), Order::Asc)
        .order_by((TagIden::Table, TagIden::Name), Order::Asc)
        .build_sqlx(SqliteQueryBuilder);

    let tags = sqlx::query_as_with::<_, Tag, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(tags)
}

#[instrument(skip(pool))]
pub async fn get_tag_groups(pool: SqlitePool) -> Result<Vec<TagGroup>, AppError> {
    let (sql, values) = Query::select()
        .column(Asterisk)
        .from(TagGroupIden::Table)
        .and_where(Expr::col(TagGroupIden::DeletedAt).is_null())
        .order_by(TagGroupIden::Name, Order::Asc)
        .build_sqlx(SqliteQueryBuilder);

    let tag_groups = sqlx::query_as_with::<_, TagGroup, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(tag_groups)
}

#[instrument(skip(pool))]
pub async fn get_track_tags(pool: SqlitePool) -> Result<Vec<TrackTag>, AppError> {
    let (sql, values) = Query::select()
        .column((TrackTagIden::Table, Asterisk))
        .from(TrackTagIden::Table)
        .join(
            JoinType::InnerJoin,
            TagIden::Table,
            Expr::col((TrackTagIden::Table, TrackTagIden::TagId))
                .equals((TagIden::Table, TagIden::Id)),
        )
        .join(
            JoinType::InnerJoin,
            TagGroupIden::Table,
            Expr::col((TagIden::Table, TagIden::TagGroupId))
                .equals((TagGroupIden::Table, TagGroupIden::Id)),
        )
        .and_where(Expr::col((TagIden::Table, TagIden::DeletedAt)).is_null())
        .and_where(Expr::col((TagGroupIden::Table, TagGroupIden::DeletedAt)).is_null())
        .build_sqlx(SqliteQueryBuilder);

    let track_tags = sqlx::query_as_with::<_, TrackTag, _>(&sql, values)
        .fetch_all(&pool)
        .await?;

    Ok(track_tags)
}

#[derive(Debug, Clone)]
pub struct TagLibrary {
    pub tags: Vec<Tag>,
    pub tag_groups: Vec<TagGroup>,
    pub track_tags: Vec<TrackTag>,
}

#[instrument(skip(pool))]
pub async fn load_tag_library(pool: SqlitePool) -> Result<TagLibrary, AppError> {
    let (tags, tag_groups, track_tags) = iced::futures::try_join!(
        get_tags(pool.clone()),
        get_tag_groups(pool.clone()),
        get_track_tags(pool)
    )?;

    Ok(TagLibrary {
        tags,
        tag_groups,
        track_tags,
    })
}
