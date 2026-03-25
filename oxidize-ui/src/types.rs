//! Types for the Oxidize UI

use oxidize_engine::UpgradeType;

#[derive(Clone, Debug)]
pub struct UpgradeEffect {
    pub id: u64,
    pub upgrade_type: UpgradeType,
    pub start_time: f64,
    pub progress: f64,
    pub ship_x: f64,
    pub ship_y: f64,
    pub permanent: bool,
}

impl UpgradeEffect {
    pub fn new(id: u64, upgrade_type: UpgradeType, ship_x: f64, ship_y: f64) -> Self {
        let permanent = matches!(upgrade_type, UpgradeType::DysonCollector | UpgradeType::StellarEngine);
        Self {
            id,
            upgrade_type,
            start_time: js_sys::Date::now(),
            progress: 0.0,
            ship_x,
            ship_y,
            permanent,
        }
    }

    pub fn duration_secs(&self) -> f64 {
        match self.upgrade_type {
            UpgradeType::SolarSail => 2.0,
            UpgradeType::PlasmaTether => 1.5,
            UpgradeType::OrbitalMirror => 2.0,
            UpgradeType::DysonCollector => 3.0,
            UpgradeType::QuantumArray => 2.0,
            UpgradeType::StellarEngine => 3.5,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.progress >= 1.0 && !self.permanent
    }
}
