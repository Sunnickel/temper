use bevy_ecs::prelude::*;
use temper_components::player::position::Position;
use temper_core::pos::BlockPos;
use temper_entities::PhysicalRegistry;
use temper_entities::components::{Baby, EntityMetadata};
use temper_state::GlobalStateResource;

use crate::astar::{AStarSearch, SearchStep};

const DEFAULT_BUDGET_PER_TICK: usize = 20;
const DEFAULT_MAX_NODES: usize = 500;

/// Pathfinding component for land mob entities.
/// Set `target` each tick (or periodically) from mob AI. The system `tick_pathfinder`
/// advances the A* search incrementally (budget_per_tick nodes/tick) and updates `path`.
/// The mob AI reads `current_waypoint()` and calls `advance_waypoint()` when it arrives.
#[derive(Component)]
pub struct Pathfinder {
    /// Target block to navigate to. Changing this restarts the search.
    pub target: Option<BlockPos>,
    /// Current computed path (nodes from start to goal).
    pub path: Vec<BlockPos>,
    /// Index of the waypoint the mob is currently heading toward.
    pub waypoint: usize,
    /// A* node expansions allowed per tick.
    pub budget_per_tick: usize,
    /// Maximum total A* expansions before giving up.
    pub max_nodes: usize,
    // private
    search: Option<AStarSearch>,
    last_target: Option<BlockPos>,
}

impl Pathfinder {
    pub fn new(budget_per_tick: usize, max_nodes: usize) -> Self {
        Self {
            target: None,
            path: Vec::new(),
            waypoint: 0,
            budget_per_tick,
            max_nodes,
            search: None,
            last_target: None,
        }
    }

    /// The block the mob should currently move toward.
    pub fn current_waypoint(&self) -> Option<BlockPos> {
        self.path.get(self.waypoint).copied()
    }

    /// Move to the next waypoint after reaching the current one.
    pub fn advance_waypoint(&mut self) {
        self.waypoint += 1;
    }

    /// True if a path is available and not yet exhausted.
    pub fn has_path(&self) -> bool {
        self.waypoint < self.path.len()
    }

    /// True if a search is currently in progress.
    pub fn is_searching(&self) -> bool {
        self.search.is_some()
    }

    /// Force a new search on the next tick, even if the target hasn't changed.
    /// Use this when you want to repath to the same block (e.g. the player didn't move).
    pub fn request_repath(&mut self) {
        self.last_target = None;
    }
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new(DEFAULT_BUDGET_PER_TICK, DEFAULT_MAX_NODES)
    }
}

/// Advances incremental A* searches for all entities with a Pathfinder component.
/// Must run before mob AI systems so the updated path is available each tick.
pub fn tick_pathfinder(
    mut query: Query<(&mut Pathfinder, &Position, &EntityMetadata, Has<Baby>)>,
    state: Res<GlobalStateResource>,
    registry: Res<PhysicalRegistry>,
) {
    let world = &state.0.world;

    for (mut pf, pos, metadata, is_baby) in &mut query {
        let Some(props) = registry.get(metadata.protocol_id(), is_baby) else {
            continue;
        };

        // Restart search when target changes, but keep the old path so the mob
        // keeps moving while the new path is being computed.
        if pf.target != pf.last_target {
            pf.last_target = pf.target;
            pf.search = None;

            if let Some(goal) = pf.target {
                let start = pos_to_block(pos);
                if start == goal {
                    pf.path = vec![goal];
                    pf.waypoint = 0;
                } else {
                    pf.search = Some(AStarSearch::new(start, goal, pf.max_nodes, props));
                }
            }
        }

        // Advance the in-progress search by up to budget_per_tick expansions.
        let budget = pf.budget_per_tick;
        if let Some(ref mut search) = pf.search {
            match search.step(world, budget) {
                SearchStep::Found(nodes) => {
                    pf.path = nodes;
                    pf.waypoint = 1; // node 0 is the start position
                    pf.search = None;
                }
                SearchStep::NoPath => {
                    pf.path.clear();
                    pf.waypoint = 0;
                    pf.search = None;
                }
                SearchStep::Continue => {}
            }
        }
    }
}

/// Convert a world-space position to a block position.
///
/// The small Y epsilon compensates for imprecise collision resolution
/// (see TODO in physics/collisions.rs) that can leave entities at e.g.
/// y=64.9999 instead of exactly y=65.0.
pub fn pos_to_block(pos: &Position) -> BlockPos {
    const EPSILON: f64 = 1e-4;
    BlockPos::of(
        pos.x.floor() as i32,
        (pos.y + EPSILON).floor() as i32,
        pos.z.floor() as i32,
    )
}
