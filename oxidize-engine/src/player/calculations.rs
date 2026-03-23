//! Energy calculation helpers for Oxidize.

use super::PlayerState;
use crate::types::UpgradeType;

impl PlayerState {
    /// Calculates total energy generation per second.
    pub fn energy_per_second(&self) -> f64 {
        let base_eps = 1.0;
        let sail_eps = (self.solar_sails as f64) * UpgradeType::SolarSail.energy_per_second();
        let tether_eps =
            (self.plasma_tethers as f64) * UpgradeType::PlasmaTether.energy_per_second();
        let mirror_eps =
            (self.orbital_mirrors as f64) * UpgradeType::OrbitalMirror.energy_per_second();
        let dyson_eps =
            (self.dyson_collectors as f64) * UpgradeType::DysonCollector.energy_per_second();
        let quantum_eps =
            (self.quantum_arrays as f64) * UpgradeType::QuantumArray.energy_per_second();
        let stellar_eps =
            (self.stellar_engines as f64) * UpgradeType::StellarEngine.energy_per_second();
        base_eps + sail_eps + tether_eps + mirror_eps + dyson_eps + quantum_eps + stellar_eps
    }

    /// Calculates offline progress.
    pub fn calculate_offline_progress(&mut self, now: u64) {
        if self.last_sync_time > 0 && now > self.last_sync_time {
            let delta_seconds = (now - self.last_sync_time) as f64 / 1000.0;
            let generated = self.energy_per_second() * delta_seconds;
            self.energy += generated;
            self.total_energy_generated += generated;
        }
        self.last_sync_time = now;
    }
}
