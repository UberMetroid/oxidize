//! Core game types for Oxidize.
//!
//! This module contains the fundamental types that define the game state,
//! including factions and upgrade types.

use serde::{Deserialize, Serialize};

/// Represents the player's chosen faction, affecting UI theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Faction {
    Red,
    #[default]
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Faction {
    /// Returns the lowercase string representation of the faction.
    pub fn as_str(&self) -> &'static str {
        match self {
            Faction::Red => "red",
            Faction::Orange => "orange",
            Faction::Yellow => "yellow",
            Faction::Green => "green",
            Faction::Blue => "blue",
            Faction::Purple => "purple",
        }
    }
}

impl std::str::FromStr for Faction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Faction::Red),
            "orange" => Ok(Faction::Orange),
            "yellow" => Ok(Faction::Yellow),
            "green" => Ok(Faction::Green),
            "blue" => Ok(Faction::Blue),
            "purple" => Ok(Faction::Purple),
            _ => Ok(Faction::Orange),
        }
    }
}

/// Types of upgrades available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeType {
    SolarSail,
    PlasmaTether,
    OrbitalMirror,
}

impl UpgradeType {
    /// Base cost before exponential scaling.
    pub fn base_cost(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 10.0,
            UpgradeType::PlasmaTether => 500.0,
            UpgradeType::OrbitalMirror => 15000.0,
        }
    }

    /// Energy generated per second by this upgrade.
    pub fn energy_per_second(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 1.0,
            UpgradeType::PlasmaTether => 25.0,
            UpgradeType::OrbitalMirror => 1000.0,
        }
    }

    /// Cost multiplier for exponential scaling (1.15 = 15% increase per owned).
    pub fn cost_multiplier(&self) -> f64 {
        1.15
    }

    /// Calculates the cost for purchasing this upgrade given current owned count.
    pub fn calculate_cost(&self, current_owned: u32) -> f64 {
        self.base_cost() * self.cost_multiplier().powi(current_owned as i32)
    }

    /// Display name of the upgrade.
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::SolarSail => "Solar Sail",
            UpgradeType::PlasmaTether => "Plasma Tether",
            UpgradeType::OrbitalMirror => "Orbital Mirror",
        }
    }

    /// Description of the upgrade's lore and function.
    pub fn description(&self) -> &'static str {
        match self {
            UpgradeType::SolarSail => {
                "Cheap, fragile sheets of reflective metamaterial. Gathers ambient radiation."
            }
            UpgradeType::PlasmaTether => {
                "Magnetic conduits siphoning direct solar wind. Highly efficient."
            }
            UpgradeType::OrbitalMirror => {
                "Massive geometric structures focusing the star's output into pure raw capital."
            }
        }
    }
}
