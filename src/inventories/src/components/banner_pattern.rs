use super::DyeColor;
use temper_codec::net_types::id_or_inline::IdOr;

pub struct BannerPatternLayer {
    pub pattern_type: IdOr<BannerPattern>,
    pub color: DyeColor,
}

pub struct BannerPattern {
    pub asset_id: String,
    pub translation_key: String,
}
