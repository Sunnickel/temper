use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Copy, Clone, Serialize, Deserialize)]
pub struct OnGround {
    /// Whether the player is grounded this tick.
    pub currently_grounded: bool,
    /// Whether the player was grounded last tick.  Is updated in physics::ground_state::snapshot. Don't update this manually
    pub was_grounded: bool,
}

impl OnGround {
    pub fn snapshot(&mut self) {
        self.was_grounded = self.currently_grounded;
    }

    pub fn set_grounded(&mut self, grounded: bool) {
        self.currently_grounded = grounded;
    }

    pub fn just_landed(&self) -> bool {
        !self.was_grounded && self.currently_grounded
    }

    pub fn just_left_ground(&self) -> bool {
        self.was_grounded && !self.currently_grounded
    }
}
