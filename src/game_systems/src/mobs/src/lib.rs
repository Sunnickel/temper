use bevy_ecs::prelude::Schedule;

pub mod pig;
pub fn register_mob_systems(schedule: &mut Schedule) {
    schedule.add_systems(pig::tick_pig);
    schedule.add_systems(pig::load_pig);
    schedule.add_systems(pig::announce_new_spawned_pig);
}

pub mod save_systems {
    pub use super::pig::save_pig;
}
