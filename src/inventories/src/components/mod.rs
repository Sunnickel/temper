pub mod banner_pattern;
pub mod block_predicate;
pub mod consume_effects;
pub mod firework;
pub mod instrument;
pub mod jukebox;
pub mod kinetic_weapon;
pub mod painting_variant;
pub mod potion_effect;
pub mod trim;

pub use banner_pattern::{BannerPattern, BannerPatternLayer};
pub use block_predicate::BlockPredicate;
pub use consume_effects::ConsumeEffect;
pub use firework::{FireworkExplosion, FireworkExplosionShape, Fireworks};
pub use instrument::Instrument;
pub use jukebox::JukeboxSong;
pub use kinetic_weapon::{KineticWeapon, KineticWeaponConditions};
pub use painting_variant::PaintingVariant;
pub use potion_effect::PotionEffect;
pub use trim::{Trim, TrimMaterial, TrimMaterialOverride, TrimPattern};

use crate::slot::InventorySlot;
use std::io::{Cursor, Read, Write};
use temper_codec::decode::errors::NetDecodeError;
use temper_codec::decode::{NetDecode, NetDecodeOpts};
use temper_codec::encode::errors::NetEncodeError;
use temper_codec::encode::{NetEncode, NetEncodeOpts};
use temper_codec::net_types::id_or_inline::IdOr;
use temper_codec::net_types::id_set::IDSet;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::network_position::NetworkPosition;
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::{Discriminant, NetDecode, NetEncode};
use temper_nbt::NBT;
use temper_nbt::blob::NbtBlob;
use temper_text::TextComponent;

macro_rules! impl_discriminant_net_encode {
    ($type:ty) => {
        impl NetEncode for $type {
            fn encode<W: Write>(
                &self,
                writer: &mut W,
                opts: &NetEncodeOpts,
            ) -> Result<(), NetEncodeError> {
                VarInt::new(self.discriminant()).encode(writer, opts)
            }
        }
    };
}

macro_rules! impl_discriminant_net_decode {
    ($type:ty { $($id:literal => $variant:path),+ $(,)? }) => {
        impl NetDecode for $type {
            fn decode<R: Read>(
                reader: &mut R,
                opts: &NetDecodeOpts,
            ) -> Result<Self, NetDecodeError> {
                match VarInt::decode(reader, opts)?.0 {
                    $($id => Ok($variant),)+
                    _ => Err(NetDecodeError::InvalidEnumVariant),
                }
            }
        }
    };
}

#[derive(Discriminant)]
pub enum ItemComponent {
    CustomData(NbtBlob),
    MaxStackSize(VarInt),
    MaxDamage(VarInt),
    Damage(VarInt),
    Unbreakable,

    CustomName(Box<TextComponent>),
    ItemName(Box<TextComponent>),
    ItemModel(String),
    Lore(LengthPrefixedVec<TextComponent>),
    Rarity(Rarity),
    Enchantments(LengthPrefixedVec<Enchantment>),

    CanPlaceOn(LengthPrefixedVec<BlockPredicate>),
    CanBreak(LengthPrefixedVec<BlockPredicate>),

    AttributeModifiers(LengthPrefixedVec<AttributeModifier>),
    CustomModelData(CustomModelData),
    TooltipDisplay(TooltipDisplay),
    RepairCost(VarInt),
    CreativeSlotLock,
    EnchantmentGlintOverride(bool),
    IntangibleProjectile(NbtBlob),
    Food(Food),

    Consumable(Consumable),

    UseRemainder(Box<InventorySlot>),
    UseCooldown(UseCooldown),
    UseEffects(UseEffects),
    MinimumAttackCharge(f32),
    DamageType(VarInt),

    DamageResistant(IDSet),

    AttackRange(AttackRange),

    Tool(Tool),

    Weapon(Weapon),
    Enchantable(VarInt),

    Equippable(Equippable),

    Repairable(IDSet),

    Glider,
    TooltipStyle(String),

    DeathProtection(LengthPrefixedVec<ConsumeEffect>),

    BlocksAttacks(BlocksAttacks),

    StoredEnchantments(LengthPrefixedVec<Enchantment>),
    DyedColor(i32),
    MapColor(i32),
    MapId(VarInt),
    MapDecorations(NbtBlob),
    MapPostProcessing(MapPostProcessing),
    ChargedProjectiles(LengthPrefixedVec<InventorySlot>),
    BundleContents(LengthPrefixedVec<InventorySlot>),

