//! Tests for player module.

use oxidize_engine::types::{Faction, UpgradeType};
use oxidize_engine::PlayerState;

#[test]
fn test_player_state_new_creates_defaults() {
    let state = PlayerState::new(Faction::Orange);
    assert_eq!(state.faction, Faction::Orange);
    assert_eq!(state.energy, 0.0);
    assert_eq!(state.total_energy_generated, 0.0);
    assert_eq!(state.solar_sails, 0);
    assert_eq!(state.plasma_tethers, 0);
    assert_eq!(state.orbital_mirrors, 0);
    assert_eq!(state.last_sync_time, 0);
}

#[test]
fn test_tick_increases_energy_by_eps_times_delta() {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 100.0;
    state.tick(10.0, 1000);
    assert!((state.energy - 110.0).abs() < 0.001);
    assert!((state.total_energy_generated - 10.0).abs() < 0.001);
}

#[test]
fn test_energy_per_second_base_is_one() {
    let state = PlayerState::new(Faction::Orange);
    assert!((state.energy_per_second() - 1.0).abs() < 0.001);
}

#[test]
fn test_energy_per_second_with_solar_sails() {
    let mut state = PlayerState::new(Faction::Orange);
    state.solar_sails = 5;
    let eps = state.energy_per_second();
    let expected = 1.0 + (5.0 * 1.0 * 1.1);
    assert!((eps - expected).abs() < 0.001);
}

#[test]
fn test_energy_per_second_with_all_upgrades() {
    let mut state = PlayerState::new(Faction::Orange);
    state.solar_sails = 2;
    state.plasma_tethers = 1;
    state.orbital_mirrors = 1;
    let expected = 1.0 + (2.0 * 1.0 * 1.1) + (1.0 * 25.0 * 1.1) + (1.0 * 1000.0 * 1.1);
    assert!((state.energy_per_second() - expected).abs() < 0.001);
}

#[test]
fn test_can_afford_returns_true_when_sufficient_energy() {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 100.0;
    assert!(state.can_afford(UpgradeType::SolarSail));
}

#[test]
fn test_can_afford_returns_false_when_insufficient_energy() {
    let state = PlayerState::new(Faction::Orange);
    assert!(!state.can_afford(UpgradeType::PlasmaTether));
    assert!(!state.can_afford(UpgradeType::OrbitalMirror));
}

#[test]
fn test_buy_upgrade_success_deducts_cost_and_increments_count() {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 100.0;
    let cost = UpgradeType::SolarSail.calculate_cost(0);
    let result = state.buy_upgrade(UpgradeType::SolarSail, 1000);
    assert!(result);
    assert_eq!(state.solar_sails, 1);
    assert!((state.energy - (100.0 - cost)).abs() < 0.001);
}

#[test]
fn test_buy_upgrade_returns_false_when_cannot_afford() {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 5.0;
    let result = state.buy_upgrade(UpgradeType::SolarSail, 1000);
    assert!(!result);
    assert_eq!(state.solar_sails, 0);
}

#[test]
fn test_buy_upgrade_returns_false_when_exactly_at_cost() {
    let mut state = PlayerState::new(Faction::Orange);
    state.energy = 10.0;
    let result = state.buy_upgrade(UpgradeType::SolarSail, 1000);
    assert!(result);
    assert_eq!(state.solar_sails, 1);
}

#[test]
fn test_calculate_cost_is_base_for_zero_owned() {
    let cost = UpgradeType::SolarSail.calculate_cost(0);
    assert!((cost - 10.0).abs() < 0.001);
}

#[test]
fn test_calculate_cost_exponential_scaling() {
    let cost_0 = UpgradeType::SolarSail.calculate_cost(0);
    let cost_1 = UpgradeType::SolarSail.calculate_cost(1);
    let cost_2 = UpgradeType::SolarSail.calculate_cost(2);
    assert!((cost_1 - cost_0 * 1.15).abs() < 0.0001);
    assert!((cost_2 - cost_1 * 1.15).abs() < 0.0001);
}

#[test]
fn test_calculate_cost_different_upgrades() {
    assert!((UpgradeType::SolarSail.calculate_cost(0) - 10.0).abs() < 0.001);
    assert!((UpgradeType::PlasmaTether.calculate_cost(0) - 500.0).abs() < 0.001);
    assert!((UpgradeType::OrbitalMirror.calculate_cost(0) - 15000.0).abs() < 0.001);
}

#[test]
fn test_faction_parsing_valid_values() {
    assert_eq!("red".parse::<Faction>().unwrap(), Faction::Red);
    assert_eq!("orange".parse::<Faction>().unwrap(), Faction::Orange);
    assert_eq!("yellow".parse::<Faction>().unwrap(), Faction::Yellow);
    assert_eq!("green".parse::<Faction>().unwrap(), Faction::Green);
    assert_eq!("blue".parse::<Faction>().unwrap(), Faction::Blue);
    assert_eq!("purple".parse::<Faction>().unwrap(), Faction::Purple);
}

#[test]
fn test_faction_parsing_invalid_defaults_to_orange() {
    assert_eq!("unknown".parse::<Faction>().unwrap(), Faction::Orange);
    assert_eq!("".parse::<Faction>().unwrap(), Faction::Orange);
    assert_eq!("RED".parse::<Faction>().unwrap(), Faction::Orange);
}

#[test]
fn test_faction_as_str() {
    assert_eq!(Faction::Red.as_str(), "red");
    assert_eq!(Faction::Orange.as_str(), "orange");
    assert_eq!(Faction::Yellow.as_str(), "yellow");
    assert_eq!(Faction::Green.as_str(), "green");
    assert_eq!(Faction::Blue.as_str(), "blue");
    assert_eq!(Faction::Purple.as_str(), "purple");
}

#[test]
fn test_count_for_upgrade() {
    let mut state = PlayerState::new(Faction::Orange);
    state.solar_sails = 5;
    state.plasma_tethers = 3;
    state.orbital_mirrors = 2;
    assert_eq!(state.count_for_upgrade(UpgradeType::SolarSail), 5);
    assert_eq!(state.count_for_upgrade(UpgradeType::PlasmaTether), 3);
    assert_eq!(state.count_for_upgrade(UpgradeType::OrbitalMirror), 2);
}

#[test]
fn test_upgrade_type_names() {
    assert_eq!(UpgradeType::SolarSail.name(), "Solar Sail");
    assert_eq!(UpgradeType::PlasmaTether.name(), "Plasma Tether");
    assert_eq!(UpgradeType::OrbitalMirror.name(), "Orbital Mirror");
}

#[test]
fn test_upgrade_type_energy_per_second() {
    assert!((UpgradeType::SolarSail.energy_per_second() - 1.0).abs() < 0.001);
    assert!((UpgradeType::PlasmaTether.energy_per_second() - 25.0).abs() < 0.001);
    assert!((UpgradeType::OrbitalMirror.energy_per_second() - 1000.0).abs() < 0.001);
}

#[test]
fn test_player_state_with_achievements() {
    let state = PlayerState::new(Faction::Blue);
    assert!(state.achievements.is_empty());
}
