//! Simple architect quips for Asteroids mode.

use oxidize_engine::architect::generate_quip;
use oxidize_engine::architect::QuipTrigger;
use oxidize_engine::UpgradeType;

pub fn quip_for_energy_milestone(_energy: f64) -> String {
    generate_quip(QuipTrigger::Purchase(UpgradeType::SolarSail))
}

pub fn quip_for_upgrade_purchase(_name: &str, _count: u32) -> String {
    generate_quip(QuipTrigger::Purchase(UpgradeType::SolarSail))
}