    PotionContents(PotionContents),

    PotionDurationScale(f32),
    SuspiciousStewEffects(LengthPrefixedVec<SuspiciousStewEffect>),
    WritableBookContent(LengthPrefixedVec<WritableBookPage>),
    WrittenBookContent(WrittenBookContent),

    Trim(Box<Trim>),

    DebugStickState(NbtBlob),
    EntityData(EntityData),
    BucketEntityData(NbtBlob),
    BlockEntityData(BlockEntityData),

    Instrument(IdOr<Instrument>),

    PiercingWeapon(PiercingWeapon),

    KineticWeapon(KineticWeapon),

    SwingAnimation(SwingAnimation),
    AdditionalTradeCost(VarInt),
    Dye(DyeColor),

    ProvidesTrimMaterial(IdOr<TrimMaterial>),

    OminousBottleAmplifier(VarInt),

    JukeboxPlayable(IdOr<JukeboxSong>),

    ProvidesBannerPatterns(IDSet),

    Recipes(NbtBlob),
    LodestoneTracker(LodestoneTracker),

    FireworkExplosion(FireworkExplosion),
    Fireworks(Fireworks),

    // not implemented
    Profile,

    NoteBlockSound(String),

    BannerPatterns(LengthPrefixedVec<BannerPatternLayer>),

    BaseColor(DyeColor),
    PotDecorations(LengthPrefixedVec<VarInt>),
    Container(LengthPrefixedVec<InventorySlot>),
    BlockState(LengthPrefixedVec<BlockStateProperty>),
    Bees(LengthPrefixedVec<Bee>),
    Lock(NbtBlob),
    ContainerLoot(NbtBlob),

    BreakSound(IdOr<SoundEvent>),

    SulfurCubeContent(Box<InventorySlot>),
    VillagerVariant(VarInt),
    WolfVariant(VarInt),
    WolfSoundVariant(VarInt),
    WolfCollar(DyeColor),
    FoxVariant(VarInt),
    SalmonSize(VarInt),
    ParrotVariant(VarInt),
    TropicalFishPattern(VarInt),
    TropicalFishBaseColor(DyeColor),
    TropicalFishPatternColor(DyeColor),
    MooshroomVariant(VarInt),
    RabbitVariant(VarInt),
    PigVariant(VarInt),
    PigSoundVariant(VarInt),
    CowVariant(VarInt),
    CowSoundVariant(VarInt),
    ChickenVariant(VarInt),
    ChickenSoundVariant(VarInt),
    FrogVariant(VarInt),
    HorseVariant(VarInt),

    PaintingVariant(IdOr<PaintingVariant>),

    LlamaVariant(VarInt),
    AxolotlVariant(VarInt),
    ZombieNautilusVariant(VarInt),
    CatVariant(VarInt),
    CatSoundVariant(VarInt),
    CatCollar(DyeColor),
    SheepColor(DyeColor),
    ShulkerColor(DyeColor),
}

#[derive(Discriminant)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl_discriminant_net_encode!(Rarity);
impl_discriminant_net_decode!(Rarity {
    0 => Rarity::Common,
    1 => Rarity::Uncommon,
    2 => Rarity::Rare,
    3 => Rarity::Epic,
});

#[derive(NetEncode, NetDecode)]
pub struct Enchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}

#[derive(NetEncode, NetDecode)]
pub struct AttributeModifier {
    pub attribute_id: VarInt,
    pub modifier_id: String,
    pub value: f64,
    pub operation: AttributeModifierOperation,
    pub slot: AttributeModifierSlot,
}

#[derive(Discriminant)]
pub enum AttributeModifierOperation {
    Add,
    MultiplyBase,
    MultiplyTotal,
}

impl_discriminant_net_encode!(AttributeModifierOperation);
impl_discriminant_net_decode!(AttributeModifierOperation {
    0 => AttributeModifierOperation::Add,
    1 => AttributeModifierOperation::MultiplyBase,
    2 => AttributeModifierOperation::MultiplyTotal,
});

#[derive(Discriminant)]
pub enum AttributeModifierSlot {
    Any,
    MainHand,
    OffHand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body,
}

