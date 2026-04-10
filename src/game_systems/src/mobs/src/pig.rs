use bevy_ecs::prelude::{Added, Commands, MessageReader, Query, Res, With};
use temper_components::combat::CombatProperties;
use temper_components::entity_identity::Identity;
use temper_components::last_synced_position::LastSyncedPosition;
use temper_components::metadata::EntityMetadata;
use temper_components::player::client_information::ClientInformationComponent;
use temper_components::player::grounded::OnGround;
use temper_components::player::player_marker::PlayerMarker;
use temper_components::player::position::Position;
use temper_components::player::rotation::Rotation;
use temper_components::player::velocity::Velocity;
use temper_components::spawn::SpawnProperties;
use temper_config::server_config::get_global_config;
use temper_core::dimension::Dimension::Overworld;
use temper_entities::PigBundle;
use temper_entities::entity_types::EntityType;
use temper_entities::markers::entity_types::Pig;
use temper_macros::get_registry_entry;
use temper_messages::load_chunk_entities::LoadChunkEntities;
use temper_messages::save_chunk_entities::SaveChunkEntities;
use temper_net_runtime::connection::StreamWriter;
use temper_state::GlobalStateResource;
use tracing::debug;

#[expect(unused_variables)]
pub fn tick_pig(query: Query<&Position, With<Pig>>, players: Query<&Position, With<Identity>>) {}

type PigQuery<'a> = (
    &'a Identity,
    &'a EntityMetadata,
    &'a CombatProperties,
    &'a SpawnProperties,
    &'a Position,
    &'a Rotation,
    &'a Velocity,
    &'a OnGround,
    &'a LastSyncedPosition,
);
pub fn save_pig(
    state: Res<GlobalStateResource>,
    query: Query<PigQuery, With<Pig>>,
    mut reader: MessageReader<SaveChunkEntities>,
) {
    for message in reader.read() {
        for (identity, meta, combat, spawn, pos, rot, vel, on_ground, last_synced) in
            query.iter().filter(|p| p.4.chunk() == message.0)
        {
            let bundle = PigBundle {
                identity: identity.clone(),
                metadata: *meta,
                combat: *combat,
                spawn: spawn.clone(),
                position: *pos,
                rotation: *rot,
                velocity: *vel,
                on_ground: *on_ground,
                last_synced_position: *last_synced,
            };
            debug!(
                "Saving pig with UUID {} at chunk {}",
                identity.uuid, message.0
            );
            let chunk = state
                .0
                .world
                .get_or_generate_chunk(message.0, Overworld)
                .expect("Failed to get or generate chunk");
            chunk.entities.insert(
                identity.uuid,
                (
                    EntityType::Pig,
                    bitcode::serialize(&bundle).expect("Failed to serialize pig bundle"),
                ),
            );
            chunk.mark_dirty();
        }
    }
}

pub fn load_pig(
    state: Res<GlobalStateResource>,
    mut cmd: Commands,
    mut reader: MessageReader<LoadChunkEntities>,
    players: Query<(&StreamWriter, &Position, &ClientInformationComponent), With<PlayerMarker>>,
) {
    for message in reader.read() {
        let chunk = state
            .0
            .world
            .get_or_generate_mut(message.0, Overworld)
            .expect("Failed to get or generate chunk");
        for kv in chunk.entities.iter() {
            let (entity_type, data) = kv.value();
            debug!(
                "Loading entity of type {:?} from chunk {}",
                entity_type, message.0
            );
            if *entity_type == EntityType::Pig {
                let bundle: PigBundle =
                    bitcode::deserialize(data).expect("Failed to deserialize pig bundle");
                let spawn_packet = temper_protocol::outgoing::spawn_entity::SpawnEntityPacket::new(
                    bundle.identity.entity_id,
                    bundle.identity.uuid.clone().as_u128(),
                    get_registry_entry!("minecraft:entity_type.entries.minecraft:pig") as i32,
                    &bundle.position,
                    &bundle.rotation,
                );
                for (conn, player_pos, client_info) in players.iter() {
                    let view_distance = client_info
                        .view_distance
                        .min(get_global_config().chunk_render_distance as u8);

                    if player_pos.distance(*bundle.position) < (view_distance as f64 * 16.0) {
                        conn.send_packet_ref(&spawn_packet)
                            .expect("Failed to send pig spawn packet");
                    }
                }
                cmd.spawn((bundle, Pig));
            }
        }
    }
}

pub fn announce_new_spawned_pig(
    query: Query<(&Identity, &Position, &Rotation), Added<Pig>>,
    players: Query<(&StreamWriter, &Position, &ClientInformationComponent), With<PlayerMarker>>,
) {
    for (identity, pos, rot) in query.iter() {
        let spawn_packet = temper_protocol::outgoing::spawn_entity::SpawnEntityPacket::new(
            identity.entity_id,
            identity.uuid.as_u128(),
            get_registry_entry!("minecraft:entity_type.entries.minecraft:pig") as i32,
            pos,
            rot,
        );
        for (conn, player_pos, client_info) in players.iter() {
            let view_distance = client_info
                .view_distance
                .min(get_global_config().chunk_render_distance as u8);

            if player_pos.distance(**pos) < (view_distance as f64 * 16.0) {
                conn.send_packet_ref(&spawn_packet)
                    .expect("Failed to send pig spawn packet");
            }
        }
    }
}
