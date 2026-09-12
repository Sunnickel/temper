use super::ItemComponent;
use temper_codec::net_types::id_set::IDSet;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_macros::Discriminant;
use temper_nbt::blob::NbtBlob;

pub struct BlockPredicate {
    pub blocks: PrefixedOptional<IDSet>,
    pub properties: PrefixedOptional<LengthPrefixedVec<BlockPredicateProperty>>,
    pub nbt: PrefixedOptional<NbtBlob>,
    pub data_components: LengthPrefixedVec<ExactDataComponentMatcher>,
    pub partial_data_component_predicates: LengthPrefixedVec<PartialDataComponentMatcher>,
}

pub struct BlockPredicateProperty {
    pub name: String,
    pub matcher: BlockPredicatePropertyMatcher,
}

pub enum BlockPredicatePropertyMatcher {
    Exact(String),
    Range {
        min_value: Option<String>,
        max_value: Option<String>,
    },
}

pub struct ExactDataComponentMatcher {
    pub component: Box<ItemComponent>,
}

pub struct PartialDataComponentMatcher {
    pub predicate_type: PartialDataComponentPredicateType,
    pub predicate: NbtBlob,
}

#[derive(Discriminant)]
pub enum PartialDataComponentPredicateType {
    Damage,
    Enchantments,
    StoredEnchantments,
    PotionContents,
    CustomData,
    Container,
    BundleContents,
    FireworkExplosion,
    Fireworks,
    WritableBookContent,
    WrittenBookContent,
    AttributeModifiers,
    Trim,
    JukeboxPlayable,
}
