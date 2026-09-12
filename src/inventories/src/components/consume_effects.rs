use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;
use crate::components::potion_effect::PotionEffect;

pub struct ConsumeEffect {
    type_id: VarInt,
    
}

pub enum ConsumeEffectData {
    ApplyEffects{
        effects: LengthPrefixedVec<PotionEffect>,
        probability: f32
    },
    
}