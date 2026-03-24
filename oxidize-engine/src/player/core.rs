
use serde::{Deserialize, Serialize};

use crate::types::UpgradeType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerState {
    pub energy: f64,
    pub total_energy_generated: f64,
    pub solar_sails: u32,
    pub plasma_tethers: u32,
    pub orbital_mirrors: u32,
    pub dyson_collectors: u32,
    pub quantum_arrays: u32,
    pub stellar_engines: u32,
    pub last_sync_time: u64,
    pub last_synced_total_energy: f64,
    pub last_purchase_time: u64,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            energy: 0.0,
            total_energy_generated: 0.0,
            solar_sails: 0,
            plasma_tethers: 0,
            orbital_mirrors: 0,
            dyson_collectors: 0,
            quantum_arrays: 0,
            stellar_engines: 0,
            last_sync_time: 0,
            last_synced_total_energy: 0.0,
            last_purchase_time: 0,
        }
    }

    pub fn count_for_upgrade(&self, upgrade: UpgradeType) -> u32 {
        match upgrade {
            UpgradeType::SolarSail => self.solar_sails,
            UpgradeType::PlasmaTether => self.plasma_tethers,
            UpgradeType::OrbitalMirror => self.orbital_mirrors,
            UpgradeType::DysonCollector => self.dyson_collectors,
            UpgradeType::QuantumArray => self.quantum_arrays,
            UpgradeType::StellarEngine => self.stellar_engines,
        }
    }

    pub fn can_afford(&self, upgrade: UpgradeType) -> bool {
        if !upgrade.is_unlocked(self.total_energy_generated) {
            return false;
        }
        let cost = upgrade.calculate_cost(self.count_for_upgrade(upgrade));
        self.energy >= cost
    }

    pub fn buy_upgrade(&mut self, upgrade: UpgradeType, current_time: u64) -> bool {
        if !upgrade.is_unlocked(self.total_energy_generated) {
            return false;
        }
        let cost = upgrade.calculate_cost(self.count_for_upgrade(upgrade));
        if self.energy >= cost {
            self.energy -= cost;
            self.last_purchase_time = current_time;
            match upgrade {
                UpgradeType::SolarSail => self.solar_sails += 1,
                UpgradeType::PlasmaTether => self.plasma_tethers += 1,
                UpgradeType::OrbitalMirror => self.orbital_mirrors += 1,
                UpgradeType::DysonCollector => self.dyson_collectors += 1,
                UpgradeType::QuantumArray => self.quantum_arrays += 1,
                UpgradeType::StellarEngine => self.stellar_engines += 1,
            }
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self, delta_seconds: f64, _current_time: u64) {
        let eps = self.energy_per_second();
        let generated = eps * delta_seconds;

        self.energy += generated;
        self.total_energy_generated += generated;
    }
}
