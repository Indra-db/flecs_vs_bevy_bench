#![allow(dead_code)]

use ::shipyard::{IntoIter, UniqueView, View};
use bevy_ecs::schedule::Schedule;
use criterion::{Criterion, criterion_group, criterion_main};
use flecs_ecs::prelude::*;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

pub mod shipyard {
    use shipyard::{
        Component as ShipyardComponent, IntoIter, Unique, UniqueViewMut, View,
        World as ShipyardWorld,
    };
    use std::hint::black_box;

    #[derive(ShipyardComponent, Unique, Clone, Copy)]
    pub struct ShipyardSingleton(pub u32);

    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp1(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp2(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp3(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp4(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp5(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp6(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp7(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp8(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp9(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp10(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp11(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp12(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp13(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp14(pub u32);
    #[derive(ShipyardComponent, Clone, Copy)]
    pub struct ShipyardComp15(pub u32);

    pub fn empty_system_15_components(
        _c1: View<ShipyardComp1>,
        _c2: View<ShipyardComp2>,
        _c3: View<ShipyardComp3>,
        _c4: View<ShipyardComp4>,
        _c5: View<ShipyardComp5>,
        _c6: View<ShipyardComp6>,
        _c7: View<ShipyardComp7>,
        _c8: View<ShipyardComp8>,
        _c9: View<ShipyardComp9>,
        _c10: View<ShipyardComp10>,
        _c11: View<ShipyardComp11>,
        _c12: View<ShipyardComp12>,
        _c13: View<ShipyardComp13>,
        _c14: View<ShipyardComp14>,
        _c15: View<ShipyardComp15>,
    ) {
    }

    pub fn empty_system_5_components(
        _c1: View<ShipyardComp1>,
        _c2: View<ShipyardComp2>,
        _c3: View<ShipyardComp3>,
        _c4: View<ShipyardComp4>,
        _c5: View<ShipyardComp5>,
    ) {
    }

    pub fn iteration_system_15_components(
        mut singleton: UniqueViewMut<ShipyardSingleton>,
        c1: View<ShipyardComp1>,
        c2: View<ShipyardComp2>,
        c3: View<ShipyardComp3>,
        c4: View<ShipyardComp4>,
        c5: View<ShipyardComp5>,
        c6: View<ShipyardComp6>,
        c7: View<ShipyardComp7>,
        c8: View<ShipyardComp8>,
        c9: View<ShipyardComp9>,
        c10: View<ShipyardComp10>,
        c11: View<ShipyardComp11>,
        c12: View<ShipyardComp12>,
        c13: View<ShipyardComp13>,
        c14: View<ShipyardComp14>,
        c15: View<ShipyardComp15>,
    ) {
        (
            &c1, &c2, &c3, &c4, &c5, &c6, &c7, &c8, &c9, &c10, &c11, &c12, &c13, &c14, &c15,
        )
            .iter()
            .for_each(|components| {
                black_box(components);
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
            });
    }

    pub fn world_empty() -> ShipyardWorld {
        ShipyardWorld::new()
    }

    pub fn world_with_entities(entity_count: usize) -> ShipyardWorld {
        let mut world = ShipyardWorld::new();
        for i in 0..entity_count {
            world.add_entity((
                ShipyardComp1(i as u32),
                ShipyardComp2(i as u32),
                ShipyardComp3(i as u32),
                ShipyardComp4(i as u32),
                ShipyardComp5(i as u32),
                ShipyardComp6(i as u32),
                ShipyardComp7(i as u32),
                ShipyardComp8(i as u32),
                ShipyardComp9(i as u32),
                ShipyardComp10(i as u32),
                ShipyardComp11(i as u32),
                ShipyardComp12(i as u32),
                ShipyardComp13(i as u32),
                ShipyardComp14(i as u32),
                ShipyardComp15(i as u32),
            ));
        }
        world
    }
}

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
    use std::hint::black_box;

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

fn bench_empty_system_overhead_15_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("Empty System Overhead (15 Components)");

    let shipyard_world = shipyard::world_empty();
    group.bench_function("Shipyard", |b| {
        b.iter(|| {
            black_box(shipyard_world.run(shipyard::empty_system_15_components));
        })
    });

    let mut bevy_world = bevy::world_empty();
    let mut schedule = Schedule::default();
    schedule.add_systems(bevy::empty_system_15_components);
    group.bench_function("Bevy", |b| {
        b.iter(|| {
            black_box(schedule.run(&mut bevy_world));
        })
    });

    let flecs_world = flecs::empty_system_15_components();
    group.bench_function("Flecs", |b| {
        b.iter(|| {
            black_box(flecs_world.progress());
        })
    });

    group.finish();
}

fn bench_empty_system_overhead_5_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("Empty System Overhead (5 Components)");

    let shipyard_world = shipyard::world_empty();
    group.bench_function("Shipyard", |b| {
        b.iter(|| {
            black_box(shipyard_world.run(shipyard::empty_system_5_components));
        })
    });

    let mut bevy_world = bevy::world_empty();
    let mut schedule = Schedule::default();
    schedule.add_systems(bevy::empty_system_5_components);
    group.bench_function("Bevy", |b| {
        b.iter(|| {
            black_box(schedule.run(&mut bevy_world));
        })
    });

    let flecs_world = flecs::empty_system_5_components();
    group.bench_function("Flecs", |b| {
        b.iter(|| {
            black_box(flecs_world.progress());
        })
    });

    group.finish();
}

fn bench_iteration_with_entities(c: &mut Criterion) {
    let entity_counts = [10000];

    for &entity_count in &entity_counts {
        let mut group =
            c.benchmark_group(format!("System Iteration with {} entities", entity_count));

        // let shipyard_world = shipyard::world_with_entities(entity_count);
        // shipyard_world.add_unique(shipyard::ShipyardSingleton(0));
        // group.bench_function("Shipyard", |b| {
        //     b.iter(|| {
        //         black_box(shipyard_world.run(shipyard::iteration_system_15_components));
        //     })
        // });
        // let shipyard_res = shipyard_world
        //     .borrow::<UniqueView<shipyard::ShipyardSingleton>>()
        //     .unwrap();
        // println!("Shipyard value: {}", shipyard_res.0);

        let flecs_world = flecs::world_with_entities(entity_count);
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

        let mut flecs_iter_count = 0;
        let mut flecs_elapsed = Duration::ZERO;
        group.bench_function("Flecs", |b| {
            b.iter_custom(|iters| {
                flecs_iter_count = iters as u32;
                let start = Instant::now();
                for _ in 0..iters {
                    black_box(s.run());
                }
                flecs_elapsed = start.elapsed();
                flecs_elapsed
            });
        });

        let flecs_res = flecs_world.cloned::<&flecs::FlecsSingleton>();
        let flecs_value = flecs_res.0;
        core::mem::drop(flecs_world);

        let mut bevy_world = bevy::world_with_entities(entity_count);
        bevy_world.insert_resource(bevy::BevySingleton(0));
        let mut schedule = Schedule::default();
        schedule.add_systems(bevy::iteration_system_15_components);

        let mut bevy_iter_count = 0;
        let mut bevy_elapsed = Duration::ZERO;
        group.bench_function("Bevy", |b| {
            b.iter_custom(|iters| {
                bevy_iter_count = iters as u32;
                let start = Instant::now();
                for _ in 0..iters {
                    black_box(schedule.run(&mut bevy_world));
                }
                bevy_elapsed = start.elapsed();
                bevy_elapsed
            });
        });

        let bevy_res = bevy_world.get_resource::<bevy::BevySingleton>().unwrap();
        let bevy_value = bevy_res.0;
        core::mem::drop(bevy_world);

        println!(
            "iter_count bevy: {}, flecs: {}",
            bevy_iter_count, flecs_iter_count
        );
        println!(
            "Flecs value: {} div value: {}",
            flecs_value,
            flecs_value / flecs_iter_count
        );
        println!(
            "Bevy value: {} div value: {}",
            bevy_value,
            bevy_value / bevy_iter_count
        );
        println!(
            "Flecs elapsed: {:?}, Bevy elapsed: {:?}",
            flecs_elapsed, bevy_elapsed
        );
        println!(
            "flecs elapsed div value: {:?}",
            flecs_elapsed / flecs_iter_count
        );
        println!(
            "bevy elapsed div value: {:?}",
            bevy_elapsed / bevy_iter_count
        );

        group.finish();
    }
}

fn bench_iteration_with_entities_queries(c: &mut Criterion) {
    let entity_counts = [10000];

    for &entity_count in &entity_counts {
        let mut group = c.benchmark_group(format!(
            "Query Iteration with {} entities queries",
            entity_count
        ));

        // Shipyard: pure query via borrowed Views (no systems)
        let shipyard_world = shipyard::world_with_entities(entity_count);
        let (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = shipyard_world
            .borrow::<(
                View<shipyard::ShipyardComp1>,
                View<shipyard::ShipyardComp2>,
                View<shipyard::ShipyardComp3>,
                View<shipyard::ShipyardComp4>,
                View<shipyard::ShipyardComp5>,
                View<shipyard::ShipyardComp6>,
                View<shipyard::ShipyardComp7>,
                View<shipyard::ShipyardComp8>,
                View<shipyard::ShipyardComp9>,
                View<shipyard::ShipyardComp10>,
                View<shipyard::ShipyardComp11>,
                View<shipyard::ShipyardComp12>,
                View<shipyard::ShipyardComp13>,
                View<shipyard::ShipyardComp14>,
                View<shipyard::ShipyardComp15>,
            )>()
            .expect("shipyard borrow");

        let mut shipyard_val: u32 = 0;

        group.bench_function("Shipyard (query)", |b| {
            b.iter(|| {
                (
                    &v1, &v2, &v3, &v4, &v5, &v6, &v7, &v8, &v9, &v10, &v11, &v12, &v13, &v14, &v15,
                )
                    .iter()
                    .for_each(|components| {
                        let _ = black_box(components);
                        shipyard_val += components.0.0
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
                    });
            })
        });

        println!("Shipyard value: {}", shipyard_val);

        // Bevy: pure query using World::query (no schedule/systems)
        let mut bevy_world = bevy::world_with_entities(entity_count);
        let mut bevy_query = bevy_world.query::<(
            &bevy::BevyComp1,
            &bevy::BevyComp2,
            &bevy::BevyComp3,
            &bevy::BevyComp4,
            &bevy::BevyComp5,
            &bevy::BevyComp6,
            &bevy::BevyComp7,
            &bevy::BevyComp8,
            &bevy::BevyComp9,
            &bevy::BevyComp10,
            &bevy::BevyComp11,
            &bevy::BevyComp12,
            &bevy::BevyComp13,
            &bevy::BevyComp14,
            &bevy::BevyComp15,
        )>();
        let mut bevy_val: u32 = 0; // Dummy value to ensure query is not optimized away
        group.bench_function("Bevy (query)", |b| {
            b.iter(|| {
                for components in bevy_query.iter(&bevy_world) {
                    black_box((bevy_val, components));
                    bevy_val += components.0.0
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
                }
            })
        });
        println!("Bevy value: {}", bevy_val);

        // Flecs: pure query iteration (table-based to reduce per-entity FFI)
        // Note: This avoids systems/progress; it uses a built query only.
        let flecs_world = flecs::world_with_entities(entity_count);
        let flecs_query = flecs_world
            .query::<(
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
            .set_cached()
            .build();

        let mut flecs_val: u32 = 0;
        group.bench_function("Flecs (query)", |b| {
            b.iter(|| {
                flecs_query.each(|comps| {
                    black_box((flecs_val, comps));

                    flecs_val += comps.0.0
                        + comps.1.0
                        + comps.2.0
                        + comps.3.0
                        + comps.4.0
                        + comps.5.0
                        + comps.6.0
                        + comps.7.0
                        + comps.8.0
                        + comps.9.0
                        + comps.10.0
                        + comps.11.0
                        + comps.12.0
                        + comps.13.0
                        + comps.14.0;
                })
            })
        });

        println!("Flecs value: {}", flecs_val);

        group.finish();
    }
}

criterion_group!(
    benches,
    // bench_empty_system_overhead_15_components,
    // bench_empty_system_overhead_5_components,
    //bench_iteration_with_entities_queries,
    bench_iteration_with_entities,
);
criterion_main!(benches);
