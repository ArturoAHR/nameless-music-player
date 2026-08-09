use std::str::Chars;

use crate::tag::models::{Tag, TagGroupId};

const TAG_KEYS: &str = "1234567890qwertyuiopasdfghjklzxcvbnm";

pub fn get_tag_index(key: &char) -> Option<usize> {
    TAG_KEYS.chars().position(|character| *key == character)
}

pub fn get_tag_keys() -> Chars<'static> {
    TAG_KEYS.chars()
}

pub fn get_tag_group_tags(tags: &[Tag], tag_group_id: TagGroupId) -> Vec<&Tag> {
    tags.iter()
        .filter(|tag| tag.tag_group_id == tag_group_id)
        .collect()
}
