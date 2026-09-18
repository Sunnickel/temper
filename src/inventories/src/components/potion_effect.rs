use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::{NetDecode, NetEncode};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, NetEncode, NetDecode)]
pub struct PotionEffect {
    pub type_id: VarInt,
    pub amplifier: VarInt,
    pub duration: VarInt,
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,

    /// Not actually used by the client but still expected in the packet
    hidden_effect: PrefixedOptional<()>,
}
