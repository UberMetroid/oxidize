//! Criterion benchmarks for oxidize-engine performance-critical functions.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use oxidize_engine::types::{Faction, UpgradeType};
use oxidize_engine::PlayerState;

fn bench_energy_per_second(c: &mut Criterion) {
    let mut state = PlayerState::new(Faction::Orange);
    state.solar_sails = 100;
    state.plasma_tethers = 50;
    state.orbital_mirrors = 20;
    state.dyson_collectors = 5;
    state.quantum_arrays = 2;
    state.stellar_engines = 1;

    let mut group = c.benchmark_group("energy_per_second");
    for size in [1, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0.0;
                for _ in 0..size {
                    sum += black_box(&state).energy_per_second();
                }
                sum
            });
        });
    }
    group.finish();
}

fn bench_tick(c: &mut Criterion) {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 1_000_000.0;
    state.solar_sails = 100;
    state.plasma_tethers = 50;
    state.orbital_mirrors = 20;

    let mut group = c.benchmark_group("tick");
    for size in [1, 10, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut s = state.clone();
                for _ in 0..size {
                    black_box(&mut s).tick(1.0, 1000);
                }
                s.energy
            });
        });
    }
    group.finish();
}

fn bench_buy_upgrade(c: &mut Criterion) {
    let mut group = c.benchmark_group("buy_upgrade");
    for size in [1, 10, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut state = PlayerState::new(Faction::Orange);
                state.energy = 1_000_000_000.0;
                for i in 0..size {
                    state.buy_upgrade(UpgradeType::SolarSail, 1000 + i as u64);
                }
                state.solar_sails
            });
        });
    }
    group.finish();
}

fn bench_calculate_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_cost");
    for size in [1, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0.0;
                for i in 0..size {
                    sum += UpgradeType::SolarSail.calculate_cost(i);
                }
                sum
            });
        });
    }
    group.finish();
}

fn bench_faction_multipliers(c: &mut Criterion) {
    let factions = [
        Faction::Red,
        Faction::Orange,
        Faction::Yellow,
        Faction::Green,
        Faction::Blue,
        Faction::Purple,
    ];

    let mut group = c.benchmark_group("faction_multipliers");
    for size in [1, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut sum = 0.0;
                for _ in 0..size {
                    for faction in factions {
                        sum += oxidize_engine::factions::get_upgrade_multiplier(
                            faction,
                            UpgradeType::OrbitalMirror,
                            10,
                        );
                    }
                }
                sum
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_energy_per_second,
    bench_tick,
    bench_buy_upgrade,
    bench_calculate_cost,
    bench_faction_multipliers
);
criterion_main!(benches);
