//! Faction-specific snarky commentary for the Architect AI.
//!
//! Each faction has a distinct personality:
//! - Red: Aggressively dismissive
//! - Orange: Enthusiastically supportive  
//! - Yellow: Overly excited
//! - Green: Mystically zen
//! - Blue: Analytically clinical
//! - Purple: Ominously void-like

use crate::{
    architect::{Milestone, QuipTrigger},
    Faction,
};

/// Generates a snarky quip for the given faction and trigger event.
pub fn generate_quip(faction: Faction, trigger: QuipTrigger) -> String {
    match faction {
        Faction::Red => generate_red_quip(trigger),
        Faction::Orange => generate_orange_quip(trigger),
        Faction::Yellow => generate_yellow_quip(trigger),
        Faction::Green => generate_green_quip(trigger),
        Faction::Blue => generate_blue_quip(trigger),
        Faction::Purple => generate_purple_quip(trigger),
    }
}

pub fn generate_red_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "Still breathing, I see. Barely.".to_string(),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "One more sail. How quaint.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "A tether. Acceptable.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
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
        QuipTrigger::FactionChange => "Red. Correct choice. Now prove it.".to_string(),
        QuipTrigger::FirstVisit => "You dare build here? Proceed. I watch.".to_string(),
    }
}

pub fn generate_orange_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "Hey. You're still there. That's cool I guess.".to_string(),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "Nice sail upgrade! Very 2000s energy.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "Ooh, tether game is strong!".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
            "Mirror moves! Main character unlocked.".to_string()
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

pub fn generate_yellow_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "YAY! You're still here! I was worried!".to_string(),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "WOW! Great choice! You're doing AMAZING!".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "TA-DA! Beautiful upgrade! Perfect!".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
            "INCREDIBLE! You absolute STAR!".to_string()
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

pub fn generate_green_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "The silence is... peaceful. The energy flows regardless.".to_string(),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "A wise choice. The wind carries your intent.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "The plasma yields to your patience.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
            "The light bends to your will. Impressive.".to_string()
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

pub fn generate_blue_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => format!("Idle duration: {} seconds. Efficiency: suboptimal.", 60),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "Solar sail acquired. Cost-benefit ratio: acceptable.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "Plasma tether operational. Output increased by 25 MW/s.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
            "Orbital mirror deployed. 1000 MW/s capacity added.".to_string()
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

pub fn generate_purple_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "The void wonders... will you continue?".to_string(),
        QuipTrigger::Purchase(crate::UpgradeType::SolarSail) => {
            "Interesting. The void notes this purchase.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::PlasmaTether) => {
            "The plasma bends to your will. Noted by the void.".to_string()
        }
        QuipTrigger::Purchase(crate::UpgradeType::OrbitalMirror) => {
            "The mirrors reflect... possibilities. The void observes.".to_string()
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
