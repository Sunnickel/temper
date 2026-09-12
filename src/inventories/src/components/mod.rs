pub mod potion_effect;
mod consume_effects;

use crate::slot::InventorySlot;
use temper_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use temper_codec::net_types::network_position::NetworkPosition;
use temper_codec::net_types::prefixed_optional::PrefixedOptional;
use temper_codec::net_types::var_int::VarInt;
use temper_macros::Discriminant;
use temper_nbt::blob::NbtBlob;
use temper_text::TextComponent;

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

    // not implemented
    CanPlaceOn,
    // not implemented
    CanBreak,

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

    // not implemented
    DamageResistant,

    AttackRange(AttackRange),

    Tool(Tool),

    Weapon(Weapon),
    Enchantable(VarInt),

    Equippable(Equippable),

    // not implemented
    Repairable,

    Glider,
    TooltipStyle(String),

    // not implemented
    DeathProtection,

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

    // not implemented
    Trim,

    DebugStickState(NbtBlob),
    EntityData(EntityData),
    BucketEntityData(NbtBlob),
    BlockEntityData(BlockEntityData),

    // not implemented
    Instrument,

    PiercingWeapon(PiercingWeapon),

    KineticWeapon(KineticWeapon),

    SwingAnimation(SwingAnimation),
    AdditionalTradeCost(VarInt),
    Dye(DyeColor),

    // not implemented
    ProvidesTrimMaterial,

    OminousBottleAmplifier(VarInt),

    // not implemented
    JukeboxPlayable,

    // not implemented
    ProvidesBannerPatterns,

    Recipes(NbtBlob),
    LodestoneTracker(LodestoneTracker),

    // not implemented
    FireworkExplosion,
    Fireworks(Fireworks),

    // not implemented
    Profile,

    NoteBlockSound(String),

    // not implemented
    BannerPatterns,

    BaseColor(DyeColor),
    PotDecorations(LengthPrefixedVec<VarInt>),
    Container(LengthPrefixedVec<InventorySlot>),
    BlockState(LengthPrefixedVec<BlockStateProperty>),
    Bees(LengthPrefixedVec<Bee>),
    Lock(NbtBlob),
    ContainerLoot(NbtBlob),

    // not implemented
    BreakSound,

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

    // not implemented
    PaintingVariant,

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

pub struct Enchantment {
    pub type_id: VarInt,
    pub level: VarInt,
}

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

pub struct CustomModelData {
    pub floats: LengthPrefixedVec<f32>,
    pub flags: LengthPrefixedVec<bool>,
    pub strings: LengthPrefixedVec<String>,
    pub colors: LengthPrefixedVec<i32>,
}

pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    pub hidden_components: LengthPrefixedVec<VarInt>,
}

pub struct Food {
    pub nutrition: VarInt,
    pub saturation_modifier: f32,
    pub can_always_eat: bool,
}

pub struct Consumable {
    pub consume_seconds: f32,
    pub animation: ConsumeAnimation,
    pub has_consume_particles: bool,
    // not implemented
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

pub struct UseCooldown {
    pub seconds: f32,
    pub cooldown_group: PrefixedOptional<String>,
}

pub struct UseEffects {
    pub can_sprint: bool,
    pub interact_vibrations: bool,
    pub speed_multiplier: f32,
}

pub struct AttackRange {
    pub min_reach: f32,
    pub max_reach: f32,
    pub min_creative_reach: f32,
    pub max_creative_reach: f32,
    pub hitbox_margin: f32,
    pub mob_factor: f32,
}

pub struct Weapon {
    pub damage_per_attack: VarInt,
    pub disable_blocking_for: f32,
}

pub struct Tool {
    pub default_mining_speed: f32,
    pub damage_per_block: VarInt,
    pub can_destroy_blocks_in_creative: bool,
    // not implemented
}

pub struct Equippable {
    pub slot: EquippableSlot,
    pub model: PrefixedOptional<String>,
    pub camera_overlay: PrefixedOptional<String>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
    pub can_be_sheared: bool,
    // not implemented
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

pub struct BlocksAttacks {
    pub block_delay_seconds: f32,
    pub disable_cooldown_scale: f32,
    pub item_damage_threshold: f32,
    pub item_damage_base: f32,
    pub item_damage_factor: f32,
    // not implemented
}

#[derive(Discriminant)]
pub enum MapPostProcessing {
    Lock,
    Scale,
}

pub struct SuspiciousStewEffect {
    pub type_id: VarInt,
    pub duration: VarInt,
}

pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<i32>,
    pub custom_name: PrefixedOptional<String>,
    // not implemented
}

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

pub struct EntityData {
    pub entity_type: VarInt,
    pub data: NbtBlob,
}

pub struct BlockEntityData {
    pub block_entity_type: VarInt,
    pub data: NbtBlob,
}

pub struct PiercingWeapon {
    pub deals_knockback: bool,
    pub dismounts: bool,
    // not implemented
}

pub struct KineticWeapon {
    pub contact_cooldown_ticks: VarInt,
    pub delay_ticks: VarInt,
    pub forward_movement: f32,
    pub damage_multiplier: f32,
    // not implemented
}

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

pub struct LodestoneTracker {
    pub has_global_position: bool,
    pub dimension: Option<String>,
    pub position: Option<NetworkPosition>,
    pub tracked: bool,
}

pub struct Fireworks {
    pub flight_duration: VarInt,
    // not implemented
}

pub struct BlockStateProperty {
    pub name: String,
    pub value: String,
}

pub struct Bee {
    pub entity_type: VarInt,
    pub entity_data: NbtBlob,
    pub ticks_in_hive: VarInt,
    pub min_ticks_in_hive: VarInt,
}
