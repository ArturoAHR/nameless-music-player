use sea_query::{Expr, ExprTrait, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::SqlitePool;
use tracing::instrument;

use crate::{
    error::AppError,
    tag::models::{TagId, TrackTagIden},
    track::models::TrackId,
};

#[instrument(skip(pool))]
pub async fn delete_track_tag(
    pool: SqlitePool,
    track_id: TrackId,
    tag_id: TagId,
) -> Result<(), AppError> {
    let (sql, values) = Query::delete()
        .from_table(TrackTagIden::Table)
        .and_where(Expr::col(TrackTagIden::TrackId).eq(track_id))
        .and_where(Expr::col(TrackTagIden::TagId).eq(tag_id))
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    Ok(())
}
