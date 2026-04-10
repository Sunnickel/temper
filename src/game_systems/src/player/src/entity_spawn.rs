use bevy_ecs::prelude::*;
use temper_components::entity_identity::Identity;
use temper_components::player::position::Position;
use temper_components::player::rotation::Rotation;
use temper_entities::bundles::*;
use temper_entities::components::EntityMetadata;
use temper_entities::entity_types::EntityType;
use temper_entities::markers::entity_types::*;
use temper_entities::markers::{HasCollisions, HasGravity, HasWaterDrag};
use temper_messages::{SpawnEntityCommand, SpawnEntityEvent};
use temper_net_runtime::connection::StreamWriter;
use temper_protocol::outgoing::spawn_entity::SpawnEntityPacket;
use temper_state::GlobalStateResource;
use tracing::{error, warn};

/// Macro for spawning ground entities (gravity + collisions + water drag)
macro_rules! spawn_ground_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident, $State:ident, $EType:path) => {{
        let bundle = $Bundle::new($position);
        let chunk = $State
            .world
            .get_or_generate_chunk(
                $position.chunk(),
                temper_core::dimension::Dimension::Overworld,
            )
            .expect("Failed to get or generate chunk");
        chunk.entities.insert(
            bundle.identity.uuid,
            (
                $EType,
                bitcode::serialize(&bundle).expect("Failed to serialize entity bundle"),
            ),
        );
        chunk.mark_dirty();
        let entity = $commands
            .spawn((bundle, $Marker, HasGravity, HasCollisions, HasWaterDrag))
            .id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Macro for spawning flying/swimming entities (collisions only)
macro_rules! spawn_flying_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident, $State:ident, $EType:path) => {{
        let bundle = $Bundle::new($position);
        let chunk = $State
            .world
            .get_or_generate_chunk(
                $position.chunk(),
                temper_core::dimension::Dimension::Overworld,
            )
            .expect("Failed to get or generate chunk");
        chunk.entities.insert(
            bundle.identity.uuid,
            (
                $EType,
                bitcode::serialize(&bundle).expect("Failed to serialize entity bundle"),
            ),
        );
        chunk.mark_dirty();
        let entity = $commands.spawn((bundle, $Marker, HasCollisions)).id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Macro for spawning entities with gravity but no water drag (lava/amphibian creatures)
macro_rules! spawn_gravity_entity {
    ($commands:expr, $position:expr, $Bundle:ident, $Marker:ident, $State:ident, $EType:path) => {{
        let bundle = $Bundle::new($position);
        let chunk = $State
            .world
            .get_or_generate_chunk(
                $position.chunk(),
                temper_core::dimension::Dimension::Overworld,
            )
            .expect("Failed to get or generate chunk");
        chunk.entities.insert(
            bundle.identity.uuid,
            (
                $EType,
                bitcode::serialize(&bundle).expect("Failed to serialize entity bundle"),
            ),
        );
        chunk.mark_dirty();
        let entity = $commands
            .spawn((bundle, $Marker, HasGravity, HasCollisions))
            .id();
        $commands.queue(move |world: &mut World| {
            broadcast_entity_spawn(world, entity);
        });
    }};
}

/// Helper function to broadcast entity spawn packets to all connected players.
///
/// This function queries the entity's components and sends the spawn packet
/// to all players. It's generic and works for any entity type.
///
/// # Arguments
///
/// * `world` - The Bevy world
/// * `entity` - The entity to broadcast
///
/// TODO: This should be removed in favor of the automated sending in the mobs systems
fn broadcast_entity_spawn(world: &mut World, entity: Entity) {
    // Get entity components
    let metadata = match world.get::<EntityMetadata>(entity) {
        Some(m) => m,
        None => {
            error!("Failed to get entity metadata for {:?}", entity);
            return;
        }
    };
    let protocol_id = metadata.protocol_id();

    let identity = match world.get::<Identity>(entity) {
        Some(i) => i,
        None => {
            error!("Failed to get entity identity for {:?}", entity);
            return;
        }
    };

    let position = match world.get::<Position>(entity) {
        Some(p) => p,
        None => {
            error!("Failed to get entity position for {:?}", entity);
            return;
        }
    };

    let rotation = match world.get::<Rotation>(entity) {
        Some(r) => r,
        None => {
            error!("Failed to get entity rotation for {:?}", entity);
            return;
        }
    };

    // Create spawn packet
    let spawn_packet = SpawnEntityPacket::new(
        identity.entity_id,
        identity.uuid.as_u128(),
        protocol_id as i32,
        position,
        rotation,
    );

    // Broadcast to all connected players
    let mut writer_query = world.query::<&StreamWriter>();
    for writer in writer_query.iter(world) {
        if let Err(e) = writer.send_packet_ref(&spawn_packet) {
            error!("Failed to send spawn packet: {:?}", e);
        }
    }
}

/// System that processes spawn commands from messages
pub fn spawn_command_processor(
    mut spawn_commands: MessageReader<SpawnEntityCommand>,
    query: Query<(&Position, &Rotation)>,
    mut spawn_events: MessageWriter<SpawnEntityEvent>,
) {
    // Process all spawn command messages
    for command in spawn_commands.read() {
        // Get player position and rotation
        if let Ok((pos, rot)) = query.get(command.player_entity) {
            // Calculate spawn position 2 blocks in front of the player
            let spawn_pos = pos.offset_forward(rot, 2.0);

            spawn_events.write(SpawnEntityEvent {
                entity_type: command.entity_type,
                position: spawn_pos,
            });
        } else {
            warn!(
                "Failed to get position for entity {:?}",
                command.player_entity
            );
        }
    }
}

/// System that listens for `SpawnEntityEvent` and spawns the entity,
/// then broadcasts the spawn packet.
pub fn handle_spawn_entity(
    mut events: MessageReader<SpawnEntityEvent>,
    mut commands: Commands,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let pos = event.position;
        let state = state.0.clone();
        match event.entity_type {
            // Ground entities (gravity + collisions + water drag)
            EntityType::Pig => {
                spawn_ground_entity!(commands, pos, PigBundle, Pig, state, EntityType::Pig)
            }
            EntityType::Cow => {
                spawn_ground_entity!(commands, pos, CowBundle, Cow, state, EntityType::Cow)
            }
            EntityType::Armadillo => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    ArmadilloBundle,
                    Armadillo,
                    state,
                    EntityType::Armadillo
                )
            }
            EntityType::Camel => {
                spawn_ground_entity!(commands, pos, CamelBundle, Camel, state, EntityType::Camel)
            }
            EntityType::Cat => {
                spawn_ground_entity!(commands, pos, CatBundle, Cat, state, EntityType::Cat)
            }
            EntityType::CaveSpider => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    CaveSpiderBundle,
                    CaveSpider,
                    state,
                    EntityType::CaveSpider
                )
            }
            EntityType::Chicken => spawn_ground_entity!(
                commands,
                pos,
                ChickenBundle,
                Chicken,
                state,
                EntityType::Chicken
            ),
            EntityType::Donkey => spawn_ground_entity!(
                commands,
                pos,
                DonkeyBundle,
                Donkey,
                state,
                EntityType::Donkey
            ),
            EntityType::Enderman => spawn_ground_entity!(
                commands,
                pos,
                EndermanBundle,
                Enderman,
                state,
                EntityType::Enderman
            ),
            EntityType::Fox => {
                spawn_ground_entity!(commands, pos, FoxBundle, Fox, state, EntityType::Fox)
            }
            EntityType::Frog => {
                spawn_ground_entity!(commands, pos, FrogBundle, Frog, state, EntityType::Frog)
            }
            EntityType::Goat => {
                spawn_ground_entity!(commands, pos, GoatBundle, Goat, state, EntityType::Goat)
            }
            EntityType::Horse => {
                spawn_ground_entity!(commands, pos, HorseBundle, Horse, state, EntityType::Horse)
            }
            EntityType::IronGolem => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    IronGolemBundle,
                    IronGolem,
                    state,
                    EntityType::IronGolem
                )
            }
            EntityType::Llama => {
                spawn_ground_entity!(commands, pos, LlamaBundle, Llama, state, EntityType::Llama)
            }
            EntityType::Mooshroom => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    MooshroomBundle,
                    Mooshroom,
                    state,
                    EntityType::Mooshroom
                )
            }
            EntityType::Mule => {
                spawn_ground_entity!(commands, pos, MuleBundle, Mule, state, EntityType::Mule)
            }
            EntityType::Ocelot => spawn_ground_entity!(
                commands,
                pos,
                OcelotBundle,
                Ocelot,
                state,
                EntityType::Ocelot
            ),
            EntityType::Panda => {
                spawn_ground_entity!(commands, pos, PandaBundle, Panda, state, EntityType::Panda)
            }
            EntityType::Piglin => spawn_ground_entity!(
                commands,
                pos,
                PiglinBundle,
                Piglin,
                state,
                EntityType::Piglin
            ),
            EntityType::PolarBear => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    PolarBearBundle,
                    PolarBear,
                    state,
                    EntityType::PolarBear
                )
            }
            EntityType::Rabbit => spawn_ground_entity!(
                commands,
                pos,
                RabbitBundle,
                Rabbit,
                state,
                EntityType::Rabbit
            ),
            EntityType::Sheep => {
                spawn_ground_entity!(commands, pos, SheepBundle, Sheep, state, EntityType::Sheep)
            }
            EntityType::SkeletonHorse => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    SkeletonHorseBundle,
                    SkeletonHorse,
                    state,
                    EntityType::SkeletonHorse
                )
            }
            EntityType::Sniffer => spawn_ground_entity!(
                commands,
                pos,
                SnifferBundle,
                Sniffer,
                state,
                EntityType::Sniffer
            ),
            EntityType::Spider => spawn_ground_entity!(
                commands,
                pos,
                SpiderBundle,
                Spider,
                state,
                EntityType::Spider
            ),
            EntityType::SnowGolem => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    SnowGolemBundle,
                    SnowGolem,
                    state,
                    EntityType::SnowGolem
                )
            }
            EntityType::TraderLlama => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    TraderLlamaBundle,
                    TraderLlama,
                    state,
                    EntityType::TraderLlama
                )
            }
            EntityType::Turtle => spawn_ground_entity!(
                commands,
                pos,
                TurtleBundle,
                Turtle,
                state,
                EntityType::Turtle
            ),
            EntityType::Villager => spawn_ground_entity!(
                commands,
                pos,
                VillagerBundle,
                Villager,
                state,
                EntityType::Villager
            ),
            EntityType::WanderingTrader => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    WanderingTraderBundle,
                    WanderingTrader,
                    state,
                    EntityType::WanderingTrader
                )
            }
            EntityType::Wolf => {
                spawn_ground_entity!(commands, pos, WolfBundle, Wolf, state, EntityType::Wolf)
            }
            EntityType::ZombieHorse => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    ZombieHorseBundle,
                    ZombieHorse,
                    state,
                    EntityType::ZombieHorse
                )
            }
            EntityType::ZombifiedPiglin => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    ZombifiedPiglinBundle,
                    ZombifiedPiglin,
                    state,
                    EntityType::ZombifiedPiglin
                )
            }

            // Flying entities (collisions only)
            EntityType::Allay => {
                spawn_flying_entity!(commands, pos, AllayBundle, Allay, state, EntityType::Allay)
            }
            EntityType::Bat => {
                spawn_flying_entity!(commands, pos, BatBundle, Bat, state, EntityType::Bat)
            }
            EntityType::Bee => {
                spawn_flying_entity!(commands, pos, BeeBundle, Bee, state, EntityType::Bee)
            }
            EntityType::Parrot => spawn_flying_entity!(
                commands,
                pos,
                ParrotBundle,
                Parrot,
                state,
                EntityType::Parrot
            ),

            // Water creatures (collisions only, no gravity/water drag)
            EntityType::Cod => {
                spawn_flying_entity!(commands, pos, CodBundle, Cod, state, EntityType::Cod)
            }
            EntityType::Dolphin => spawn_flying_entity!(
                commands,
                pos,
                DolphinBundle,
                Dolphin,
                state,
                EntityType::Dolphin
            ),
            EntityType::Drowned => spawn_flying_entity!(
                commands,
                pos,
                DrownedBundle,
                Drowned,
                state,
                EntityType::Drowned
            ),
            EntityType::GlowSquid => {
                spawn_flying_entity!(
                    commands,
                    pos,
                    GlowSquidBundle,
                    GlowSquid,
                    state,
                    EntityType::GlowSquid
                )
            }
            EntityType::Pufferfish => {
                spawn_flying_entity!(
                    commands,
                    pos,
                    PufferfishBundle,
                    Pufferfish,
                    state,
                    EntityType::Pufferfish
                )
            }
            EntityType::Salmon => spawn_flying_entity!(
                commands,
                pos,
                SalmonBundle,
                Salmon,
                state,
                EntityType::Salmon
            ),
            EntityType::Squid => {
                spawn_flying_entity!(commands, pos, SquidBundle, Squid, state, EntityType::Squid)
            }
            EntityType::Tadpole => spawn_flying_entity!(
                commands,
                pos,
                TadpoleBundle,
                Tadpole,
                state,
                EntityType::Tadpole
            ),
            EntityType::TropicalFish => {
                spawn_flying_entity!(
                    commands,
                    pos,
                    TropicalFishBundle,
                    TropicalFish,
                    state,
                    EntityType::TropicalFish
                )
            }

            // Special: gravity but no water drag (amphibians, lava creatures)
            EntityType::Axolotl => spawn_gravity_entity!(
                commands,
                pos,
                AxolotlBundle,
                Axolotl,
                state,
                EntityType::Axolotl
            ),
            EntityType::Strider => spawn_gravity_entity!(
                commands,
                pos,
                StriderBundle,
                Strider,
                state,
                EntityType::Strider
            ),
            EntityType::MagmaCube => {
                spawn_gravity_entity!(
                    commands,
                    pos,
                    MagmaCubeBundle,
                    MagmaCube,
                    state,
                    EntityType::MagmaCube
                )
            }
            EntityType::Slime => {
                spawn_gravity_entity!(commands, pos, SlimeBundle, Slime, state, EntityType::Slime)
            }

            // Hostile ground entities
            EntityType::Bogged => spawn_ground_entity!(
                commands,
                pos,
                BoggedBundle,
                Bogged,
                state,
                EntityType::Bogged
            ),
            EntityType::Creaking => spawn_ground_entity!(
                commands,
                pos,
                CreakingBundle,
                Creaking,
                state,
                EntityType::Creaking
            ),
            EntityType::Creeper => spawn_ground_entity!(
                commands,
                pos,
                CreeperBundle,
                Creeper,
                state,
                EntityType::Creeper
            ),
            EntityType::Endermite => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    EndermiteBundle,
                    Endermite,
                    state,
                    EntityType::Endermite
                )
            }
            EntityType::Evoker => spawn_ground_entity!(
                commands,
                pos,
                EvokerBundle,
                Evoker,
                state,
                EntityType::Evoker
            ),
            EntityType::Hoglin => spawn_ground_entity!(
                commands,
                pos,
                HoglinBundle,
                Hoglin,
                state,
                EntityType::Hoglin
            ),
            EntityType::Husk => {
                spawn_ground_entity!(commands, pos, HuskBundle, Husk, state, EntityType::Husk)
            }
            EntityType::PiglinBrute => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    PiglinBruteBundle,
                    PiglinBrute,
                    state,
                    EntityType::PiglinBrute
                )
            }
            EntityType::Pillager => spawn_ground_entity!(
                commands,
                pos,
                PillagerBundle,
                Pillager,
                state,
                EntityType::Pillager
            ),
            EntityType::Ravager => spawn_ground_entity!(
                commands,
                pos,
                RavagerBundle,
                Ravager,
                state,
                EntityType::Ravager
            ),
            EntityType::Silverfish => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    SilverfishBundle,
                    Silverfish,
                    state,
                    EntityType::Silverfish
                )
            }
            EntityType::Skeleton => spawn_ground_entity!(
                commands,
                pos,
                SkeletonBundle,
                Skeleton,
                state,
                EntityType::Skeleton
            ),
            EntityType::Stray => {
                spawn_ground_entity!(commands, pos, StrayBundle, Stray, state, EntityType::Stray)
            }
            EntityType::Vindicator => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    VindicatorBundle,
                    Vindicator,
                    state,
                    EntityType::Vindicator
                )
            }
            EntityType::Warden => spawn_ground_entity!(
                commands,
                pos,
                WardenBundle,
                Warden,
                state,
                EntityType::Warden
            ),
            EntityType::Witch => {
                spawn_ground_entity!(commands, pos, WitchBundle, Witch, state, EntityType::Witch)
            }
            EntityType::WitherSkeleton => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    WitherSkeletonBundle,
                    WitherSkeleton,
                    state,
                    EntityType::WitherSkeleton
                )
            }
            EntityType::Zoglin => spawn_ground_entity!(
                commands,
                pos,
                ZoglinBundle,
                Zoglin,
                state,
                EntityType::Zoglin
            ),
            EntityType::Zombie => spawn_ground_entity!(
                commands,
                pos,
                ZombieBundle,
                Zombie,
                state,
                EntityType::Zombie
            ),
            EntityType::ZombieVillager => {
                spawn_ground_entity!(
                    commands,
                    pos,
                    ZombieVillagerBundle,
                    ZombieVillager,
                    state,
                    EntityType::ZombieVillager
                )
            }
            EntityType::Shulker => spawn_ground_entity!(
                commands,
                pos,
                ShulkerBundle,
                Shulker,
                state,
                EntityType::Shulker
            ),

            // Hostile flying entities
            EntityType::Blaze => {
                spawn_flying_entity!(commands, pos, BlazeBundle, Blaze, state, EntityType::Blaze)
            }
            EntityType::Breeze => spawn_flying_entity!(
                commands,
                pos,
                BreezeBundle,
                Breeze,
                state,
                EntityType::Breeze
            ),
            EntityType::Ghast => {
                spawn_flying_entity!(commands, pos, GhastBundle, Ghast, state, EntityType::Ghast)
            }
            EntityType::Phantom => spawn_flying_entity!(
                commands,
                pos,
                PhantomBundle,
                Phantom,
                state,
                EntityType::Phantom
            ),
            EntityType::Vex => {
                spawn_flying_entity!(commands, pos, VexBundle, Vex, state, EntityType::Vex)
            }

            // Hostile water entities
            EntityType::ElderGuardian => {
                spawn_flying_entity!(
                    commands,
                    pos,
                    ElderGuardianBundle,
                    ElderGuardian,
                    state,
                    EntityType::ElderGuardian
                )
            }
            EntityType::Guardian => spawn_flying_entity!(
                commands,
                pos,
                GuardianBundle,
                Guardian,
                state,
                EntityType::Guardian
            ),
        }
    }
}
