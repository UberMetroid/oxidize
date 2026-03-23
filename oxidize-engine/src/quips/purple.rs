//! Purple faction quips - Ominously void-like.

use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "The void wonders... will you continue?".to_string(),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => {
            "Interesting. The void notes this purchase.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => {
            "The plasma bends to your will. Noted by the void.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "The mirrors reflect... possibilities. The void observes.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "Dyson structures rise. The void watches with... interest.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "Quantum arrays shimmer. The void trembles... slightly.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "A Stellar Engine burns. Even the void pauses to witness.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => {
            "100 MW. The void extends its acknowledgment.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "1,000 MW. The void is... impressed.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "10,000 MW. Even the void did not expect this.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => {
            "100,000 MW. The void trembles. Slightly.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "The first sail. The void marks it.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => {
            "The first tether. The void accepts.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "The first mirror. The void... approves.".to_string()
        }
        QuipTrigger::FactionChange => "Purple. The void recognizes your choice.".to_string(),
        QuipTrigger::FirstVisit => {
            "The void acknowledges your presence. Build, if you dare.".to_string()
        }
    }
}
