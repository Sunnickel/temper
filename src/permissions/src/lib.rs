use std::collections::HashMap;

pub mod group;
pub mod player;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Permissions {
    ALL,

    StopServer,
    Teleport,
    Kill,
    Ban,
    Kick,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Access {
    Allow,
    Deny,
}

pub type PermissionSet = HashMap<Permissions, Access>;

pub mod default_groups {
    use super::{Access, Permissions};
    use crate::group::{GroupID, PermissionGroup};

    pub const DEFAULT_GROUP_ID: GroupID = GroupID::new(0);
    pub const ADMIN_GROUP_ID: GroupID = GroupID::new(1);

    pub fn default_group() -> PermissionGroup {
        PermissionGroup::new("Default".to_string(), DEFAULT_GROUP_ID, 0)
    }

    pub fn admin_group() -> PermissionGroup {
        let mut group = PermissionGroup::new("Admin".to_string(), ADMIN_GROUP_ID, u32::MAX);
        group.add_permission(Permissions::ALL, Access::Allow);
        group
    }
}
