use sea_query::enum_def;

#[enum_def(table_name = "tag")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tag {
    id: i64,
    name: String,
    tag_group_id: i64,
    created_at: i64,
    updated_at: i64,
    deleted_at: Option<i64>,
}

#[enum_def(table_name = "tag_group")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TagGroup {
    id: i64,
    name: String,
    created_at: i64,
    updated_at: i64,
    deleted_at: Option<i64>,
}

#[enum_def(table_name = "track_tag")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TrackTag {
    id: i64,
    tag_id: i64,
    track_id: i64,
    created_at: i64,
}
