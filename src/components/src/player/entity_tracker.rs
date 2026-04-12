use bevy_ecs::entity::EntityHashSet;
use bevy_ecs::prelude::{Component, Entity};
use crossbeam_queue::SegQueue;
use uuid::Uuid;

#[derive(Component, Default)]
pub struct EntityTracker {
    pub to_track: SegQueue<(Uuid, u16)>,
    pub tracking: EntityHashSet,
    pub to_untrack: SegQueue<Entity>,
}
