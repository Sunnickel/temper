#![expect(clippy::type_complexity)]

use bevy_ecs::prelude::{Entity, MessageWriter, Query};
use temper_commands::arg::entities::EntityArgument;
use temper_commands::Sender;
use temper_components::entity_identity::Identity;
use temper_components::player::player_marker::PlayerMarker;
use temper_macros::command;
use temper_messages::destroy_entity::DestroyEntity;

#[command("kill")]
fn kill_command(
    #[sender] sender: Sender,
    #[arg] entity_argument: EntityArgument,
    args: (
        Query<(Entity, &Identity, Option<&PlayerMarker>)>,
        MessageWriter<DestroyEntity>,
    ),
) {
    let (query, mut writer) = args;

    let selected_entities = entity_argument.resolve(query.iter());

    selected_entities.iter().for_each(|e| {
        writer.write(DestroyEntity(*e));
    });

    sender.send_message(
        format!(
            "Killed {} entities (excluding players).",
            selected_entities.len()
        )
        .into(),
        false,
    );
}
