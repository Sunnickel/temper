use crate::{Access, PermissionSet, Permissions, default_groups};
use bevy_ecs::prelude::Resource;
use bitcode::{Decode, Encode};
use std::collections::HashMap;
use type_hash::TypeHash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, Default, TypeHash)]
pub struct GroupID(u32);

impl GroupID {
    pub const fn new(id: u32) -> Self {
        GroupID(id)
    }
}

pub struct PermissionGroup {
    pub name: String,
    pub(crate) id: GroupID,
    pub priority: u32,
    pub(crate) permissions: PermissionSet,
}

impl PermissionGroup {
    pub fn new(name: String, id: GroupID, priority: u32) -> Self {
        PermissionGroup {
            name,
            id,
            priority,
            permissions: HashMap::new(),
        }
    }

    pub fn add_permission(&mut self, permission: Permissions, access: Access) {
        self.permissions.insert(permission, access);
    }

    pub fn remove_permission(&mut self, permission: &Permissions) {
        self.permissions.remove(permission);
    }
}

#[derive(Resource)]
pub struct PermissionGroups {
    pub(crate) groups: HashMap<GroupID, PermissionGroup>,
    pub(crate) ordered: Vec<GroupID>,
}

impl Default for PermissionGroups {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionGroups {
    pub fn new() -> Self {
        PermissionGroups {
            groups: HashMap::from([
                (
                    default_groups::DEFAULT_GROUP_ID,
                    default_groups::default_group(),
                ),
                (
                    default_groups::ADMIN_GROUP_ID,
                    default_groups::admin_group(),
                ),
            ]),
            ordered: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group: PermissionGroup) {
        if !self.groups.contains_key(&group.id) {
            self.ordered.push(group.id);
        }
        self.groups.insert(group.id, group);
        self.ordered
            .sort_by_key(|id| self.groups.get(id).unwrap().priority);
    }

    pub fn remove_group(&mut self, id: &GroupID) {
        self.groups.remove(id);
        self.ordered.retain(|group_id| group_id != id);
    }

    pub fn get_group(&self, id: &GroupID) -> Option<&PermissionGroup> {
        self.groups.get(id)
    }

    pub fn get_group_mut(&mut self, id: &GroupID) -> Option<&mut PermissionGroup> {
        self.groups.get_mut(id)
    }
}
