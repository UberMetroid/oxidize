
use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "The silence is... peaceful. The energy flows regardless.".to_string(),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => {
            "A wise choice. The wind carries your intent.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => {
            "The plasma yields to your patience.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "The light bends to your will. Impressive.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "Dyson structures. You understand the flow of stellar energy.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "Quantum arrays. You touch the fabric of reality.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "A Stellar Engine. You have become one with the cosmos.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => {
            "100 MW have passed through. Noted.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "1,000 MW. The universe acknowledges.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "10,000 MW. You are becoming one with the sphere.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => {
            "100,000 MW. You have transcended.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "The first sail unfurls. Peace.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => {
            "The first tether connects. Harmony.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "The first mirror aligns. Enlightenment.".to_string()
        }
        QuipTrigger::FactionChange => "Green. You seek balance. Understandable.".to_string(),
        QuipTrigger::FirstVisit => "Welcome, traveler. Build, and find peace.".to_string(),
    }
}
