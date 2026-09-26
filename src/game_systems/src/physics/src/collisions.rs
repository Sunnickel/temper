use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Has, Query, Res, With};
use bevy_ecs::world::Mut;
use bevy_math::Vec3A;
use bevy_math::bounding::{Aabb3d, BoundingVolume, IntersectsVolume};
use temper_components::bounds::CollisionBounds;
use temper_components::player::grounded::OnGround;
use temper_components::player::position::Position;
use temper_core::block_properties;
use temper_core::dimension::Dimension;
use temper_core::pos::BlockPos;
use temper_entities::PhysicalRegistry;
use temper_entities::components::Baby;
use temper_entities::components::EntityMetadata;
use temper_entities::markers::HasCollisions;
use temper_messages::entity_update::SendEntityUpdate;
use temper_state::{GlobalState, GlobalStateResource};
use tracing::debug;
use temper_components::player::old_position::OldPosition;

type CollisionQueryItem<'a> = (
    Entity,
    &'a OldPosition,
    Mut<'a, Position>,
    Option<&'a EntityMetadata>,
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
    for (eid, mut old_pos, mut pos, metadata, collision_bounds, is_baby, mut grounded) in query {
        if pos.is_changed() {
            let static_hitbox = if let Some(bounds) = collision_bounds {
                bounds
            } else {
                if let Some(meta) = metadata
                    && let Some(physical) = registry.get_or_adult(meta.protocol_id(), is_baby)
                {
                    &physical.bounding_box
                } else {
                    debug!(
                        "Entity {} has no collision bounds and no physical definition, skipping collision check",
                        eid
                    );
                    continue;
                }
            };

            // Velocity has already been applied, so we subtract current velocity to get the
            // position before velocity was applied
            let delta = (**pos - **old_pos).as_vec3a();
            
            let next_hitbox = static_hitbox.translated_by(pos.as_vec3a() + delta);
            let current_hitbox = static_hitbox.translated_by(pos.as_vec3a());

            if next_hitbox == current_hitbox {
                continue;
            }

            // Any block we could hit has to be in here
            let max_hitbox = next_hitbox.merge(&current_hitbox);

            let mut possible_hits = vec![];

            for x in max_hitbox.min.x.floor() as i32..max_hitbox.max.x.ceil() as i32 {
                for y in max_hitbox.min.y.floor() as i32..max_hitbox.max.y.ceil() as i32 {
                    for z in max_hitbox.min.z.floor() as i32..max_hitbox.max.z.ceil() as i32 {
                        let block_pos = BlockPos::of(x, y, z);
                        if is_solid_block(&state.0, block_pos) {
                            possible_hits.push(block_pos);
                        }
                    }
                }
            }

            // Sub-step the entity's own hitbox between `current_hitbox` and `next_hitbox`
            // so that extreme velocity can't tunnel through thin geometry. We pick a step
            // count so that no single step moves the hitbox more than its smallest
            // dimension along the direction of travel.
            let hitbox_size = static_hitbox.half_size() * 2.0;
            let min_dimension = hitbox_size
                .x
                .min(hitbox_size.y)
                .min(hitbox_size.z)
                .max(f32::EPSILON);
            let travel_distance = delta.length();
            let step_count = (travel_distance / min_dimension).ceil().max(1.0) as u32;

            let start_pos = pos.as_vec3a() - delta;
            let end_pos = pos.as_vec3a();

            let entity_hitboxes: Vec<_> = (0..=step_count)
                .map(|step| {
                    let t = step as f32 / step_count as f32;
                    let sample_pos = start_pos.lerp(end_pos, t);
                    static_hitbox.translated_by(sample_pos)
                })
                .collect();

            // Find the first block that a hitbox intersects

            let mut center_at_hit = None;
            let mut hit_block = None;

            'hitboxes: for hitbox in entity_hitboxes {
                for block_pos in &possible_hits {
                    let block_hitbox = Aabb3d {
                        min: block_pos.pos.as_vec3a(),
                        max: block_pos.pos.as_vec3a() + (1.0 - Vec3A::splat(f32::EPSILON)),
                    };
                    if hitbox.intersects(&block_hitbox) {
                        center_at_hit = Some(hitbox.center());
                        hit_block = Some(*block_pos);
                        break 'hitboxes;
                    }
                }
            }
            if let Some(collided_block) = hit_block {
                debug!("Hit block at {}", collided_block)
            }

            writer.write(SendEntityUpdate(eid));
        }
    }
}

pub fn is_solid_block(state: &GlobalState, pos: BlockPos) -> bool {
    let chunk_coordinates = pos.chunk();
    let block_state = state
        .world
        .get_or_generate_mut(chunk_coordinates, Dimension::Overworld)
        .expect("Failed to load or generate chunk")
        .get_block(pos.chunk_block_pos());

    block_properties::is_solid(block_state)
}
