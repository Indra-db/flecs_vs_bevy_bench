pub mod bevy {
    use bevy_ecs::component::Component as BevyComponent;
    use bevy_ecs::prelude::*;
    use std::hint::black_box;

    #[derive(Resource, Clone, Copy)]
    pub struct BevySingleton(pub u32);

    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp1(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp2(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp3(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp4(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp5(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp6(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp7(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp8(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp9(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp10(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp11(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp12(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp13(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp14(pub u32);
    #[derive(BevyComponent, Clone, Copy)]
    pub struct BevyComp15(pub u32);

    #[derive(Bundle)]
    pub struct BevyBundle {
        c1: BevyComp1,
        c2: BevyComp2,
        c3: BevyComp3,
        c4: BevyComp4,
        c5: BevyComp5,
        c6: BevyComp6,
        c7: BevyComp7,
        c8: BevyComp8,
        c9: BevyComp9,
        c10: BevyComp10,
        c11: BevyComp11,
        c12: BevyComp12,
        c13: BevyComp13,
        c14: BevyComp14,
        c15: BevyComp15,
    }

    pub fn empty_system_15_components(
        _q: Query<(
            &BevyComp1,
            &BevyComp2,
            &BevyComp3,
            &BevyComp4,
            &BevyComp5,
            &BevyComp6,
            &BevyComp7,
            &BevyComp8,
            &BevyComp9,
            &BevyComp10,
            &BevyComp11,
            &BevyComp12,
            &BevyComp13,
            &BevyComp14,
            &BevyComp15,
        )>,
    ) {
    }

    pub fn empty_system_5_components(
        _q: Query<(&BevyComp1, &BevyComp2, &BevyComp3, &BevyComp4, &BevyComp5)>,
    ) {
    }

    pub fn iteration_system_15_components(
        mut singleton: ResMut<BevySingleton>,
        q: Query<(
            &BevyComp1,
            &BevyComp2,
            &BevyComp3,
            &BevyComp4,
            &BevyComp5,
            &BevyComp6,
            &BevyComp7,
            &BevyComp8,
            &BevyComp9,
            &BevyComp10,
            &BevyComp11,
            &BevyComp12,
            &BevyComp13,
            &BevyComp14,
            &BevyComp15,
        )>,
    ) {
        for components in q.iter() {
            singleton.0 += components.0.0
                + components.1.0
                + components.2.0
                + components.3.0
                + components.4.0
                + components.5.0
                + components.6.0
                + components.7.0
                + components.8.0
                + components.9.0
                + components.10.0
                + components.11.0
                + components.12.0
                + components.13.0
                + components.14.0;
            black_box((&singleton, components));
        }
    }

    pub fn world_empty() -> World {
        World::new()
    }

    pub fn world_with_entities(entity_count: usize) -> World {
        let mut world = World::new();
        for i in 0..entity_count {
            // using a bundle isn't fair for the benchmark since it creates less archetypes
            // so it has to query over less tables
            world.spawn((
                BevyComp1(i as u32),
                BevyComp2(i as u32),
                BevyComp3(i as u32),
                BevyComp4(i as u32),
                BevyComp5(i as u32),
                BevyComp6(i as u32),
                BevyComp7(i as u32),
                BevyComp8(i as u32),
                BevyComp9(i as u32),
                BevyComp10(i as u32),
                BevyComp11(i as u32),
                BevyComp12(i as u32),
                BevyComp13(i as u32),
                BevyComp14(i as u32),
                BevyComp15(i as u32),
            ));
        }
        world
    }
}

pub mod flecs {
    use flecs_ecs::prelude::*;

    #[derive(Component, Clone, Copy)]
    pub struct FlecsSingleton(pub u32);

    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp1(pub u32);

    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp2(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp3(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp4(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp5(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp6(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp7(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp8(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp9(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp10(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp11(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp12(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp13(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp14(pub u32);
    #[derive(Component, Clone, Copy)]
    pub struct FlecsComp15(pub u32);

    pub fn empty_system_15_components() -> World {
        let world = World::new();
        world.component::<FlecsComp1>();
        world.component::<FlecsComp2>();
        world.component::<FlecsComp3>();
        world.component::<FlecsComp4>();
        world.component::<FlecsComp5>();
        world.component::<FlecsComp6>();
        world.component::<FlecsComp7>();
        world.component::<FlecsComp8>();
        world.component::<FlecsComp9>();
        world.component::<FlecsComp10>();
        world.component::<FlecsComp11>();
        world.component::<FlecsComp12>();
        world.component::<FlecsComp13>();
        world.component::<FlecsComp14>();
        world.component::<FlecsComp15>();

        world
            .system::<(
                &FlecsComp1,
                &FlecsComp2,
                &FlecsComp3,
                &FlecsComp4,
                &FlecsComp5,
                &FlecsComp6,
                &FlecsComp7,
                &FlecsComp8,
                &FlecsComp9,
                &FlecsComp10,
                &FlecsComp11,
                &FlecsComp12,
                &FlecsComp13,
                &FlecsComp14,
                &FlecsComp15,
            )>()
            .each(|_| {});

        world
    }

    pub fn empty_system_5_components() -> World {
        let world = World::new();
        world.component::<FlecsComp1>();
        world.component::<FlecsComp2>();
        world.component::<FlecsComp3>();
        world.component::<FlecsComp4>();
        world.component::<FlecsComp5>();

        world
            .system::<(
                &FlecsComp1,
                &FlecsComp2,
                &FlecsComp3,
                &FlecsComp4,
                &FlecsComp5,
            )>()
            .each(|_| {});

        world
    }

    pub fn world_with_entities(entity_count: usize) -> World {
        let world = World::new();

        world
            .component::<FlecsSingleton>()
            .add_trait::<flecs::Singleton>();

        world.component::<FlecsComp1>();
        world.component::<FlecsComp2>();
        world.component::<FlecsComp3>();
        world.component::<FlecsComp4>();
        world.component::<FlecsComp5>();
        world.component::<FlecsComp6>();
        world.component::<FlecsComp7>();
        world.component::<FlecsComp8>();
        world.component::<FlecsComp9>();
        world.component::<FlecsComp10>();
        world.component::<FlecsComp11>();
        world.component::<FlecsComp12>();
        world.component::<FlecsComp13>();
        world.component::<FlecsComp14>();
        world.component::<FlecsComp15>();

        for i in 0..entity_count {
            world
                .entity()
                .set(FlecsComp1(i as u32))
                .set(FlecsComp2(i as u32))
                .set(FlecsComp3(i as u32))
                .set(FlecsComp4(i as u32))
                .set(FlecsComp5(i as u32))
                .set(FlecsComp6(i as u32))
                .set(FlecsComp7(i as u32))
                .set(FlecsComp8(i as u32))
                .set(FlecsComp9(i as u32))
                .set(FlecsComp10(i as u32))
                .set(FlecsComp11(i as u32))
                .set(FlecsComp12(i as u32))
                .set(FlecsComp13(i as u32))
                .set(FlecsComp14(i as u32))
                .set(FlecsComp15(i as u32));
        }

        world
    }
}

use std::{hint::black_box, time::Instant};

use bevy_ecs::schedule::Schedule;
use flecs_ecs::prelude::*;

const ENTITY_COUNT: usize = 10_000;
const ITER_COUNT: usize = 1_000;

fn bench_iteration_with_entities() {
    println!("system iteration with {} entities", ENTITY_COUNT);

    let flecs_world = flecs::world_with_entities(ENTITY_COUNT);
    flecs_world.set(flecs::FlecsSingleton(0));

    let s = flecs_world
        .system::<(
            &mut flecs::FlecsSingleton,
            &flecs::FlecsComp1,
            &flecs::FlecsComp2,
            &flecs::FlecsComp3,
            &flecs::FlecsComp4,
            &flecs::FlecsComp5,
            &flecs::FlecsComp6,
            &flecs::FlecsComp7,
            &flecs::FlecsComp8,
            &flecs::FlecsComp9,
            &flecs::FlecsComp10,
            &flecs::FlecsComp11,
            &flecs::FlecsComp12,
            &flecs::FlecsComp13,
            &flecs::FlecsComp14,
            &flecs::FlecsComp15,
        )>()
        .each(
            |(singleton, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15)| {
                singleton.0 += c1.0
                    + c2.0
                    + c3.0
                    + c4.0
                    + c5.0
                    + c6.0
                    + c7.0
                    + c8.0
                    + c9.0
                    + c10.0
                    + c11.0
                    + c12.0
                    + c13.0
                    + c14.0
                    + c15.0;

                black_box((
                    singleton, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15,
                ));
            },
        );

    let start = Instant::now();
    for _ in 0..ITER_COUNT {
        black_box(s.run());
    }
    let flecs_elapsed = start.elapsed();

    let flecs_res = flecs_world.cloned::<&flecs::FlecsSingleton>();
    let flecs_value = flecs_res.0;
    core::mem::drop(flecs_world);

    let mut bevy_world = bevy::world_with_entities(ENTITY_COUNT);
    bevy_world.insert_resource(bevy::BevySingleton(0));
    let mut schedule = Schedule::default();
    schedule.add_systems(bevy::iteration_system_15_components);

    let start = Instant::now();
    for _ in 0..ITER_COUNT {
        black_box(schedule.run(&mut bevy_world));
    }
    let bevy_elapsed = start.elapsed();

    let bevy_res = bevy_world.get_resource::<bevy::BevySingleton>().unwrap();
    let bevy_value = bevy_res.0;
    core::mem::drop(bevy_world);

    println!(
        "Flecs value: {} div value: {}",
        flecs_value,
        flecs_value / ITER_COUNT as u32
    );
    println!(
        "Bevy value: {} div value: {}",
        bevy_value,
        bevy_value / ITER_COUNT as u32
    );
    println!(
        "Flecs elapsed: {:?}, elapsed / iter_count {:?}",
        flecs_elapsed,
        flecs_elapsed / ITER_COUNT as u32
    );
    println!(
        "Bevy elapsed: {:?}, elapsed / iter_count {:?}",
        bevy_elapsed,
        bevy_elapsed / ITER_COUNT as u32
    );
}

fn main() {
    bench_iteration_with_entities();
}
