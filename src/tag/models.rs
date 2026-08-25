use sea_query::enum_def;

use crate::traits::Identifiable;

#[enum_def(table_name = "tag")]
#[cfg_attr(any(test, feature = "testing"), derive(Default))]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub tag_group_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Identifiable for Tag {
    type Identifier = i64;

    fn id(&self) -> &Self::Identifier {
        &self.id
    }
}

pub type TagId = <Tag as Identifiable>::Identifier;

#[enum_def(table_name = "tag_group")]
#[cfg_attr(any(test, feature = "testing"), derive(Default))]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TagGroup {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Identifiable for TagGroup {
    type Identifier = i64;

    fn id(&self) -> &Self::Identifier {
        &self.id
    }
}

pub type TagGroupId = <TagGroup as Identifiable>::Identifier;

#[enum_def(table_name = "track_tag")]
#[cfg_attr(any(test, feature = "testing"), derive(Default))]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TrackTag {
    pub id: i64,
    pub tag_id: i64,
    pub track_id: i64,
    pub created_at: i64,
}
