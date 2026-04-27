use bevy_ecs::prelude::{Entity, Query, Res};
use temper_commands::arg::primitive::string::GreedyString;
use temper_commands::Sender;
use temper_components::entity_identity::Identity;
use temper_macros::command;
use temper_permissions::group::PermissionGroups;
use temper_permissions::player::PlayerPermission;
use temper_text::{TextComponent, TextComponentBuilder};

#[command("permissions")]
fn test_command(#[sender] sender: Sender, args: (Query<&PlayerPermission>, Res<PermissionGroups>)) {
    let (permissions_query, permission_groups) = args;

    let Sender::Player(entity) = sender else {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    };

    let player_permissions = match permissions_query.get(entity) {
        Ok(perms) => perms,
        Err(_) => {
            sender.send_message("Could not find your player entity.".into(), false);
            return;
        }
    };

    let mut response = String::new();
    for group_id in &player_permissions.groups {
        if let Some(group) = permission_groups.groups.get(group_id) {
            response.push_str(&format!("Group: {}\n", group.name));
            response.push_str("Permissions:\n");
            for (perm, access) in &group.permissions {
                response.push_str(&format!("- {:?}: {:?}\n", perm, access));
            }
        } else {
            response.push_str(&format!("Group ID {} (not found)\n", group_id));
        }
        response.push('\n');
    }
    response.push_str("Individual Permissions:\n");
    for (perm, access) in &player_permissions.permissions {
        response.push_str(&format!("- {:?}: {:?}\n", perm, access));
    }

    sender.send_message(TextComponentBuilder::new(response).build(), false);
}
