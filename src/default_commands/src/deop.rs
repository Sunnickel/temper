use bevy_ecs::prelude::{Entity, Query, Res};
use temper_commands::arg::entities::EntityArgument;
use temper_commands::Sender;
use temper_components::entity_identity::Identity;
use temper_components::player::player_marker::PlayerMarker;
use temper_macros::command;
use temper_permissions::group::PermissionGroups;
use temper_permissions::player::PlayerPermission;
use temper_text::TextComponent;

#[command("deop")]
pub fn deop_command(
    #[arg] target: EntityArgument,
    #[sender] sender: Sender,
    args: (
        Query<(Entity, &Identity, Option<&PlayerMarker>)>,
        Query<&mut PlayerPermission>,
        Res<PermissionGroups>,
    ),
) {
    let (query, mut permissions_query, permission_groups) = args;
    let entities = target.resolve(query.into_iter());

    let is_permitted = match sender {
        Sender::Player(entity) => {
            if let Ok(player_perm) = permissions_query.get(entity) {
                player_perm.can(&permission_groups, temper_permissions::Permissions::DeOp)
            } else {
                false
            }
        }
        _ => true, // Non-player senders are always permitted
    };

    if !is_permitted {
        sender.send_message(
            "You don't have permission to use this command.".into(),
            false,
        );
        return;
    }

    for entity in entities {
        if let Ok(mut player_permission) = permissions_query.get_mut(entity) {
            player_permission.remove_group(&temper_permissions::default_groups::ADMIN_GROUP_ID);
            temper_core::mq::queue(
                TextComponent::from("You have been de-opped".to_string()),
                false,
                entity,
            );
        }
    }
}
