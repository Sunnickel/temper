use crate::components::potion_effect::PotionEffect;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;

pub struct ConsumeEffect {
    pub type_id: VarInt,
    pub data: ConsumeEffectData,
}

pub enum ConsumeEffectData {
    ApplyEffects {
        effects: LengthPrefixedVec<PotionEffect>,
        probability: f32,
    },
}
