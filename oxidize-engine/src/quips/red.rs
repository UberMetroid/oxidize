//! Red faction quips - Aggressively dismissive.

use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "Still breathing, I see. Barely.".to_string(),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => "One more sail. How quaint.".to_string(),
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => "A tether. Acceptable.".to_string(),
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "Mirrors now? Ambitious. Pathetic, but ambitious.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => {
            "100 MW. Adequate for a warm-up.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "1,000 MW. Finally, barely acceptable.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "10,000 MW. You might survive.".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => {
            "100,000 MW. Do not disappoint me.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "Your first sail. Endearing.".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => "A tether. Finally.".to_string(),
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "Mirrors. Bold choice. We shall see.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "Dyson structures? You think you can harness a star.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "Quantum arrays. You reach for infinity.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "A Stellar Engine. You dare challenge the sun itself.".to_string()
        }
        QuipTrigger::FactionChange => "Red. Correct choice. Now prove it.".to_string(),
        QuipTrigger::FirstVisit => "You dare build here? Proceed. I watch.".to_string(),
    }
}
