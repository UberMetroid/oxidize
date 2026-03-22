//! Oxidize Game Engine
//!
//! Core game logic for the Oxidize incremental management game.
//!
//! ## Modules
//!
//! - [`types`] - Faction and UpgradeType definitions
//! - [`player`] - PlayerState and game mechanics
//! - [`architect`] - AI commentary system with faction-specific quips
//! - [`quips`] - Faction-specific snarky messages

pub mod architect;
pub mod player;
pub mod quips;
pub mod types;

// Re-export commonly used types for convenience
pub use architect::{generate_quip, Milestone, QuipTrigger};
pub use player::PlayerState;
pub use types::{Faction, UpgradeType};
