//! Architect quip generators for reactive feedback

use oxidize_engine::architect::QuipTrigger;

/// Generate reactive quip based on trigger type
pub fn generate_reactive_quip(trigger: QuipTrigger) -> Option<String> {
    match trigger {
        QuipTrigger::Idle => Some(oxidize_engine::architect::generate_quip(trigger)),
        _ => None,
    }
}

/// Quip for energy milestones
pub fn quip_for_energy_milestone(mw: f64) -> String {
    if mw >= 1_000_000_000.0 {
        "You've transcended stellar energy. I'm genuinely impressed.".to_string()
    } else if mw >= 100_000_000.0 {
        "A Stellar Engine. You went full sci-fi. Respect.".to_string()
    } else if mw >= 10_000_000.0 {
        "Quantum-level output. The universe noticed.".to_string()
    } else if mw >= 1_000_000.0 {
        "Megawatt empire achieved. You might survive out here.".to_string()
    } else if mw >= 100_000.0 {
        "Six figures of clean energy. Now we're cooking.".to_string()
    } else if mw >= 10_000.0 {
        "Double digits in kilowatt-seconds. Getting somewhere.".to_string()
    } else if mw >= 1_000.0 {
        "First kilowatt. Not bad for a rookie.".to_string()
    } else {
        "Generating your first real power. Don't waste it.".to_string()
    }
}

/// Quip for visiting planets
pub fn quip_for_planet_visit(planet_idx: usize) -> String {
    match planet_idx {
        0 => "Mercury — closest to the fire. Bold choice.".to_string(),
        1 => "Venus. Lovely, but I wouldn't breathe in.".to_string(),
        2 => "Earth. Home sweet home.".to_string(),
        3 => "Mars. Getting warmer. And colder. You know what I mean.".to_string(),
        4 => "Jupiter. The big leagues. Watch out for that Great Red Spot.".to_string(),
        5 => "Saturn and its rings. Fashionable and powerful.".to_string(),
        6 => "Uranus. The joke writes itself.".to_string(),
        7 => "Neptune. The edge of the system. You're committed.".to_string(),
        _ => "New planet unlocked. Add it to your passport.".to_string(),
    }
}

/// Quip for upgrade purchases
pub fn quip_for_upgrade_purchase(upgrade_type: &str, count: u32) -> String {
    let item = match upgrade_type {
        "SolarSail" => "Solar Sails",
        "PlasmaTether" => "Plasma Tethers",
        "OrbitalMirror" => "Orbital Mirrors",
        "DysonCollector" => "Dyson Collectors",
        "QuantumArray" => "Quantum Arrays",
        "StellarEngine" => "Stellar Engines",
        _ => upgrade_type,
    };
    if count == 1 {
        format!("Your first {} deployed. The beginning of something big.", item)
    } else if count == 10 {
        format!("{} deployments. You really committed to {}.", count, item)
    } else if count == 50 {
        format!("{} of {} active. You're building an empire.", count, item)
    } else if count == 100 {
        format!("{} {} online. The energy is staggering.", count, item)
    } else {
        format!("{} #{} deployed. Keep climbing.", item, count)
    }
}
