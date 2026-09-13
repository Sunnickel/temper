use super::ItemComponent;
use std::io::Write;
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::id_set::IDSet;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::{Discriminant, NetEncode};
use temper_nbt::blob::NbtBlob;

#[derive(NetEncode)]
pub struct BlockPredicate {
    pub blocks: PrefixedOptional<IDSet>,
    pub properties: PrefixedOptional<LengthPrefixedVec<BlockPredicateProperty>>,
    pub nbt: PrefixedOptional<NbtBlob>,
    pub data_components: LengthPrefixedVec<ExactDataComponentMatcher>,
    pub partial_data_component_predicates: LengthPrefixedVec<PartialDataComponentMatcher>,
}

#[derive(NetEncode)]
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

impl NetEncode for ExactDataComponentMatcher {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.component.as_ref().encode(writer, &NetEncodeOpts::None)
    }
}

#[derive(NetEncode)]
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

impl NetEncode for BlockPredicatePropertyMatcher {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Self::Exact(value) => {
                true.encode(writer, &NetEncodeOpts::None)?;
                value.encode(writer, &NetEncodeOpts::None)
            }
            Self::Range {
                min_value,
                max_value,
            } => {
                false.encode(writer, &NetEncodeOpts::None)?;
                min_value.encode(writer, &NetEncodeOpts::None)?;
                max_value.encode(writer, &NetEncodeOpts::None)
            }
        }
    }
}

impl NetEncode for PartialDataComponentPredicateType {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.discriminant()).encode(writer, opts)
    }
}
