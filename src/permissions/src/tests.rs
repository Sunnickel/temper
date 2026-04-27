use crate::default_groups::{ADMIN_GROUP_ID, admin_group};
use crate::group::{GroupID, PermissionGroup, PermissionGroups};
use crate::player::PlayerPermission;
use crate::{Access, Permissions};

fn group(
    id: u32,
    priority: u32,
    permissions: impl IntoIterator<Item = (Permissions, Access)>,
) -> PermissionGroup {
    let mut group = PermissionGroup::new(format!("group-{id}"), GroupID::new(id), priority);
    for (permission, access) in permissions {
        group.add_permission(permission, access);
    }
    group
}

fn groups(permission_groups: impl IntoIterator<Item = PermissionGroup>) -> PermissionGroups {
    let mut groups = PermissionGroups::new();
    for group in permission_groups {
        groups.add_group(group);
    }
    groups
}

#[test]
fn denies_permission_when_player_has_no_matching_permissions() {
    let player = PlayerPermission::new();
    let groups = PermissionGroups::new();

    assert!(!player.can(&groups, Permissions::Teleport));
}

#[test]
fn ignores_permissions_from_groups_the_player_is_not_in() {
    let groups = PermissionGroups::new();
    let player = PlayerPermission::new();

    assert!(!player.can(&groups, Permissions::StopServer));
}

#[test]
fn default_admin_group_grants_all_permissions_when_player_is_in_group() {
    let groups = PermissionGroups::new();
    let mut player = PlayerPermission::new();

    player.add_group(ADMIN_GROUP_ID);

    assert!(player.can(&groups, Permissions::Kill));
}

#[test]
fn allows_permission_from_player_group() {
    let group_id = GroupID::new(2);
    let groups = groups([group(2, 0, [(Permissions::Teleport, Access::Allow)])]);
    let mut player = PlayerPermission::new();

    player.add_group(group_id);

    assert!(player.can(&groups, Permissions::Teleport));
}

#[test]
fn higher_priority_group_overrides_lower_priority_group() {
    let low_priority = GroupID::new(1);
    let high_priority = GroupID::new(2);
    let groups = groups([
        group(2, 20, [(Permissions::Teleport, Access::Deny)]),
        group(1, 10, [(Permissions::Teleport, Access::Allow)]),
    ]);
    let mut player = PlayerPermission::new();

    player.add_group(low_priority);
    player.add_group(high_priority);

    assert!(!player.can(&groups, Permissions::Teleport));
}

#[test]
fn higher_priority_specific_deny_overrides_lower_priority_all_allow() {
    let low_priority = GroupID::new(1);
    let high_priority = GroupID::new(2);
    let groups = groups([
        group(1, 10, [(Permissions::ALL, Access::Allow)]),
        group(2, 20, [(Permissions::Kick, Access::Deny)]),
    ]);
    let mut player = PlayerPermission::new();

    player.add_group(low_priority);
    player.add_group(high_priority);

    assert!(!player.can(&groups, Permissions::Kick));
    assert!(player.can(&groups, Permissions::Ban));
}

#[test]
fn group_specific_deny_overrides_group_all_allow() {
    let group_id = GroupID::new(1);
    let groups = groups([group(
        1,
        10,
        [
            (Permissions::ALL, Access::Allow),
            (Permissions::StopServer, Access::Deny),
        ],
    )]);
    let mut player = PlayerPermission::new();

    player.add_group(group_id);

    assert!(!player.can(&groups, Permissions::StopServer));
    assert!(player.can(&groups, Permissions::Teleport));
}

#[test]
fn player_specific_permission_overrides_group_permissions() {
    let mut groups = PermissionGroups::new();
    groups.add_group(admin_group());
    let mut player = PlayerPermission::new();

    player.add_group(ADMIN_GROUP_ID);
    player.set_permission(Permissions::StopServer, Access::Deny);

    assert!(!player.can(&groups, Permissions::StopServer));
    assert!(player.can(&groups, Permissions::Teleport));
}

#[test]
fn player_all_allow_grants_permissions_unless_specific_player_permission_denies() {
    let groups = PermissionGroups::new();
    let mut player = PlayerPermission::new();

    player.set_permission(Permissions::ALL, Access::Allow);
    player.set_permission(Permissions::Kill, Access::Deny);

    assert!(player.can(&groups, Permissions::Teleport));
    assert!(!player.can(&groups, Permissions::Kill));
}

#[test]
fn denying_all_does_not_deny_all_permissions() {
    let group_id = GroupID::new(1);
    let groups = groups([group(
        1,
        10,
        [
            (Permissions::ALL, Access::Deny),
            (Permissions::Teleport, Access::Allow),
        ],
    )]);
    let mut player = PlayerPermission::new();

    player.add_group(group_id);
    player.set_permission(Permissions::ALL, Access::Deny);

    assert!(player.can(&groups, Permissions::Teleport));
    assert!(!player.can(&groups, Permissions::Ban));
}

#[test]
fn removing_group_or_permission_revokes_access() {
    let group_id = GroupID::new(1);
    let mut player = PlayerPermission::new();

    player.add_group(group_id);
    player.set_permission(Permissions::Ban, Access::Allow);
    let mut groups = groups([group(1, 10, [(Permissions::Teleport, Access::Allow)])]);

    assert!(player.can(&groups, Permissions::Teleport));
    assert!(player.can(&groups, Permissions::Ban));

    groups
        .get_group_mut(&group_id)
        .unwrap()
        .remove_permission(&Permissions::Teleport);
    player.remove_permission(&Permissions::Ban);

    assert!(!player.can(&groups, Permissions::Teleport));
    assert!(!player.can(&groups, Permissions::Ban));
}

#[test]
fn replacing_group_keeps_one_ordering_entry_and_uses_new_priority() {
    let shared_id = GroupID::new(1);
    let high_priority = GroupID::new(2);
    let mut groups = PermissionGroups::new();
    groups.add_group(group(1, 10, [(Permissions::Teleport, Access::Allow)]));
    groups.add_group(group(2, 20, [(Permissions::Teleport, Access::Deny)]));
    groups.add_group(group(1, 30, [(Permissions::Teleport, Access::Allow)]));
    let mut player = PlayerPermission::new();

    player.add_group(shared_id);
    player.add_group(high_priority);

    assert!(player.can(&groups, Permissions::Teleport));

    groups.remove_group(&shared_id);

    assert!(!player.can(&groups, Permissions::Teleport));
}
