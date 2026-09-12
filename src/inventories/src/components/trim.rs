use temper_codec::net_types::id_or_inline::IdOr;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::var_int::VarInt;
use temper_text::TextComponent;

pub struct Trim {
    pub material: IdOr<TrimMaterial>,
    pub pattern: IdOr<TrimPattern>,
}

pub struct TrimMaterial {
    pub suffix: String,
    pub overrides: LengthPrefixedVec<TrimMaterialOverride>,
    pub description: TextComponent,
}

pub struct TrimMaterialOverride {
    pub armor_material_type: String,
    pub overridden_asset_name: String,
}

pub struct TrimPattern {
    pub asset_name: String,
    pub template_item: VarInt,
    pub description: TextComponent,
    pub decal: bool,
}
