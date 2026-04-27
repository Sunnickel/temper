use crate::group::{GroupID, PermissionGroups};
use crate::{Access, PermissionSet, Permissions};
use bevy_ecs::prelude::Component;
use bitcode::{Decode, Encode};
use std::collections::{HashMap, HashSet};
use tracing::error;
use type_hash::TypeHash;
use crate::default_groups::default_group;

/// Component representing a player's permissions, including group memberships and individual permissions.
#[derive(Component, Clone, Debug, Encode, Decode, TypeHash)]
pub struct PlayerPermission {
    groups: HashSet<GroupID>,
    permissions: PermissionSet,
}

impl Default for PlayerPermission {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerPermission {
    pub fn new() -> Self {
        PlayerPermission {
            groups: HashSet::from([default_group().id]),
            permissions: HashMap::new(),
        }
    }

    /// Adds a group to the player's set of groups.
    pub fn add_group(&mut self, group_id: GroupID) {
        self.groups.insert(group_id);
    }

    /// Removes a group from the player's set of groups.
    pub fn remove_group(&mut self, group_id: &GroupID) {
        self.groups.remove(group_id);
    }

    /// Adds a permission to the player's individual permissions.
    pub fn set_permission(&mut self, permission: Permissions, access: Access) {
        self.permissions.insert(permission, access);
    }

    /// Removes a permission from the player's individual permissions.
    pub fn remove_permission(&mut self, permission: &Permissions) {
        self.permissions.remove(permission);
    }

    /// Checks if the player has a specific permission, considering both group permissions and
    /// individual permissions. Individual permissions take precedence over group permissions, and
    /// higher priority groups take precedence over lower priority groups. If a group has the `ALL`
    /// permission, it grants all permissions to the player, but denying ALL does not deny all permissions.
    pub fn can(&self, groups: &PermissionGroups, permission: Permissions) -> bool {
        let mut result = None;

        for group_id in &groups.ordered {
            if !self.groups.contains(group_id) {
                continue;
            }

            let Some(group) = groups.groups.get(group_id) else {
                error!("Group with ID {:?} not found in PermissionGroups", group_id);
                continue;
            };

            if let Some(value) = group.permissions.get(&Permissions::ALL)
                && matches!(value, Access::Allow)
            {
                result = Some(value);
            }

            if let Some(value) = group.permissions.get(&permission) {
                result = Some(value);
            }
        }

        if let Some(value) = self.permissions.get(&Permissions::ALL)
            && matches!(value, Access::Allow)
        {
            result = Some(value);
        }

        if let Some(value) = self.permissions.get(&permission) {
            result = Some(value);
        }

        matches!(result, Some(Access::Allow))
    }
}
