
use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => format!("Idle duration: {} seconds. Efficiency: suboptimal.", 60),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => {
            "Solar sail acquired. Cost-benefit ratio: acceptable.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => {
            "Plasma tether operational. Output increased by 25 MW/s.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "Orbital mirror deployed. 1000 MW/s capacity added.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "Dyson Collector logged. Stellar harvesting capacity: significant.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "Quantum Array activated. Zero-point extraction: nominal.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "Stellar Engine mounted. Stellar output: 100,000,000 MW/s. Efficiency: optimal."
                .to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => {
            "Energy threshold: 100 MW. Proceeding to next phase.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "Energy threshold: 1,000 MW. Performance: satisfactory.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "Energy threshold: 10,000 MW. Efficiency metrics: impressive.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => {
            "Energy threshold: 100,000 MW. Analysis: exceptional.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "Solar sail milestone logged. Catalog updated.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => {
            "Plasma tether milestone logged. Database updated.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "Orbital mirror milestone logged. Records confirmed.".to_string()
        }
        QuipTrigger::FactionChange => {
            "Faction: Blue. Notation recorded. Analysis mode: engaged.".to_string()
        }
        QuipTrigger::FirstVisit => "System initialized. Analysis commencing. Welcome.".to_string(),
    }
}
