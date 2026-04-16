use bevy_ecs::prelude::{Query, With};
use temper_components::combat::CombatProperties;
use temper_components::entity_identity::Identity;
use temper_components::last_synced_position::LastSyncedPosition;
use temper_components::metadata::EntityMetadata;
use temper_components::player::grounded::OnGround;
use temper_components::player::position::Position;
use temper_components::player::rotation::Rotation;
use temper_components::player::velocity::Velocity;
use temper_components::spawn::SpawnProperties;
use temper_entities::markers::entity_types::Pig;
use temper_entities::PigBundle;

#[expect(unused_variables)]
pub fn tick_pig(query: Query<&Position, With<Pig>>, players: Query<&Position, With<Identity>>) {}

crate::define_entity_save_load!(
    pig,
    marker = Pig,
    bundle = PigBundle,
    entity_type = Pig,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    ),
    fields = {
        identity: Identity => clone,
        metadata: EntityMetadata => copy,
        combat: CombatProperties => copy,
        spawn: SpawnProperties => clone,
        position: Position => copy,
        rotation: Rotation => copy,
        velocity: Velocity => copy,
        on_ground: OnGround => copy,
        last_synced_position: LastSyncedPosition => copy,
    }
);

/*
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
                    EntityTypeEnum::Pig,
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
            if *entity_type == EntityTypeEnum::Pig {
                let bundle: PigBundle =
                    bitcode::deserialize(data).expect("Failed to deserialize pig bundle");
                cmd.spawn((bundle, Pig));
            }
        }
    }
}
*/
