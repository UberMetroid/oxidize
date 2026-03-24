
use crate::architect::{Milestone, QuipTrigger};
use crate::UpgradeType;

pub fn generate(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "Hey. You're still there. That's cool I guess.".to_string(),
        QuipTrigger::Purchase(UpgradeType::SolarSail) => {
            "Nice sail upgrade! Very 2000s energy.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::PlasmaTether) => {
            "Ooh, tether game is strong!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror) => {
            "Mirror moves! Main character unlocked.".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::DysonCollector) => {
            "Dyson Collector! You're basically a god now!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::QuantumArray) => {
            "Quantum Array?! This is getting insane!".to_string()
        }
        QuipTrigger::Purchase(UpgradeType::StellarEngine) => {
            "STELLAR ENGINE?! You're literally harnessing the cosmos!".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100) => "100 MW! We're cooking now!".to_string(),
        QuipTrigger::Milestone(Milestone::Energy1000) => {
            "1,000 MW! You're doing great bestie!".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy10000) => {
            "10K MW! This is actually insane".to_string()
        }
        QuipTrigger::Milestone(Milestone::Energy100000) => {
            "100K MW! Okay now you're just showing off".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstSolarSail) => {
            "First sail! This is so exciting!".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstPlasmaTether) => {
            "First tether! You're amazing!".to_string()
        }
        QuipTrigger::Milestone(Milestone::FirstOrbitalMirror) => {
            "First mirror! LEGEND STATUS!".to_string()
        }
        QuipTrigger::FactionChange => "Orange! Vibes are immaculate".to_string(),
        QuipTrigger::FirstVisit => {
            "Hey! Welcome to the sphere. Let's build something cool.".to_string()
        }
    }
}
