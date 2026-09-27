use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Has, Query, Res, With};
use bevy_ecs::world::Mut;
use bevy_math::Vec3A;
use bevy_math::bounding::{Aabb3d, BoundingVolume, IntersectsVolume};
use std::time::Instant;
use temper_components::bounds::CollisionBounds;
use temper_components::entity_identity::Identity;
use temper_components::player::grounded::OnGround;
use temper_components::player::old_position::OldPosition;
use temper_components::player::player_marker::PlayerMarker;
use temper_components::player::position::Position;
use temper_components::player::velocity::Velocity;
use temper_core::block_properties;
use temper_core::dimension::Dimension;
use temper_core::pos::BlockPos;
use temper_entities::PhysicalRegistry;
use temper_entities::components::Baby;
use temper_entities::components::EntityMetadata;
use temper_entities::markers::HasCollisions;
use temper_messages::entity_update::SendEntityUpdate;
use temper_messages::particle::SendParticle;
use temper_state::{GlobalState, GlobalStateResource};
use tracing::{debug, error};
use temper_components::player::gamemode::{GameModeComponent, GameMode};

type CollisionQueryItem<'a> = (
    Entity,
    Option<&'a OldPosition>,
    Mut<'a, Position>,
    Option<&'a EntityMetadata>,
    Option<&'a mut Velocity>,
    Option<&'a CollisionBounds>,
    Option<&'a GameModeComponent>,
    Has<Baby>,
    Mut<'a, OnGround>,
    &'a Identity,
    Has<PlayerMarker>,
);

/// This whole thing is a complete mess since players have unreliable and largely unused velocity
/// but do have client-side collision prediction and mobs have velocities but no client-side collisions.
pub fn handle(
    query: Query<CollisionQueryItem, With<HasCollisions>>,
    mut entity_updates_writer: MessageWriter<SendEntityUpdate>,
    state: Res<GlobalStateResource>,
    registry: Res<PhysicalRegistry>,
) {
    for (
        eid,
        old_pos,
        mut pos,
        metadata,
        vel,
        collision_bounds,
        gamemode,
        is_baby,
        mut grounded,
        identity,
        is_player,
    ) in query
    {
        if let Some(gamemode) = gamemode && matches!(gamemode.0, GameMode::Spectator) {
            continue;
        }
        if pos.is_changed() {
            let start = Instant::now();
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

            // This is really odd but for players velocity isn't really a thing so we calculate the
            // delta from their last position. For mobs though there is a velocity component, and
            // velocity has already been applied before figuring out collisions. So we have to do
            // different things for players vs non-players. This is stupid and should be fixed once
            // serverside player physics are implemented properly.

            let delta = if let Some(vel) = &vel
                && !is_player
            {
                ***vel
            } else if let Some(old_pos) = old_pos {
                (**pos - **old_pos).as_vec3a()
            } else {
                error!(
                    "Entity {} has no velocity and no old position, skipping collision check",
                    eid
                );
                continue;
            };

            let (current_hitbox, next_hitbox) = if let Some(vel) = &vel
                && !is_player
            {
                (
                    static_hitbox.translated_by(pos.as_vec3a() - ***vel),
                    static_hitbox.translated_by(pos.as_vec3a()),
                )
            } else if is_player {
                (
                    static_hitbox.translated_by(pos.as_vec3a()),
                    static_hitbox.translated_by(pos.as_vec3a() + delta),
                )
            } else {
                error!(
                    "Entity {} has no velocity and not a player, skipping collision check",
                    eid
                );
                continue;
            };

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

            let samples: Vec<Vec3A> = (0..=step_count)
                .map(|step| {
                    let t = step as f32 / step_count as f32;
                    start_pos.lerp(end_pos, t)
                })
                .collect();

            // Find the first sample whose hitbox intersects a block, keeping track of the
            // last sample we know is still clear so we can resolve back to it below.

            let mut hit_block = None;
            let mut safe_sample = start_pos;
            let mut blocked_sample = None;

            let mut checked_blocks = 0usize;

            'samples: for sample_pos in samples {
                let hitbox = static_hitbox.translated_by(sample_pos);
                for block_pos in &possible_hits {
                    checked_blocks += 1;
                    let block_hitbox = Aabb3d {
                        min: block_pos.pos.as_vec3a(),
                        max: block_pos.pos.as_vec3a() + (1.0 - Vec3A::splat(f32::EPSILON)),
                    };
                    if hitbox.intersects(&block_hitbox) {
                        hit_block = Some(*block_pos);
                        blocked_sample = Some(sample_pos);
                        break 'samples;
                    }
                }
                safe_sample = sample_pos;
            }
            if let Some(collided_block) = hit_block {
                debug!(
                    "{} Hit block at {}",
                    identity.name.as_ref().expect("Entity has no name"),
                    collided_block
                );
                debug!(
                    "{} blocks checked in total for collide, took {:?}",
                    checked_blocks,
                    Instant::now() - start
                );

                // If it's not a player we need to set their position to not be colliding with the block.
                // If we ever get around to doing an anticheat system we can do server-side player
                // collision resolution but for now we just let the client handle it.
                if !is_player {
                    let block_hitbox = Aabb3d {
                        min: collided_block.pos.as_vec3a(),
                        max: collided_block.pos.as_vec3a() + (1.0 - Vec3A::splat(f32::EPSILON)),
                    };

                    let mut safe = safe_sample;
                    let mut blocked =
                        blocked_sample.expect("hit_block implies a blocked sample exists");

                    // Binary search along the travel path between the last known-clear sample
                    // and the one that collided, so we land as close to the block as possible
                    // instead of snapping all the way back to the previous substep.
                    for _ in 0..12 {
                        let mid = safe.lerp(blocked, 0.5);
                        if static_hitbox.translated_by(mid).intersects(&block_hitbox) {
                            blocked = mid;
                        } else {
                            safe = mid;
                        }
                    }

                    pos.coords = safe.as_dvec3();

                    // Zero out velocity on the axis that actually drove us into the block so we
                    // don't just re-collide (and re-correct) again next tick. This assumes the
                    // dominant axis of `delta` is the one that hit, which holds while a substep
                    // only ever moves along one block dimension - it'll need proper per-axis MTV
                    // once we're picking the *closest* block instead of just the first one found.
                    if let Some(mut vel) = vel {
                        let axis = if delta.x.abs() >= delta.y.abs() && delta.x.abs() >= delta.z.abs()
                        {
                            // Hit something on the x-axis
                            0
                        } else if delta.y.abs() >= delta.z.abs() {
                            // Hit something on the y-axis
                            grounded.0 = true;
                            1
                        } else {
                            // Hit on the z-axis
                            2
                        };
                        vel.vec[axis] = 0.0;
                    }
                }
            }

            entity_updates_writer.write(SendEntityUpdate(eid));
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