impl_discriminant_net_encode!(AttributeModifierSlot);
impl_discriminant_net_decode!(AttributeModifierSlot {
    0 => AttributeModifierSlot::Any,
    1 => AttributeModifierSlot::MainHand,
    2 => AttributeModifierSlot::OffHand,
    3 => AttributeModifierSlot::Hand,
    4 => AttributeModifierSlot::Feet,
    5 => AttributeModifierSlot::Legs,
    6 => AttributeModifierSlot::Chest,
    7 => AttributeModifierSlot::Head,
    8 => AttributeModifierSlot::Armor,
    9 => AttributeModifierSlot::Body,
});

#[derive(NetEncode, NetDecode)]
pub struct CustomModelData {
    pub floats: LengthPrefixedVec<f32>,
    pub flags: LengthPrefixedVec<bool>,
    pub strings: LengthPrefixedVec<String>,
    pub colors: LengthPrefixedVec<i32>,
}

#[derive(NetEncode, NetDecode)]
pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    pub hidden_components: LengthPrefixedVec<VarInt>,
}

#[derive(NetEncode, NetDecode)]
pub struct Food {
    pub nutrition: VarInt,
    pub saturation_modifier: f32,
    pub can_always_eat: bool,
}

#[derive(NetEncode, NetDecode)]
pub struct Consumable {
    pub consume_seconds: f32,
    pub animation: ConsumeAnimation,
    pub sound: IdOr<SoundEvent>,
    pub has_consume_particles: bool,
    pub effects: LengthPrefixedVec<ConsumeEffect>,
}

#[derive(Discriminant)]
pub enum ConsumeAnimation {
    None,
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

impl_discriminant_net_encode!(ConsumeAnimation);
impl_discriminant_net_decode!(ConsumeAnimation {
    0 => ConsumeAnimation::None,
    1 => ConsumeAnimation::Eat,
    2 => ConsumeAnimation::Drink,
    3 => ConsumeAnimation::Block,
    4 => ConsumeAnimation::Bow,
    5 => ConsumeAnimation::Spear,
    6 => ConsumeAnimation::Crossbow,
    7 => ConsumeAnimation::Spyglass,
    8 => ConsumeAnimation::TootHorn,
    9 => ConsumeAnimation::Brush,
});

#[derive(NetEncode, NetDecode)]
pub struct UseCooldown {
    pub seconds: f32,
    pub cooldown_group: PrefixedOptional<String>,
}

#[derive(NetEncode, NetDecode)]
pub struct UseEffects {
    pub can_sprint: bool,
    pub interact_vibrations: bool,
    pub speed_multiplier: f32,
}

#[derive(NetEncode, NetDecode)]
pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}

#[derive(NetEncode, NetDecode)]
pub struct Weapon {
    pub damage_per_attack: VarInt,
    pub disable_blocking_for: f32,
}

#[derive(NetEncode, NetDecode)]
pub struct Tool {
    pub rules: LengthPrefixedVec<ToolRule>,
    pub default_mining_speed: f32,
    pub damage_per_block: VarInt,
    pub can_destroy_blocks_in_creative: bool,
}

#[derive(NetEncode, NetDecode)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<f32>,
    pub correct_drop_for_blocks: PrefixedOptional<bool>,
}

#[derive(NetEncode, NetDecode)]
pub struct Equippable {
    pub slot: EquippableSlot,
    pub equip_sound: IdOr<SoundEvent>,
    pub model: PrefixedOptional<String>,
    pub camera_overlay: PrefixedOptional<String>,
    pub allowed_entities: PrefixedOptional<IDSet>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
    pub can_be_sheared: bool,
    pub shearing_sound: IdOr<SoundEvent>,
}

#[derive(Discriminant)]
pub enum EquippableSlot {
    MainHand,
    Feet,
    Legs,
    Chest,
    Head,
    OffHand,
    Body,
}

impl_discriminant_net_encode!(EquippableSlot);
impl_discriminant_net_decode!(EquippableSlot {
    0 => EquippableSlot::MainHand,
    1 => EquippableSlot::Feet,
    2 => EquippableSlot::Legs,
    3 => EquippableSlot::Chest,
    4 => EquippableSlot::Head,
    5 => EquippableSlot::OffHand,
    6 => EquippableSlot::Body,
});

