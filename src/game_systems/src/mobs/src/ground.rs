use bevy_ecs::schedule::Schedule;

crate::define_standard_mob_save_load!(
    armadillo,
    marker = temper_entities::markers::entity_types::Armadillo,
    bundle = temper_entities::ArmadilloBundle,
    entity_type = Armadillo,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    bogged,
    marker = temper_entities::markers::entity_types::Bogged,
    bundle = temper_entities::BoggedBundle,
    entity_type = Bogged,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    camel,
    marker = temper_entities::markers::entity_types::Camel,
    bundle = temper_entities::CamelBundle,
    entity_type = Camel,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    cat,
    marker = temper_entities::markers::entity_types::Cat,
    bundle = temper_entities::CatBundle,
    entity_type = Cat,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    cave_spider,
    marker = temper_entities::markers::entity_types::CaveSpider,
    bundle = temper_entities::CaveSpiderBundle,
    entity_type = CaveSpider,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    chicken,
    marker = temper_entities::markers::entity_types::Chicken,
    bundle = temper_entities::ChickenBundle,
    entity_type = Chicken,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    cow,
    marker = temper_entities::markers::entity_types::Cow,
    bundle = temper_entities::CowBundle,
    entity_type = Cow,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    creaking,
    marker = temper_entities::markers::entity_types::Creaking,
    bundle = temper_entities::CreakingBundle,
    entity_type = Creaking,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    creeper,
    marker = temper_entities::markers::entity_types::Creeper,
    bundle = temper_entities::CreeperBundle,
    entity_type = Creeper,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    donkey,
    marker = temper_entities::markers::entity_types::Donkey,
    bundle = temper_entities::DonkeyBundle,
    entity_type = Donkey,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    enderman,
    marker = temper_entities::markers::entity_types::Enderman,
    bundle = temper_entities::EndermanBundle,
    entity_type = Enderman,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    endermite,
    marker = temper_entities::markers::entity_types::Endermite,
    bundle = temper_entities::EndermiteBundle,
    entity_type = Endermite,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    evoker,
    marker = temper_entities::markers::entity_types::Evoker,
    bundle = temper_entities::EvokerBundle,
    entity_type = Evoker,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    fox,
    marker = temper_entities::markers::entity_types::Fox,
    bundle = temper_entities::FoxBundle,
    entity_type = Fox,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    frog,
    marker = temper_entities::markers::entity_types::Frog,
    bundle = temper_entities::FrogBundle,
    entity_type = Frog,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    goat,
    marker = temper_entities::markers::entity_types::Goat,
    bundle = temper_entities::GoatBundle,
    entity_type = Goat,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    hoglin,
    marker = temper_entities::markers::entity_types::Hoglin,
    bundle = temper_entities::HoglinBundle,
    entity_type = Hoglin,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    horse,
    marker = temper_entities::markers::entity_types::Horse,
    bundle = temper_entities::HorseBundle,
    entity_type = Horse,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    husk,
    marker = temper_entities::markers::entity_types::Husk,
    bundle = temper_entities::HuskBundle,
    entity_type = Husk,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    iron_golem,
    marker = temper_entities::markers::entity_types::IronGolem,
    bundle = temper_entities::IronGolemBundle,
    entity_type = IronGolem,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    llama,
    marker = temper_entities::markers::entity_types::Llama,
    bundle = temper_entities::LlamaBundle,
    entity_type = Llama,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    mooshroom,
    marker = temper_entities::markers::entity_types::Mooshroom,
    bundle = temper_entities::MooshroomBundle,
    entity_type = Mooshroom,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    mule,
    marker = temper_entities::markers::entity_types::Mule,
    bundle = temper_entities::MuleBundle,
    entity_type = Mule,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    ocelot,
    marker = temper_entities::markers::entity_types::Ocelot,
    bundle = temper_entities::OcelotBundle,
    entity_type = Ocelot,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    panda,
    marker = temper_entities::markers::entity_types::Panda,
    bundle = temper_entities::PandaBundle,
    entity_type = Panda,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    pig,
    marker = temper_entities::markers::entity_types::Pig,
    bundle = temper_entities::PigBundle,
    entity_type = Pig,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    piglin,
    marker = temper_entities::markers::entity_types::Piglin,
    bundle = temper_entities::PiglinBundle,
    entity_type = Piglin,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    piglin_brute,
    marker = temper_entities::markers::entity_types::PiglinBrute,
    bundle = temper_entities::PiglinBruteBundle,
    entity_type = PiglinBrute,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    pillager,
    marker = temper_entities::markers::entity_types::Pillager,
    bundle = temper_entities::PillagerBundle,
    entity_type = Pillager,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    polar_bear,
    marker = temper_entities::markers::entity_types::PolarBear,
    bundle = temper_entities::PolarBearBundle,
    entity_type = PolarBear,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    rabbit,
    marker = temper_entities::markers::entity_types::Rabbit,
    bundle = temper_entities::RabbitBundle,
    entity_type = Rabbit,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    ravager,
    marker = temper_entities::markers::entity_types::Ravager,
    bundle = temper_entities::RavagerBundle,
    entity_type = Ravager,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    sheep,
    marker = temper_entities::markers::entity_types::Sheep,
    bundle = temper_entities::SheepBundle,
    entity_type = Sheep,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    shulker,
    marker = temper_entities::markers::entity_types::Shulker,
    bundle = temper_entities::ShulkerBundle,
    entity_type = Shulker,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    silverfish,
    marker = temper_entities::markers::entity_types::Silverfish,
    bundle = temper_entities::SilverfishBundle,
    entity_type = Silverfish,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    skeleton,
    marker = temper_entities::markers::entity_types::Skeleton,
    bundle = temper_entities::SkeletonBundle,
    entity_type = Skeleton,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    skeleton_horse,
    marker = temper_entities::markers::entity_types::SkeletonHorse,
    bundle = temper_entities::SkeletonHorseBundle,
    entity_type = SkeletonHorse,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    sniffer,
    marker = temper_entities::markers::entity_types::Sniffer,
    bundle = temper_entities::SnifferBundle,
    entity_type = Sniffer,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    snow_golem,
    marker = temper_entities::markers::entity_types::SnowGolem,
    bundle = temper_entities::SnowGolemBundle,
    entity_type = SnowGolem,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    spider,
    marker = temper_entities::markers::entity_types::Spider,
    bundle = temper_entities::SpiderBundle,
    entity_type = Spider,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    stray,
    marker = temper_entities::markers::entity_types::Stray,
    bundle = temper_entities::StrayBundle,
    entity_type = Stray,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    trader_llama,
    marker = temper_entities::markers::entity_types::TraderLlama,
    bundle = temper_entities::TraderLlamaBundle,
    entity_type = TraderLlama,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    turtle,
    marker = temper_entities::markers::entity_types::Turtle,
    bundle = temper_entities::TurtleBundle,
    entity_type = Turtle,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    villager,
    marker = temper_entities::markers::entity_types::Villager,
    bundle = temper_entities::VillagerBundle,
    entity_type = Villager,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    vindicator,
    marker = temper_entities::markers::entity_types::Vindicator,
    bundle = temper_entities::VindicatorBundle,
    entity_type = Vindicator,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    wandering_trader,
    marker = temper_entities::markers::entity_types::WanderingTrader,
    bundle = temper_entities::WanderingTraderBundle,
    entity_type = WanderingTrader,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    warden,
    marker = temper_entities::markers::entity_types::Warden,
    bundle = temper_entities::WardenBundle,
    entity_type = Warden,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    witch,
    marker = temper_entities::markers::entity_types::Witch,
    bundle = temper_entities::WitchBundle,
    entity_type = Witch,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    wither_skeleton,
    marker = temper_entities::markers::entity_types::WitherSkeleton,
    bundle = temper_entities::WitherSkeletonBundle,
    entity_type = WitherSkeleton,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    wolf,
    marker = temper_entities::markers::entity_types::Wolf,
    bundle = temper_entities::WolfBundle,
    entity_type = Wolf,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    zoglin,
    marker = temper_entities::markers::entity_types::Zoglin,
    bundle = temper_entities::ZoglinBundle,
    entity_type = Zoglin,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    zombie,
    marker = temper_entities::markers::entity_types::Zombie,
    bundle = temper_entities::ZombieBundle,
    entity_type = Zombie,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    zombie_horse,
    marker = temper_entities::markers::entity_types::ZombieHorse,
    bundle = temper_entities::ZombieHorseBundle,
    entity_type = ZombieHorse,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    zombie_villager,
    marker = temper_entities::markers::entity_types::ZombieVillager,
    bundle = temper_entities::ZombieVillagerBundle,
    entity_type = ZombieVillager,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

crate::define_standard_mob_save_load!(
    zombified_piglin,
    marker = temper_entities::markers::entity_types::ZombifiedPiglin,
    bundle = temper_entities::ZombifiedPiglinBundle,
    entity_type = ZombifiedPiglin,
    runtime_components = (
        temper_entities::markers::HasGravity,
        temper_entities::markers::HasCollisions,
        temper_entities::markers::HasWaterDrag
    )
);

pub fn register_load_systems(schedule: &mut Schedule) {
    schedule.add_systems(load_armadillo);
    schedule.add_systems(load_bogged);
    schedule.add_systems(load_camel);
    schedule.add_systems(load_cat);
    schedule.add_systems(load_cave_spider);
    schedule.add_systems(load_chicken);
    schedule.add_systems(load_cow);
    schedule.add_systems(load_creaking);
    schedule.add_systems(load_creeper);
    schedule.add_systems(load_donkey);
    schedule.add_systems(load_enderman);
    schedule.add_systems(load_endermite);
    schedule.add_systems(load_evoker);
    schedule.add_systems(load_fox);
    schedule.add_systems(load_frog);
    schedule.add_systems(load_goat);
    schedule.add_systems(load_hoglin);
    schedule.add_systems(load_horse);
    schedule.add_systems(load_husk);
    schedule.add_systems(load_iron_golem);
    schedule.add_systems(load_llama);
    schedule.add_systems(load_mooshroom);
    schedule.add_systems(load_mule);
    schedule.add_systems(load_ocelot);
    schedule.add_systems(load_panda);
    schedule.add_systems(load_pig);
    schedule.add_systems(load_piglin);
    schedule.add_systems(load_piglin_brute);
    schedule.add_systems(load_pillager);
    schedule.add_systems(load_polar_bear);
    schedule.add_systems(load_rabbit);
    schedule.add_systems(load_ravager);
    schedule.add_systems(load_sheep);
    schedule.add_systems(load_shulker);
    schedule.add_systems(load_silverfish);
    schedule.add_systems(load_skeleton);
    schedule.add_systems(load_skeleton_horse);
    schedule.add_systems(load_sniffer);
    schedule.add_systems(load_snow_golem);
    schedule.add_systems(load_spider);
    schedule.add_systems(load_stray);
    schedule.add_systems(load_trader_llama);
    schedule.add_systems(load_turtle);
    schedule.add_systems(load_villager);
    schedule.add_systems(load_vindicator);
    schedule.add_systems(load_wandering_trader);
    schedule.add_systems(load_warden);
    schedule.add_systems(load_witch);
    schedule.add_systems(load_wither_skeleton);
    schedule.add_systems(load_wolf);
    schedule.add_systems(load_zoglin);
    schedule.add_systems(load_zombie);
    schedule.add_systems(load_zombie_horse);
    schedule.add_systems(load_zombie_villager);
    schedule.add_systems(load_zombified_piglin);
}

pub fn register_save_systems(schedule: &mut Schedule) {
    schedule.add_systems(save_armadillo);
    schedule.add_systems(save_bogged);
    schedule.add_systems(save_camel);
    schedule.add_systems(save_cat);
    schedule.add_systems(save_cave_spider);
    schedule.add_systems(save_chicken);
    schedule.add_systems(save_cow);
    schedule.add_systems(save_creaking);
    schedule.add_systems(save_creeper);
    schedule.add_systems(save_donkey);
    schedule.add_systems(save_enderman);
    schedule.add_systems(save_endermite);
    schedule.add_systems(save_evoker);
    schedule.add_systems(save_fox);
    schedule.add_systems(save_frog);
    schedule.add_systems(save_goat);
    schedule.add_systems(save_hoglin);
    schedule.add_systems(save_horse);
    schedule.add_systems(save_husk);
    schedule.add_systems(save_iron_golem);
    schedule.add_systems(save_llama);
    schedule.add_systems(save_mooshroom);
    schedule.add_systems(save_mule);
    schedule.add_systems(save_ocelot);
    schedule.add_systems(save_panda);
    schedule.add_systems(save_pig);
    schedule.add_systems(save_piglin);
    schedule.add_systems(save_piglin_brute);
    schedule.add_systems(save_pillager);
    schedule.add_systems(save_polar_bear);
    schedule.add_systems(save_rabbit);
    schedule.add_systems(save_ravager);
    schedule.add_systems(save_sheep);
    schedule.add_systems(save_shulker);
    schedule.add_systems(save_silverfish);
    schedule.add_systems(save_skeleton);
    schedule.add_systems(save_skeleton_horse);
    schedule.add_systems(save_sniffer);
    schedule.add_systems(save_snow_golem);
    schedule.add_systems(save_spider);
    schedule.add_systems(save_stray);
    schedule.add_systems(save_trader_llama);
    schedule.add_systems(save_turtle);
    schedule.add_systems(save_villager);
    schedule.add_systems(save_vindicator);
    schedule.add_systems(save_wandering_trader);
    schedule.add_systems(save_warden);
    schedule.add_systems(save_witch);
    schedule.add_systems(save_wither_skeleton);
    schedule.add_systems(save_wolf);
    schedule.add_systems(save_zoglin);
    schedule.add_systems(save_zombie);
    schedule.add_systems(save_zombie_horse);
    schedule.add_systems(save_zombie_villager);
    schedule.add_systems(save_zombified_piglin);
}
