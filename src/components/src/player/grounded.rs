use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Copy, Clone, Serialize, Deserialize)]
pub struct OnGround {
    pub currently_grounded: bool,
    pub was_grounded: bool,
}

impl OnGround {
    pub fn set_grounded(&mut self, grounded: bool) {
        self.was_grounded = self.currently_grounded;
        self.currently_grounded = grounded;
    }

    pub fn just_landed(&self) -> bool {
        !self.was_grounded && self.currently_grounded
    }

    pub fn just_left_ground(&self) -> bool {
        self.was_grounded && !self.currently_grounded
    }
}
