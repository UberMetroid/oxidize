
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeType {
    SolarSail,
    PlasmaTether,
    OrbitalMirror,
    DysonCollector,
    QuantumArray,
    StellarEngine,
}

impl UpgradeType {
    pub fn base_cost(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 10.0,
            UpgradeType::PlasmaTether => 500.0,
            UpgradeType::OrbitalMirror => 15000.0,
            UpgradeType::DysonCollector => 500000.0,
            UpgradeType::QuantumArray => 20000000.0,
            UpgradeType::StellarEngine => 1000000000.0,
        }
    }

    pub fn energy_per_second(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 1.0,
            UpgradeType::PlasmaTether => 25.0,
            UpgradeType::OrbitalMirror => 1000.0,
            UpgradeType::DysonCollector => 50000.0,
            UpgradeType::QuantumArray => 2000000.0,
            UpgradeType::StellarEngine => 100000000.0,
        }
    }

    pub fn cost_multiplier(&self) -> f64 {
        1.15
    }

    pub fn unlock_threshold(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 0.0,
            UpgradeType::PlasmaTether => 0.0,
            UpgradeType::OrbitalMirror => 0.0,
            UpgradeType::DysonCollector => 1_000_000.0,
            UpgradeType::QuantumArray => 100_000_000.0,
            UpgradeType::StellarEngine => 10_000_000_000.0,
        }
    }

    pub fn is_unlocked(&self, total_energy_generated: f64) -> bool {
        total_energy_generated >= self.unlock_threshold()
    }

    pub fn calculate_cost(&self, current_owned: u32) -> f64 {
        self.base_cost() * self.cost_multiplier().powi(current_owned as i32)
    }

    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::SolarSail => "Solar Sail",
            UpgradeType::PlasmaTether => "Plasma Tether",
            UpgradeType::OrbitalMirror => "Orbital Mirror",
            UpgradeType::DysonCollector => "Dyson Collector",
            UpgradeType::QuantumArray => "Quantum Array",
            UpgradeType::StellarEngine => "Stellar Engine",
        }
    }

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
            UpgradeType::DysonCollector => {
                "Orbital megastructures harvesting stellar wind directly from the photosphere."
            }
            UpgradeType::QuantumArray => {
                "Quantum coherence arrays that tap into zero-point energy fluctuations."
            }
            UpgradeType::StellarEngine => {
                "Dyson-Harberman engines that siphon a fraction of the star's core output."
            }
        }
    }

    pub fn basic_upgrades() -> Vec<UpgradeType> {
        vec![
            UpgradeType::SolarSail,
            UpgradeType::PlasmaTether,
            UpgradeType::OrbitalMirror,
        ]
    }

    pub fn advanced_upgrades() -> Vec<UpgradeType> {
        vec![
            UpgradeType::DysonCollector,
            UpgradeType::QuantumArray,
            UpgradeType::StellarEngine,
        ]
    }
}
