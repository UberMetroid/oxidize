//! Oxidize Game Engine
//!
//! Core game logic for the Oxidize incremental management game.
//!
//! ## Modules
//!
//! - [`types`] - Faction and UpgradeType definitions
//! - [`player`] - PlayerState and game mechanics
//! - [`factions`] - Faction-specific bonuses and mechanics
//! - [`achievements`] - Achievement definitions and tracking
//! - [`architect`] - AI commentary system with faction-specific quips
//! - [`quips`] - Faction-specific snarky messages

pub mod achievements;
pub mod architect;
pub mod factions;
pub mod player;
pub mod quips;
pub mod types;

// Re-export commonly used types for convenience
pub use achievements::Achievement;
pub use architect::{generate_quip, Milestone, QuipTrigger};
pub use factions::{
    calculate_meditation_bonus, get_cost_multiplier, get_offline_multiplier, get_sync_interval,
    get_upgrade_multiplier, FactionInfo,
};
pub use player::PlayerState;
pub use types::{Faction, UpgradeType};
