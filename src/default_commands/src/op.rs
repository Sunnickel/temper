use bevy_ecs::prelude::{Entity, Query};
use temper_commands::arg::entities::EntityArgument;
use temper_commands::Sender;
use temper_components::entity_identity::Identity;
use temper_components::player::player_marker::PlayerMarker;
use temper_macros::command;
use temper_permissions::player::PlayerPermission;

#[command("op")]
fn op_command(
    #[arg] target: EntityArgument,
    #[sender] sender: Sender,
    args: (
        Query<(Entity, &Identity, Option<&PlayerMarker>)>,
        Query<&mut PlayerPermission>,
    ),
) {
    let (query, mut permissions_query) = args;
    let entities = target.resolve(query.into_iter());

    for entity in entities {
        if let Ok(mut player_permission) = permissions_query.get_mut(entity) {
            player_permission.set_permission(
                temper_permissions::Permissions::ALL,
                temper_permissions::Access::Allow,
            );
        }
    }
}
