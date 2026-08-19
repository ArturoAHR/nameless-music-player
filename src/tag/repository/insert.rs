use sea_query::{OnConflict, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;
use tracing::instrument;

use crate::{
    error::AppError,
    tag::models::{TagGroupId, TagGroupIden, TagId, TagIden, TrackTagIden},
    track::models::TrackId,
};

#[instrument(skip(pool))]
pub async fn insert_tag(
    pool: SqlitePool,
    tag_group_id: TagGroupId,
    tag_name: String,
) -> Result<(), AppError> {
    let (sql, values) = Query::insert()
        .columns([TagIden::Name, TagIden::TagGroupId])
        .into_table(TagIden::Table)
        .values([tag_name.into(), tag_group_id.into()])?
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}

pub async fn insert_tag_group(pool: SqlitePool, tag_group_name: String) -> Result<(), AppError> {
    let (sql, values) = Query::insert()
        .columns([TagGroupIden::Name])
        .into_table(TagGroupIden::Table)
        .values([tag_group_name.into()])?
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn insert_track_tag(
    pool: SqlitePool,
    track_id: TrackId,
    tag_id: TagId,
) -> Result<(), AppError> {
    let (sql, values) = Query::insert()
        .columns([TrackTagIden::TrackId, TrackTagIden::TagId])
        .into_table(TrackTagIden::Table)
        .values([track_id.into(), tag_id.into()])?
        .on_conflict(
            OnConflict::columns([TrackTagIden::TagId, TrackTagIden::TrackId])
                .do_nothing()
                .to_owned(),
        )
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}
