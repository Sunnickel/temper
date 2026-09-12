use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::Discriminant;

pub struct Fireworks {
    pub flight_duration: VarInt,
    pub explosions: LengthPrefixedVec<FireworkExplosion>,
}

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
