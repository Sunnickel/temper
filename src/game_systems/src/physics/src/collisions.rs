use std::ops::Deref;
use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Has, Query, Res, With};
use bevy_ecs::world::Mut;
use bevy_math::IVec3;
use bevy_math::bounding::{Aabb3d, BoundingVolume};
use temper_components::bounds::CollisionBounds;
use temper_components::player::grounded::OnGround;
use temper_components::player::position::Position;
use temper_components::player::velocity::Velocity;
use temper_core::block_properties;
use temper_core::dimension::Dimension;
use temper_core::pos::{ChunkBlockPos, ChunkPos};
use temper_entities::PhysicalRegistry;
use temper_entities::components::Baby;
use temper_entities::components::EntityMetadata;
use temper_entities::markers::HasCollisions;
use temper_messages::entity_update::SendEntityUpdate;
use temper_state::{GlobalState, GlobalStateResource};

type CollisionQueryItem<'a> = (
    Entity,
    Mut<'a, Velocity>,
    Mut<'a, Position>,
    &'a EntityMetadata,
    Option<&'a CollisionBounds>,
    Has<Baby>,
    Mut<'a, OnGround>,
);

pub fn handle(
    query: Query<CollisionQueryItem, With<HasCollisions>>,
    mut writer: MessageWriter<SendEntityUpdate>,
    state: Res<GlobalStateResource>,
    registry: Res<PhysicalRegistry>,
) {
    for (eid, mut vel, mut pos, metadata, collision_bounds, is_baby, mut grounded) in query {
        let Some(physical) = registry.get_or_adult(metadata.protocol_id(), is_baby) else {
            continue;
        };
        if pos.is_changed() || vel.is_changed() {
            let static_hitbox = if collision_bounds.is_some() {
                collision_bounds.unwrap()
            } else {
                &physical.bounding_box
            };
            
            
            writer.write(SendEntityUpdate(eid));
        }
    }
}

pub fn is_solid_block(state: &GlobalState, pos: IVec3) -> bool {
    let chunk_coordinates = ChunkPos::from(pos.as_dvec3());
    let block_state = state
        .world
        .get_or_generate_mut(chunk_coordinates, Dimension::Overworld)
        .expect("Failed to load or generate chunk")
        .get_block(ChunkBlockPos::from(pos));

    block_properties::is_solid(block_state)
}
