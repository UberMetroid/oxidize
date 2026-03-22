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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeType {
    SolarSail,
    PlasmaTether,
    OrbitalMirror,
}

impl UpgradeType {
    pub fn base_cost(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 10.0,
            UpgradeType::PlasmaTether => 500.0,
            UpgradeType::OrbitalMirror => 15000.0,
        }
    }

    pub fn energy_per_second(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 1.0,
            UpgradeType::PlasmaTether => 25.0,
            UpgradeType::OrbitalMirror => 1000.0,
        }
    }

    pub fn cost_multiplier(&self) -> f64 {
        match self {
            UpgradeType::SolarSail => 1.15,
            UpgradeType::PlasmaTether => 1.15,
            UpgradeType::OrbitalMirror => 1.15,
        }
    }

    pub fn calculate_cost(&self, current_owned: u32) -> f64 {
        self.base_cost() * self.cost_multiplier().powi(current_owned as i32)
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::SolarSail => "Solar Sail",
            UpgradeType::PlasmaTether => "Plasma Tether",
            UpgradeType::OrbitalMirror => "Orbital Mirror",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            UpgradeType::SolarSail => "Cheap, fragile sheets of reflective metamaterial. Gathers ambient radiation.",
            UpgradeType::PlasmaTether => "Magnetic conduits siphoning direct solar wind. Highly efficient.",
            UpgradeType::OrbitalMirror => "Massive geometric structures focusing the star's output into pure raw capital.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerState {
    pub faction: Faction,
    pub energy: f64,
    pub total_energy_generated: f64,
    pub solar_sails: u32,
    pub plasma_tethers: u32,
    pub orbital_mirrors: u32,
    pub last_sync_time: u64,
}

impl PlayerState {
    pub fn new(faction: Faction) -> Self {
        Self {
            faction,
            energy: 0.0,
            total_energy_generated: 0.0,
            solar_sails: 0,
            plasma_tethers: 0,
            orbital_mirrors: 0,
            last_sync_time: 0,
        }
    }

    pub fn energy_per_second(&self) -> f64 {
        let mut eps = 1.0; // Base generation of 1 MW/s just for existing
        eps += (self.solar_sails as f64) * UpgradeType::SolarSail.energy_per_second();
        eps += (self.plasma_tethers as f64) * UpgradeType::PlasmaTether.energy_per_second();
        eps += (self.orbital_mirrors as f64) * UpgradeType::OrbitalMirror.energy_per_second();
        eps
    }

    pub fn tick(&mut self, delta_seconds: f64) {
        let generated = self.energy_per_second() * delta_seconds;
        self.energy += generated;
        self.total_energy_generated += generated;
    }

    pub fn count_for_upgrade(&self, upgrade: UpgradeType) -> u32 {
        match upgrade {
            UpgradeType::SolarSail => self.solar_sails,
            UpgradeType::PlasmaTether => self.plasma_tethers,
            UpgradeType::OrbitalMirror => self.orbital_mirrors,
        }
    }

    pub fn can_afford(&self, upgrade: UpgradeType) -> bool {
        self.energy >= upgrade.calculate_cost(self.count_for_upgrade(upgrade))
    }

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
