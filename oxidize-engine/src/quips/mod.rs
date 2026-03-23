//! Neutral snarky commentary for the Architect AI.
//!
//! Since factions are gone, we use a unified personality that's
//! analytically clinical with a touch of zen wisdom.

use crate::architect::QuipTrigger;

pub fn generate_quip(trigger: QuipTrigger) -> String {
    match trigger {
        QuipTrigger::Idle => "The simulation continues its eternal dance. Energy flows like a river through the cosmos.".to_string(),
        QuipTrigger::Purchase(upgrade) => format!("Another {} joins the orbital ballet. The energy harvest intensifies.", upgrade.name()),
        QuipTrigger::Milestone(_) => "A milestone achieved. The energy network grows stronger.".to_string(),
        QuipTrigger::FactionChange => "The simulation adapts to new parameters.".to_string(),
        QuipTrigger::FirstVisit => "Welcome to the energy harvesting simulation.".to_string(),
    }
}
