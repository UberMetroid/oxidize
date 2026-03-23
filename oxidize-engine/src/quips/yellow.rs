//! Yellow faction quips - Overly excited.

use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "YAY! You're still here! I was worried!".to_string(),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => {
            "WOW! Great choice! You're doing AMAZING!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => {
            "TA-DA! Beautiful upgrade! Perfect!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "INCREDIBLE! You absolute STAR!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "DYSON! DYSON! DYSON! THIS IS THE GREATEST THING EVER!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "QUANTUM! WE'RE HYPERDRIVE MODE!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "STELLAR ENGINE!!! THE SUN IS YOURS!!!".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => "100 MW! EVERYONE LOOK!".to_string(),
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "1,000 MW! YOU'RE A SUPERSTAR!!".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "10,000 MW! THIS IS SO exciting!!".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => "100,000 MW! YOU DID IT!!!".to_string(),
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "Your FIRST sail! I'm so proud!!".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => {
            "Your FIRST tether! INCREDIBLE!!".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "Your FIRST mirror! LEGENDARY!!!".to_string()
        }
        QuipTrigger::FactionChange => "Yellow! So bright and happy!".to_string(),
        QuipTrigger::FirstVisit => {
            "YAY! Welcome welcome! This is gonna be SO much fun!".to_string()
        }
    }
}