#[derive(NetEncode, NetDecode)]
pub struct BlocksAttacks {
    pub block_delay_seconds: f32,
    pub disable_cooldown_scale: f32,
    pub damage_reductions: LengthPrefixedVec<DamageReduction>,
    pub item_damage_threshold: f32,
    pub item_damage_base: f32,
    pub item_damage_factor: f32,
    pub bypassed_by: PrefixedOptional<IDSet>,
    pub block_sound: PrefixedOptional<IdOr<SoundEvent>>,
    pub disable_sound: PrefixedOptional<IdOr<SoundEvent>>,
}

#[derive(NetEncode, NetDecode)]
pub struct DamageReduction {
    pub horizontal_blocking_angle: f32,
    pub r#type: PrefixedOptional<IDSet>,
    pub base: f32,
    pub factor: f32,
}

#[derive(Discriminant)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

impl_discriminant_net_encode!(MapPostProcessing);
impl_discriminant_net_decode!(MapPostProcessing {
    0 => MapPostProcessing::Lock,
    1 => MapPostProcessing::Scale,
});

#[derive(NetEncode, NetDecode)]
pub struct SuspiciousStewEffect {
    pub type_id: VarInt,
    pub duration: VarInt,
}

#[derive(NetEncode, NetDecode)]
pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<i32>,
    pub custom_effects: LengthPrefixedVec<PotionEffect>,
    pub custom_name: PrefixedOptional<String>,
}

#[derive(NetEncode, NetDecode)]
pub struct WritableBookPage {
    pub raw_content: String,
    pub filtered_content: PrefixedOptional<String>,
}

pub struct WrittenBookContent {
    pub raw_title: String,
    pub filtered_title: PrefixedOptional<String>,
    pub author: String,
    pub generation: VarInt,
    pub pages: LengthPrefixedVec<WrittenBookPage>,
    pub resolved: bool,
}

pub struct WrittenBookPage {
    pub raw_content: TextComponent,
    pub filtered_content: PrefixedOptional<TextComponent>,
}

#[derive(NetEncode, NetDecode)]
pub struct EntityData {
    pub entity_type: VarInt,
    pub data: NbtBlob,
}

#[derive(NetEncode, NetDecode)]
pub struct BlockEntityData {
    pub block_entity_type: VarInt,
    pub data: NbtBlob,
}

#[derive(NetEncode, NetDecode)]
pub struct PiercingWeapon {
    pub deals_knockback: bool,
    pub dismounts: bool,
    pub sound: PrefixedOptional<SoundEvent>,
    pub hit_sound: PrefixedOptional<SoundEvent>,
}

#[derive(NetEncode, NetDecode)]
pub struct SwingAnimation {
    pub r#type: SwingAnimationType,
    pub duration: VarInt,
}

#[derive(Discriminant)]
pub enum SwingAnimationType {
    None,
    Whack,
    Stab,
}

impl_discriminant_net_encode!(SwingAnimationType);
impl_discriminant_net_decode!(SwingAnimationType {
    0 => SwingAnimationType::None,
    1 => SwingAnimationType::Whack,
    2 => SwingAnimationType::Stab,
});

#[derive(Discriminant)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

impl_discriminant_net_encode!(DyeColor);
impl_discriminant_net_decode!(DyeColor {
    0 => DyeColor::White,
    1 => DyeColor::Orange,
    2 => DyeColor::Magenta,
    3 => DyeColor::LightBlue,
    4 => DyeColor::Yellow,
    5 => DyeColor::Lime,
    6 => DyeColor::Pink,
    7 => DyeColor::Gray,
    8 => DyeColor::LightGray,
    9 => DyeColor::Cyan,
    10 => DyeColor::Purple,
    11 => DyeColor::Blue,
    12 => DyeColor::Brown,
    13 => DyeColor::Green,
    14 => DyeColor::Red,
    15 => DyeColor::Black,
});

#[derive(NetEncode)]
pub struct LodestoneTracker {
    pub has_global_position: bool,
    pub dimension: Option<String>,
    pub position: Option<NetworkPosition>,
    pub tracked: bool,
}

#[derive(NetEncode, NetDecode)]
pub struct SoundEvent {
    pub sound_id: String,
    pub fixed_range: PrefixedOptional<f32>,
}

#[derive(NetEncode, NetDecode)]
pub struct BlockStateProperty {
    pub name: String,
    pub value: String,
}

#[derive(NetEncode, NetDecode)]
pub struct Bee {
    pub entity_type: VarInt,
    pub entity_data: NbtBlob,
    pub ticks_in_hive: VarInt,
    pub min_ticks_in_hive: VarInt,
}

