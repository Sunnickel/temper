use super::SoundEvent;
use bitcode::{Decode, Encode};
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::{NetDecode, NetEncode};

#[derive(PartialEq, Debug, Clone, Encode, Decode, NetEncode, NetDecode)]
pub struct KineticWeapon {
    pub contact_cooldown_ticks: VarInt,
    pub delay_ticks: VarInt,
    pub dismount_conditions: PrefixedOptional<KineticWeaponConditions>,
    pub knockback_conditions: PrefixedOptional<KineticWeaponConditions>,
    pub damage_conditions: PrefixedOptional<KineticWeaponConditions>,
    pub forward_movement: f32,
    pub damage_multiplier: f32,
    pub sound: PrefixedOptional<SoundEvent>,
    pub hit_sound: PrefixedOptional<SoundEvent>,
}
#[derive(PartialEq, Debug, Clone, Encode, Decode, NetEncode, NetDecode)]
pub struct KineticWeaponConditions {
    pub max_duration_ticks: VarInt,
    pub min_speed: f32,
    pub min_relative_speed: f32,
}
