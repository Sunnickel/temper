use super::DyeColor;
use bitcode::{Decode, Encode};
use temper_codec::net_types::id_or_inline::IdOr;
use temper_macros::{NetDecode, NetEncode};

#[derive(PartialEq, Debug, Clone, Encode, Decode, NetEncode, NetDecode)]
pub struct BannerPatternLayer {
    pub pattern_type: IdOr<BannerPattern>,
    pub color: DyeColor,
}

#[derive(PartialEq, Debug, Clone, Encode, Decode, NetEncode, NetDecode)]
pub struct BannerPattern {
    pub asset_id: String,
    pub translation_key: String,
}
