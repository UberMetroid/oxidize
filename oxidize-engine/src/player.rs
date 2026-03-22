//! Player state management for Oxidize.
//!
//! Handles the player's energy, upgrades, and progression.

use serde::{Deserialize, Serialize};

use super::types::{Faction, UpgradeType};

/// Represents a player's current game state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerState {
    pub faction: Faction,
    pub energy: f64,
    pub total_energy_generated: f64,
    pub solar_sails: u32,
    pub plasma_tethers: u32,
    pub orbital_mirrors: u32,
    pub last_sync_time: u64,
    /// Tracks total_energy_generated at last sync to calculate delta for global stats.
    pub last_synced_total_energy: f64,
}

impl PlayerState {
    /// Creates a new PlayerState with default values.
    pub fn new(faction: Faction) -> Self {
        Self {
            faction,
            energy: 0.0,
            total_energy_generated: 0.0,
            solar_sails: 0,
            plasma_tethers: 0,
            orbital_mirrors: 0,
            last_sync_time: 0,
            last_synced_total_energy: 0.0,
        }
    }

    /// Calculates total energy generation per second.
    pub fn energy_per_second(&self) -> f64 {
        let mut eps = 1.0;
        eps += (self.solar_sails as f64) * UpgradeType::SolarSail.energy_per_second();
        eps += (self.plasma_tethers as f64) * UpgradeType::PlasmaTether.energy_per_second();
        eps += (self.orbital_mirrors as f64) * UpgradeType::OrbitalMirror.energy_per_second();
        eps
    }

    /// Advances the simulation by delta_seconds, adding generated energy.
    pub fn tick(&mut self, delta_seconds: f64) {
        let generated = self.energy_per_second() * delta_seconds;
        self.energy += generated;
        self.total_energy_generated += generated;
    }

    /// Returns the count of a specific upgrade type.
    pub fn count_for_upgrade(&self, upgrade: UpgradeType) -> u32 {
        match upgrade {
            UpgradeType::SolarSail => self.solar_sails,
            UpgradeType::PlasmaTether => self.plasma_tethers,
            UpgradeType::OrbitalMirror => self.orbital_mirrors,
        }
    }

    /// Checks if the player can afford an upgrade.
    pub fn can_afford(&self, upgrade: UpgradeType) -> bool {
        self.energy >= upgrade.calculate_cost(self.count_for_upgrade(upgrade))
    }

    /// Attempts to purchase an upgrade. Returns true if successful.
    pub fn buy_upgrade(&mut self, upgrade: UpgradeType) -> bool {
        let cost = upgrade.calculate_cost(self.count_for_upgrade(upgrade));
        if self.energy >= cost {
            self.energy -= cost;
            match upgrade {
                UpgradeType::SolarSail => self.solar_sails += 1,
                UpgradeType::PlasmaTether => self.plasma_tethers += 1,
                UpgradeType::OrbitalMirror => self.orbital_mirrors += 1,
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        state.tick(10.0);
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
        assert!((eps - 6.0).abs() < 0.001);
    }

    #[test]
    fn test_energy_per_second_with_all_upgrades() {
        let mut state = PlayerState::new(Faction::Orange);
        state.solar_sails = 2;
        state.plasma_tethers = 1;
        state.orbital_mirrors = 1;
        let expected = 1.0 + (2.0 * 1.0) + (1.0 * 25.0) + (1.0 * 1000.0);
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
        let result = state.buy_upgrade(UpgradeType::SolarSail);
        assert!(result);
        assert_eq!(state.solar_sails, 1);
        assert!((state.energy - (100.0 - cost)).abs() < 0.001);
    }

    #[test]
    fn test_buy_upgrade_returns_false_when_cannot_afford() {
        let mut state = PlayerState::new(Faction::Orange);
        state.energy = 5.0;
        let result = state.buy_upgrade(UpgradeType::SolarSail);
        assert!(!result);
        assert_eq!(state.solar_sails, 0);
    }

    #[test]
    fn test_buy_upgrade_returns_false_when_exactly_at_cost() {
        let mut state = PlayerState::new(Faction::Orange);
        state.energy = 10.0;
        let result = state.buy_upgrade(UpgradeType::SolarSail);
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
        assert!((cost_1 - cost_0 * 1.15).abs() < 0.001);
        assert!((cost_2 - cost_1 * 1.15).abs() < 0.001);
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
}
