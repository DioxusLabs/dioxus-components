use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::dashboard::common::{AvatarProfile, AVATAR_PROFILE_OPTIONS};

pub(super) fn avatar_profile_for_key(key: &str) -> &'static AvatarProfile {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let index = (hasher.finish() as usize) % AVATAR_PROFILE_OPTIONS.len();
    &AVATAR_PROFILE_OPTIONS[index]
}
