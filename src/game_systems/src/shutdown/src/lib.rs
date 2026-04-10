use bevy_ecs::schedule::IntoScheduleConfigs;
pub mod send_save_message;
mod send_shutdown_packet;

use crate::send_save_message::send_save_message;
use background::world_sync::sync_world;
use mobs::save_systems::*;

pub fn register_shutdown_systems(schedule: &mut bevy_ecs::prelude::Schedule) {
    schedule.add_systems((send_save_message, (save_pig,), sync_world).chain());
    schedule.add_systems(send_shutdown_packet::handle);
}
