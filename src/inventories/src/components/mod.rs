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
use std::io::Write;
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

#[derive(NetEncode)]
pub struct Enchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}

#[derive(NetEncode)]
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

#[derive(NetEncode)]
pub struct CustomModelData {
    pub floats: LengthPrefixedVec<f32>,
    pub flags: LengthPrefixedVec<bool>,
    pub strings: LengthPrefixedVec<String>,
    pub colors: LengthPrefixedVec<i32>,
}

#[derive(NetEncode)]
pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    pub hidden_components: LengthPrefixedVec<VarInt>,
}

#[derive(NetEncode)]
pub struct Food {
    pub nutrition: VarInt,
    pub saturation_modifier: f32,
    pub can_always_eat: bool,
}

#[derive(NetEncode)]
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

#[derive(NetEncode)]
pub struct UseCooldown {
    pub seconds: f32,
    pub cooldown_group: PrefixedOptional<String>,
}

#[derive(NetEncode)]
pub struct UseEffects {
    pub can_sprint: bool,
    pub interact_vibrations: bool,
    pub speed_multiplier: f32,
}

#[derive(NetEncode)]
pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}

#[derive(NetEncode)]
pub struct Weapon {
    pub damage_per_attack: VarInt,
    pub disable_blocking_for: f32,
}

#[derive(NetEncode)]
pub struct Tool {
    pub rules: LengthPrefixedVec<ToolRule>,
    pub default_mining_speed: f32,
    pub damage_per_block: VarInt,
    pub can_destroy_blocks_in_creative: bool,
}

#[derive(NetEncode)]
pub struct ToolRule {
    pub blocks: IDSet,
    pub speed: PrefixedOptional<f32>,
    pub correct_drop_for_blocks: PrefixedOptional<bool>,
}

#[derive(NetEncode)]
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

#[derive(NetEncode)]
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

#[derive(NetEncode)]
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

#[derive(NetEncode)]
pub struct SuspiciousStewEffect {
    pub type_id: VarInt,
    pub duration: VarInt,
}

#[derive(NetEncode)]
pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<i32>,
    pub custom_effects: LengthPrefixedVec<PotionEffect>,
    pub custom_name: PrefixedOptional<String>,
}

#[derive(NetEncode)]
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

#[derive(NetEncode)]
pub struct EntityData {
    pub entity_type: VarInt,
    pub data: NbtBlob,
}

#[derive(NetEncode)]
pub struct BlockEntityData {
    pub block_entity_type: VarInt,
    pub data: NbtBlob,
}

#[derive(NetEncode)]
pub struct PiercingWeapon {
    pub deals_knockback: bool,
    pub dismounts: bool,
    pub sound: PrefixedOptional<SoundEvent>,
    pub hit_sound: PrefixedOptional<SoundEvent>,
}

#[derive(NetEncode)]
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

#[derive(NetEncode)]
pub struct BlockStateProperty {
    pub name: String,
    pub value: String,
}

#[derive(NetEncode)]
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
