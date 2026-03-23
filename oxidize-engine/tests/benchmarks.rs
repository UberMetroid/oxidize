//! Benchmarks for oxidize-engine performance-critical functions.

use oxidize_engine::types::{Faction, UpgradeType};
use oxidize_engine::PlayerState;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn bench_energy_per_second_with_many_upgrades() {
        let mut state = PlayerState::new(Faction::Orange);
        state.solar_sails = 100;
        state.plasma_tethers = 50;
        state.orbital_mirrors = 20;
        state.dyson_collectors = 5;
        state.quantum_arrays = 2;
        state.stellar_engines = 1;

        let mut sum = 0.0;
        for _ in 0..1000 {
            sum += black_box(&state).energy_per_second();
        }
        assert!(sum > 0.0);
    }

    #[test]
    fn bench_tick_with_many_upgrades() {
        let mut state = PlayerState::new(Faction::Orange);
        state.energy = 1_000_000.0;
        state.solar_sails = 100;
        state.plasma_tethers = 50;
        state.orbital_mirrors = 20;

        for _ in 0..100 {
            black_box(&mut state).tick(1.0, 1000);
        }
        assert!(state.energy > 1_000_000.0);
    }

    #[test]
    fn bench_buy_upgrade() {
        let mut state = PlayerState::new(Faction::Orange);
        state.energy = 1_000_000_000.0;

        for i in 0..100 {
            black_box(&mut state).buy_upgrade(UpgradeType::SolarSail, 1000 + i as u64);
        }
        assert_eq!(state.solar_sails, 100);
    }

    #[test]
    fn bench_calculate_cost() {
        let mut sum = 0.0;
        for i in 0..100 {
            sum += UpgradeType::SolarSail.calculate_cost(i);
        }
        assert!(sum > 0.0);
    }

    #[test]
    fn bench_faction_multipliers() {
        let factions = [
            Faction::Red,
            Faction::Orange,
            Faction::Yellow,
            Faction::Green,
            Faction::Blue,
            Faction::Purple,
        ];

        let mut sum = 0.0;
        for _ in 0..100 {
            for faction in factions {
                sum += oxidize_engine::factions::get_upgrade_multiplier(
                    faction,
                    UpgradeType::OrbitalMirror,
                    10,
                );
            }
        }
        assert!(sum > 0.0);
    }
}
