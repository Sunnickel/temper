use crate::player::position::Position;
use bevy_ecs::prelude::Component;
use bevy_math::DVec3;
use deref_derive::{Deref, DerefMut};

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct OldPosition(DVec3);

impl OldPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        OldPosition(DVec3::new(x, y, z))
    }
}

impl From<Position> for OldPosition {
    fn from(value: Position) -> Self {
        OldPosition(value.coords)
    }
}

impl From<OldPosition> for Position {
    fn from(value: OldPosition) -> Self {
        Position::new(value.x, value.y, value.z)
    }
}
