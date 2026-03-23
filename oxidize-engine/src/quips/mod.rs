//! Faction-specific snarky commentary for the Architect AI.
//!
//! Each faction has a distinct personality:
//! - Red: Aggressively dismissive
//! - Orange: Enthusiastically supportive  
//! - Yellow: Overly excited
//! - Green: Mystically zen
//! - Blue: Analytically clinical
//! - Purple: Ominously void-like

mod blue;
mod green;
mod orange;
mod purple;
mod red;
mod yellow;

use crate::architect::QuipTrigger;
use crate::Faction;

pub fn generate_quip(faction: Faction, trigger: QuipTrigger) -> String {
    match faction {
        Faction::Red => red::generate(trigger),
        Faction::Orange => orange::generate(trigger),
        Faction::Yellow => yellow::generate(trigger),
        Faction::Green => green::generate(trigger),
        Faction::Blue => blue::generate(trigger),
        Faction::Purple => purple::generate(trigger),
    }
}
