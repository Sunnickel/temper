use bevy_ecs::message::MessageWriter;
use bevy_ecs::prelude::{DetectChanges, Entity, Has, Query, Res, With};
use bevy_ecs::world::Mut;
use bevy_math::Vec3A;
use bevy_math::bounding::BoundingVolume;
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

            let start_pos = pos.as_vec3a() - delta;

            // Sweep the entity's own hitbox analytically against every candidate block and
            // find the exact time (0..=1 along `start_pos -> pos`) it would first touch each
            // one. Keeping the candidate with the smallest entry time gives us the actual
            // first block hit along the path, rather than whichever one happens to come first
            // in `possible_hits` or gets sampled first by a coarse substep.
            let moving_min0 = start_pos + static_hitbox.min;
            let moving_max0 = start_pos + static_hitbox.max;

            let mut best_hit: Option<(f32, usize, BlockPos)> = None;

            for block_pos in &possible_hits {
                let target_min = block_pos.pos.as_vec3a();
                let target_max = target_min + Vec3A::ONE;

                if let Some((entry_time, axis)) =
                    sweep_aabb(moving_min0, moving_max0, target_min, target_max, delta)
                    && best_hit.is_none_or(|(best_time, _, _)| entry_time < best_time)
                {
                    best_hit = Some((entry_time, axis, *block_pos));
                }
            }

            if let Some((entry_time, axis, collided_block)) = best_hit {
                debug!(
                    "{} Hit block at {}, {} blocks checked, took {:?}",
                    identity.name.as_ref().expect("Entity has no name"),
                    collided_block,
                    possible_hits.len(),
                    Instant::now() - start
                );

                // If it's not a player we need to set their position to not be colliding with the block.
                // If we ever get around to doing an anticheat system we can do server-side player
                // collision resolution but for now we just let the client handle it.
                if !is_player {
                    let mut resolved = start_pos + delta * entry_time;

                    // Snap the blocking axis directly to the block's boundary rather than
                    // trusting the interpolated `entry_time`, so we land exactly flush against
                    // it instead of a hair short/long due to the division above.
                    let block_min = collided_block.pos.as_vec3a();
                    resolved[axis] = if delta[axis] > 0.0 {
                        block_min[axis] - static_hitbox.max[axis]
                    } else {
                        block_min[axis] + 1.0 - static_hitbox.min[axis]
                    };

                    pos.coords = resolved.as_dvec3();

                    // Zero out velocity on the axis we actually collided on so we don't just
                    // re-collide (and re-correct) again next tick.
                    if let Some(mut vel) = vel {
                        vel.vec[axis] = 0.0;
                    }

                    if axis == 1 {
                        grounded.0 = true;
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

/// Analytic swept-AABB test: given a moving box at `t=0` (`moving_min0`/`moving_max0`)
/// travelling by `velocity` over `t` in `[0, 1]`, finds the exact time it first touches the
/// static `target` box, along with which axis (0=x, 1=y, 2=z) the contact happened on.
/// Returns `None` if the boxes never touch during the sweep.
fn sweep_aabb(
    moving_min0: Vec3A,
    moving_max0: Vec3A,
    target_min: Vec3A,
    target_max: Vec3A,
    velocity: Vec3A,
) -> Option<(f32, usize)> {
    let mut entry = [f32::NEG_INFINITY; 3];
    let mut exit = [f32::INFINITY; 3];

    for axis in 0..3 {
        let v = velocity[axis];
        if v > 0.0 {
            entry[axis] = (target_min[axis] - moving_max0[axis]) / v;
            exit[axis] = (target_max[axis] - moving_min0[axis]) / v;
        } else if v < 0.0 {
            entry[axis] = (target_max[axis] - moving_min0[axis]) / v;
            exit[axis] = (target_min[axis] - moving_max0[axis]) / v;
        } else if moving_max0[axis] <= target_min[axis] || moving_min0[axis] >= target_max[axis] {
            // Never overlapping on this axis regardless of travel on the other two.
            return None;
        }
    }

    let entry_time = entry[0].max(entry[1]).max(entry[2]);
    let exit_time = exit[0].min(exit[1]).min(exit[2]);

    if entry_time > exit_time || entry_time > 1.0 || exit_time < 0.0 {
        return None;
    }

    let axis = if entry[0] >= entry[1] && entry[0] >= entry[2] {
        0
    } else if entry[1] >= entry[2] {
        1
    } else {
        2
    };

    Some((entry_time.max(0.0), axis))
}
