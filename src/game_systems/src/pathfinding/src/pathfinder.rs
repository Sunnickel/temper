use bevy_ecs::prelude::*;
use temper_components::pathfinder::Pathfinder;
use temper_components::player::position::Position;
use temper_core::pos::BlockPos;
use temper_entities::PhysicalRegistry;
use temper_entities::components::{Baby, EntityMetadata};
use temper_state::GlobalStateResource;

use crate::astar::{AStarSearch, SearchStep};

/// Internal A* search state, kept separate from the public Pathfinder component
/// to avoid a dependency on this crate from temper-components.
#[derive(Component, Default)]
pub struct PathfinderSearch(pub(crate) Option<AStarSearch>);

type PathfinderQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static mut Pathfinder,
        Option<&'static mut PathfinderSearch>,
        &'static Position,
        &'static EntityMetadata,
        Has<Baby>,
    ),
>;

/// Advances incremental A* searches for all entities with a Pathfinder component.
/// Must run before mob AI systems so the updated path is available each tick.
pub fn tick_pathfinder(
    mut commands: Commands,
    mut query: PathfinderQuery,
    state: Res<GlobalStateResource>,
    registry: Res<PhysicalRegistry>,
) {
    let world = &state.0.world;

    for (entity, mut pf, search, pos, metadata, is_baby) in &mut query {
        let Some(props) = registry.get(metadata.protocol_id(), is_baby) else {
            continue;
        };

        let mut search = match search {
            Some(s) => s,
            None => {
                commands.entity(entity).insert(PathfinderSearch::default());
                continue;
            }
        };

        // Restart search when target changes, but keep the old path so the mob
        // keeps moving while the new path is being computed.
        if pf.target != pf.last_target {
            pf.last_target = pf.target;
            search.0 = None;

            if let Some(goal) = pf.target {
                let start = pos_to_block(pos);
                if start == goal {
                    pf.path = vec![goal];
                    pf.waypoint = 0;
                    pf.is_searching = false;
                } else {
                    search.0 = Some(AStarSearch::new(start, goal, pf.max_nodes, props));
                    pf.is_searching = true;
                }
            }
        }

        // Advance the in-progress search by up to budget_per_tick expansions.
        let budget = pf.budget_per_tick;
        if let Some(ref mut astar) = search.0 {
            match astar.step(world, budget) {
                SearchStep::Found(nodes) => {
                    pf.path = nodes;
                    pf.waypoint = 1; // node 0 is the start position
                    pf.is_searching = false;
                    search.0 = None;
                }
                SearchStep::NoPath => {
                    pf.path.clear();
                    pf.waypoint = 0;
                    pf.is_searching = false;
                    search.0 = None;
                }
                SearchStep::Continue => {}
            }
        }
    }
}

/// Convert a world-space position to a block position.
///
/// The small Y epsilon compensates for imprecise collision resolution
/// that can leave entities at e.g. y=64.9999 instead of exactly y=65.0.
pub fn pos_to_block(pos: &Position) -> BlockPos {
    const EPSILON: f64 = 1e-4;
    BlockPos::of(
        pos.x.floor() as i32,
        (pos.y + EPSILON).floor() as i32,
        pos.z.floor() as i32,
    )
}