pub(crate) fn encode_text_component<W: Write>(
    text: &TextComponent,
    writer: &mut W,
    opts: &NetEncodeOpts,
) -> Result<(), NetEncodeError> {
    NBT::from(text.clone()).encode(writer, opts)
}

fn encode_text_components<W: Write>(
    text: &LengthPrefixedVec<TextComponent>,
    writer: &mut W,
) -> Result<(), NetEncodeError> {
    text.length.encode(writer, &NetEncodeOpts::None)?;
    for component in &text.data {
        encode_text_component(component, writer, &NetEncodeOpts::None)?;
    }
    Ok(())
}

fn encode_component_body<W: Write>(
    component: &ItemComponent,
    writer: &mut W,
) -> Result<(), NetEncodeError> {
    VarInt::new(component.discriminant()).encode(writer, &NetEncodeOpts::None)?;

    match component {
        ItemComponent::CustomData(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MaxStackSize(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MaxDamage(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Damage(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Unbreakable => Ok(()),
        ItemComponent::CustomName(value) => {
            encode_text_component(value.as_ref(), writer, &NetEncodeOpts::None)
        }
        ItemComponent::ItemName(value) => {
            encode_text_component(value.as_ref(), writer, &NetEncodeOpts::None)
        }
        ItemComponent::ItemModel(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Lore(value) => encode_text_components(value, writer),
        ItemComponent::Rarity(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Enchantments(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CanPlaceOn(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CanBreak(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::AttributeModifiers(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CustomModelData(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::TooltipDisplay(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::RepairCost(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CreativeSlotLock => Ok(()),
        ItemComponent::EnchantmentGlintOverride(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::IntangibleProjectile(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Food(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Consumable(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::UseRemainder(value) => value.as_ref().encode(writer, &NetEncodeOpts::None),
        ItemComponent::UseCooldown(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::UseEffects(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MinimumAttackCharge(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::DamageType(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::DamageResistant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::AttackRange(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Tool(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Weapon(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Enchantable(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Equippable(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Repairable(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Glider => Ok(()),
        ItemComponent::TooltipStyle(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::DeathProtection(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BlocksAttacks(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::StoredEnchantments(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::DyedColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MapColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MapId(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MapDecorations(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MapPostProcessing(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ChargedProjectiles(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BundleContents(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PotionContents(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PotionDurationScale(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::SuspiciousStewEffects(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::WritableBookContent(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::WrittenBookContent(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Trim(value) => value.as_ref().encode(writer, &NetEncodeOpts::None),
        ItemComponent::DebugStickState(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::EntityData(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BucketEntityData(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BlockEntityData(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Instrument(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PiercingWeapon(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::KineticWeapon(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::SwingAnimation(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::AdditionalTradeCost(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Dye(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ProvidesTrimMaterial(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::OminousBottleAmplifier(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::JukeboxPlayable(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ProvidesBannerPatterns(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Recipes(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::LodestoneTracker(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::FireworkExplosion(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Fireworks(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Profile => unimplemented!(),
        ItemComponent::NoteBlockSound(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BannerPatterns(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BaseColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PotDecorations(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Container(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BlockState(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Bees(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::Lock(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ContainerLoot(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::BreakSound(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::SulfurCubeContent(value) => value.as_ref().encode(writer, &NetEncodeOpts::None),
        ItemComponent::VillagerVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::WolfVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::WolfSoundVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::WolfCollar(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::FoxVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::SalmonSize(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ParrotVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::TropicalFishPattern(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::TropicalFishBaseColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::TropicalFishPatternColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::MooshroomVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::RabbitVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PigVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PigSoundVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CowVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CowSoundVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ChickenVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ChickenSoundVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::FrogVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::HorseVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::PaintingVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::LlamaVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::AxolotlVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ZombieNautilusVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CatVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CatSoundVariant(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::CatCollar(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::SheepColor(value) => value.encode(writer, &NetEncodeOpts::None),
        ItemComponent::ShulkerColor(value) => value.encode(writer, &NetEncodeOpts::None),
    }
}

impl NetEncode for ItemComponent {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match opts {
            NetEncodeOpts::None => encode_component_body(self, writer),
            NetEncodeOpts::WithLength => {
                let mut body = Vec::new();
                encode_component_body(self, &mut body)?;

                VarInt::new(body.len() as i32).encode(writer, &NetEncodeOpts::None)?;
                writer.write_all(&body)?;
                Ok(())
            }
            e => unimplemented!("Unsupported option for NetEncode: {:?}", e),
        }
    }
}

impl NetEncode for WrittenBookContent {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.raw_title.encode(writer, &NetEncodeOpts::None)?;
        self.filtered_title.encode(writer, &NetEncodeOpts::None)?;
        self.author.encode(writer, &NetEncodeOpts::None)?;
        self.generation.encode(writer, &NetEncodeOpts::None)?;
        self.pages.encode(writer, &NetEncodeOpts::None)?;
        self.resolved.encode(writer, &NetEncodeOpts::None)
    }
}

impl NetEncode for WrittenBookPage {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        encode_text_component(&self.raw_content, writer, &NetEncodeOpts::None)?;

        match &self.filtered_content {
            PrefixedOptional::None => false.encode(writer, &NetEncodeOpts::None),
            PrefixedOptional::Some(value) => {
                true.encode(writer, &NetEncodeOpts::None)?;
                encode_text_component(value, writer, &NetEncodeOpts::None)
            }
        }
    }
}

impl NetDecode for LodestoneTracker {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let has_global_position = bool::decode(reader, opts)?;
        let dimension = if has_global_position {
            Some(String::decode(reader, opts)?)
        } else {
            None
        };
        let position = if has_global_position {
            Some(NetworkPosition::decode(reader, opts)?)
        } else {
            None
        };
        let tracked = bool::decode(reader, opts)?;

        Ok(Self {
            has_global_position,
            dimension,
            position,
            tracked,
        })
    }
}

impl NetDecode for ItemComponent {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        match opts {
            NetDecodeOpts::None => decode_component_body(reader),
            NetDecodeOpts::IsSizePrefixed => {
                let length = VarInt::decode(reader, &NetDecodeOpts::None)?.0;
                if length < 0 {
                    return Err(NetDecodeError::ExternalError(
                        format!("negative item component length: {length}").into(),
                    ));
                }

                let mut buf = vec![0; length as usize];
                reader.read_exact(&mut buf)?;
                decode_component_body(&mut Cursor::new(buf))
            }
        }
    }
}

fn decode_component_body<R: Read>(reader: &mut R) -> Result<ItemComponent, NetDecodeError> {
    match VarInt::decode(reader, &NetDecodeOpts::None)?.0 {
        0 => Ok(ItemComponent::CustomData(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        1 => Ok(ItemComponent::MaxStackSize(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        2 => Ok(ItemComponent::MaxDamage(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        3 => Ok(ItemComponent::Damage(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        4 => Ok(ItemComponent::Unbreakable),
        5 => unsupported_component("custom_name"),
        6 => unsupported_component("item_name"),
        7 => Ok(ItemComponent::ItemModel(String::decode(reader, &NetDecodeOpts::None)?)),
        8 => unsupported_component("lore"),
        9 => Ok(ItemComponent::Rarity(Rarity::decode(reader, &NetDecodeOpts::None)?)),
        10 => Ok(ItemComponent::Enchantments(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        11 => Ok(ItemComponent::CanPlaceOn(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        12 => Ok(ItemComponent::CanBreak(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        13 => Ok(ItemComponent::AttributeModifiers(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        14 => Ok(ItemComponent::CustomModelData(CustomModelData::decode(reader, &NetDecodeOpts::None)?)),
        15 => Ok(ItemComponent::TooltipDisplay(TooltipDisplay::decode(reader, &NetDecodeOpts::None)?)),
        16 => Ok(ItemComponent::RepairCost(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        17 => Ok(ItemComponent::CreativeSlotLock),
        18 => Ok(ItemComponent::EnchantmentGlintOverride(bool::decode(reader, &NetDecodeOpts::None)?)),
        19 => Ok(ItemComponent::IntangibleProjectile(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        20 => Ok(ItemComponent::Food(Food::decode(reader, &NetDecodeOpts::None)?)),
        21 => Ok(ItemComponent::Consumable(Consumable::decode(reader, &NetDecodeOpts::None)?)),
        22 => Ok(ItemComponent::UseRemainder(decode_box(reader)?)),
        23 => Ok(ItemComponent::UseCooldown(UseCooldown::decode(reader, &NetDecodeOpts::None)?)),
        24 => Ok(ItemComponent::UseEffects(UseEffects::decode(reader, &NetDecodeOpts::None)?)),
        25 => Ok(ItemComponent::MinimumAttackCharge(f32::decode(reader, &NetDecodeOpts::None)?)),
        26 => Ok(ItemComponent::DamageType(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        27 => Ok(ItemComponent::DamageResistant(IDSet::decode(reader, &NetDecodeOpts::None)?)),
        28 => Ok(ItemComponent::AttackRange(AttackRange::decode(reader, &NetDecodeOpts::None)?)),
        29 => Ok(ItemComponent::Tool(Tool::decode(reader, &NetDecodeOpts::None)?)),
        30 => Ok(ItemComponent::Weapon(Weapon::decode(reader, &NetDecodeOpts::None)?)),
        31 => Ok(ItemComponent::Enchantable(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        32 => Ok(ItemComponent::Equippable(Equippable::decode(reader, &NetDecodeOpts::None)?)),
        33 => Ok(ItemComponent::Repairable(IDSet::decode(reader, &NetDecodeOpts::None)?)),
        34 => Ok(ItemComponent::Glider),
        35 => Ok(ItemComponent::TooltipStyle(String::decode(reader, &NetDecodeOpts::None)?)),
        36 => Ok(ItemComponent::DeathProtection(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        37 => Ok(ItemComponent::BlocksAttacks(BlocksAttacks::decode(reader, &NetDecodeOpts::None)?)),
        38 => Ok(ItemComponent::StoredEnchantments(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        39 => Ok(ItemComponent::DyedColor(i32::decode(reader, &NetDecodeOpts::None)?)),
        40 => Ok(ItemComponent::MapColor(i32::decode(reader, &NetDecodeOpts::None)?)),
        41 => Ok(ItemComponent::MapId(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        42 => Ok(ItemComponent::MapDecorations(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        43 => Ok(ItemComponent::MapPostProcessing(MapPostProcessing::decode(reader, &NetDecodeOpts::None)?)),
        44 => Ok(ItemComponent::ChargedProjectiles(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        45 => Ok(ItemComponent::BundleContents(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        46 => Ok(ItemComponent::PotionContents(PotionContents::decode(reader, &NetDecodeOpts::None)?)),
        47 => Ok(ItemComponent::PotionDurationScale(f32::decode(reader, &NetDecodeOpts::None)?)),
        48 => Ok(ItemComponent::SuspiciousStewEffects(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        49 => Ok(ItemComponent::WritableBookContent(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        50 => unsupported_component("written_book_content"),
        51 => Ok(ItemComponent::Trim(Box::new(Trim {
            material: decode_id_or_inline_unsupported::<TrimMaterial, _>(reader, "trim material")?,
            pattern: decode_id_or_inline_unsupported::<TrimPattern, _>(reader, "trim pattern")?,
        }))),
        52 => Ok(ItemComponent::DebugStickState(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        53 => Ok(ItemComponent::EntityData(EntityData::decode(reader, &NetDecodeOpts::None)?)),
        54 => Ok(ItemComponent::BucketEntityData(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        55 => Ok(ItemComponent::BlockEntityData(BlockEntityData::decode(reader, &NetDecodeOpts::None)?)),
        56 => Ok(ItemComponent::Instrument(
            decode_id_or_inline_unsupported::<Instrument, _>(reader, "instrument")?,
        )),
        57 => Ok(ItemComponent::PiercingWeapon(PiercingWeapon::decode(reader, &NetDecodeOpts::None)?)),
        58 => Ok(ItemComponent::KineticWeapon(KineticWeapon::decode(reader, &NetDecodeOpts::None)?)),
        59 => Ok(ItemComponent::SwingAnimation(SwingAnimation::decode(reader, &NetDecodeOpts::None)?)),
        60 => Ok(ItemComponent::AdditionalTradeCost(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        61 => Ok(ItemComponent::Dye(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        62 => Ok(ItemComponent::ProvidesTrimMaterial(
            decode_id_or_inline_unsupported::<TrimMaterial, _>(reader, "trim material")?,
        )),
        63 => Ok(ItemComponent::OminousBottleAmplifier(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        64 => Ok(ItemComponent::JukeboxPlayable(
            decode_id_or_inline_unsupported::<JukeboxSong, _>(reader, "jukebox song")?,
        )),
        65 => Ok(ItemComponent::ProvidesBannerPatterns(IDSet::decode(reader, &NetDecodeOpts::None)?)),
        66 => Ok(ItemComponent::Recipes(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        67 => Ok(ItemComponent::LodestoneTracker(LodestoneTracker::decode(reader, &NetDecodeOpts::None)?)),
        68 => Ok(ItemComponent::FireworkExplosion(FireworkExplosion::decode(reader, &NetDecodeOpts::None)?)),
        69 => Ok(ItemComponent::Fireworks(Fireworks::decode(reader, &NetDecodeOpts::None)?)),
        70 => unsupported_component("profile"),
        71 => Ok(ItemComponent::NoteBlockSound(String::decode(reader, &NetDecodeOpts::None)?)),
        72 => Ok(ItemComponent::BannerPatterns(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        73 => Ok(ItemComponent::BaseColor(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        74 => Ok(ItemComponent::PotDecorations(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        75 => Ok(ItemComponent::Container(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        76 => Ok(ItemComponent::BlockState(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        77 => Ok(ItemComponent::Bees(LengthPrefixedVec::decode(reader, &NetDecodeOpts::None)?)),
        78 => Ok(ItemComponent::Lock(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        79 => Ok(ItemComponent::ContainerLoot(NbtBlob::decode(reader, &NetDecodeOpts::None)?)),
        80 => Ok(ItemComponent::BreakSound(IdOr::decode(reader, &NetDecodeOpts::None)?)),
        81 => Ok(ItemComponent::SulfurCubeContent(decode_box(reader)?)),
        82 => Ok(ItemComponent::VillagerVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        83 => Ok(ItemComponent::WolfVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        84 => Ok(ItemComponent::WolfSoundVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        85 => Ok(ItemComponent::WolfCollar(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        86 => Ok(ItemComponent::FoxVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        87 => Ok(ItemComponent::SalmonSize(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        88 => Ok(ItemComponent::ParrotVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        89 => Ok(ItemComponent::TropicalFishPattern(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        90 => Ok(ItemComponent::TropicalFishBaseColor(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        91 => Ok(ItemComponent::TropicalFishPatternColor(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        92 => Ok(ItemComponent::MooshroomVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        93 => Ok(ItemComponent::RabbitVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        94 => Ok(ItemComponent::PigVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        95 => Ok(ItemComponent::PigSoundVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        96 => Ok(ItemComponent::CowVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        97 => Ok(ItemComponent::CowSoundVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        98 => Ok(ItemComponent::ChickenVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        99 => Ok(ItemComponent::ChickenSoundVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        100 => Ok(ItemComponent::FrogVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        101 => Ok(ItemComponent::HorseVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        102 => Ok(ItemComponent::PaintingVariant(IdOr::decode(reader, &NetDecodeOpts::None)?)),
        103 => Ok(ItemComponent::LlamaVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        104 => Ok(ItemComponent::AxolotlVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        105 => Ok(ItemComponent::ZombieNautilusVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        106 => Ok(ItemComponent::CatVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        107 => Ok(ItemComponent::CatSoundVariant(VarInt::decode(reader, &NetDecodeOpts::None)?)),
        108 => Ok(ItemComponent::CatCollar(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        109 => Ok(ItemComponent::SheepColor(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        110 => Ok(ItemComponent::ShulkerColor(DyeColor::decode(reader, &NetDecodeOpts::None)?)),
        _ => Err(NetDecodeError::InvalidEnumVariant),
    }
}

fn decode_box<T, R>(reader: &mut R) -> Result<Box<T>, NetDecodeError>
where
    T: NetDecode,
    R: Read,
{
    Ok(Box::new(T::decode(reader, &NetDecodeOpts::None)?))
}

fn decode_id_or_inline_unsupported<T, R>(
    reader: &mut R,
    type_name: &str,
) -> Result<IdOr<T>, NetDecodeError>
where
    R: Read,
{
    match VarInt::decode(reader, &NetDecodeOpts::None)?.0 {
        0 => Err(NetDecodeError::ExternalError(
            format!("decoding inline {type_name} is not implemented yet").into(),
        )),
        id @ 1.. => Ok(IdOr::Id(VarInt::new(id - 1))),
        _ => Err(NetDecodeError::InvalidEnumVariant),
    }
}

fn unsupported_component<T>(component_name: &str) -> Result<T, NetDecodeError> {
    Err(NetDecodeError::ExternalError(
        format!("decoding item component {component_name} is not implemented yet").into(),
    ))
}
