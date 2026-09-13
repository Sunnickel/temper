use crate::components::potion_effect::PotionEffect;
use std::io::Write;
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::NetEncode;

#[derive(NetEncode)]
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

impl NetEncode for ConsumeEffectData {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Self::ApplyEffects {
                effects,
                probability,
            } => {
                effects.encode(writer, &NetEncodeOpts::None)?;
                probability.encode(writer, &NetEncodeOpts::None)
            }
        }
    }
}
