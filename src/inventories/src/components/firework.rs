use std::io::Write;
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::{Discriminant, NetEncode};

#[derive(NetEncode)]
pub struct Fireworks {
    pub flight_duration: VarInt,
    pub explosions: LengthPrefixedVec<FireworkExplosion>,
}

#[derive(NetEncode)]
pub struct FireworkExplosion {
    pub shape: FireworkExplosionShape,
    pub colors: LengthPrefixedVec<i32>,
    pub fade_colors: LengthPrefixedVec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}

#[derive(Discriminant)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl NetEncode for FireworkExplosionShape {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.discriminant()).encode(writer, opts)
    }
}
