//! Faction definitions and bonuses for Oxidize.
//!
//! Each faction has a unique identity with passive bonuses and special mechanics.
//! Choose wisely - your faction defines your playstyle.

use crate::types::{Faction, UpgradeType};

/// Represents the unique identity and bonuses of each faction.
#[derive(Debug, Clone, Copy)]
pub struct FactionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub color: &'static str,
    pub orbital_mirror_mult: f64,
    pub plasma_tether_mult: f64,
    pub solar_sail_mult: f64,
    pub cost_mult: f64,
    pub offline_mult: f64,
    pub sync_interval_ms: u32,
}

impl FactionInfo {
    /// Returns the FactionInfo for a given faction.
    pub fn from_faction(faction: Faction) -> Self {
        match faction {
            Faction::Red => FactionInfo {
                name: "The Aggressor",
                description: "+25% Orbital Mirrors. Momentum: +5% base EPS per Orbital Mirror owned.",
                color: "#ef4444",
                orbital_mirror_mult: 1.25,
                plasma_tether_mult: 1.0,
                solar_sail_mult: 1.0,
                cost_mult: 1.0,
                offline_mult: 1.0,
                sync_interval_ms: 2000,
            },
            Faction::Orange => FactionInfo {
                name: "The Balanced",
                description: "+10% all generation. Steady: Double-tick every 60s for free generation.",
                color: "#f97316",
                orbital_mirror_mult: 1.10,
                plasma_tether_mult: 1.10,
                solar_sail_mult: 1.10,
                cost_mult: 1.0,
                offline_mult: 1.0,
                sync_interval_ms: 2000,
            },
            Faction::Yellow => FactionInfo {
                name: "The Rush",
                description: "+30% first 5 of each, -10% after. Adrenaline: 1.5s sync rate.",
                color: "#eab308",
                orbital_mirror_mult: 1.30,
                plasma_tether_mult: 1.30,
                solar_sail_mult: 1.30,
                cost_mult: 1.0,
                offline_mult: 1.0,
                sync_interval_ms: 1500,
            },
            Faction::Green => FactionInfo {
                name: "The Zen",
                description: "+50% offline. Meditation: After 2min idle, generates 0.5% max energy/min.",
                color: "#22c55e",
                orbital_mirror_mult: 1.0,
                plasma_tether_mult: 1.0,
                solar_sail_mult: 1.0,
                cost_mult: 1.0,
                offline_mult: 1.50,
                sync_interval_ms: 2000,
            },
            Faction::Blue => FactionInfo {
                name: "The Analyst",
                description: "+5% all output, +15% upgrade costs. Calculator: +3% efficiency per upgrade.",
                color: "#3b82f6",
                orbital_mirror_mult: 1.05,
                plasma_tether_mult: 1.05,
                solar_sail_mult: 1.05,
                cost_mult: 1.15,
                offline_mult: 1.0,
                sync_interval_ms: 2000,
            },
            Faction::Purple => FactionInfo {
                name: "The Void",
                description: "+75% Plasma, -20% Sail, -10% Mirror. Void Touch: Every 10 purchases grants Void Fragments.",
                color: "#a855f7",
                orbital_mirror_mult: 0.90,
                plasma_tether_mult: 1.75,
                solar_sail_mult: 0.80,
                cost_mult: 1.0,
                offline_mult: 1.0,
                sync_interval_ms: 2000,
            },
        }
    }
}

/// Calculates the energy multiplier for a specific upgrade type based on faction.
/// For Yellow faction, early upgrades get bonus, late upgrades get penalty.
/// For Blue faction, each upgrade owned adds +3% efficiency.
pub fn get_upgrade_multiplier(faction: Faction, upgrade: UpgradeType, owned: u32) -> f64 {
    let info = FactionInfo::from_faction(faction);
    let base_mult = match upgrade {
        UpgradeType::SolarSail => info.solar_sail_mult,
        UpgradeType::PlasmaTether => info.plasma_tether_mult,
        UpgradeType::OrbitalMirror => info.orbital_mirror_mult,
        UpgradeType::DysonCollector => 1.0,
        UpgradeType::QuantumArray => 1.0,
        UpgradeType::StellarEngine => 1.0,
    };

    match faction {
        Faction::Yellow => {
            if owned < 5 {
                base_mult * 1.3
            } else {
                base_mult * 0.9
            }
        }
        Faction::Blue => {
            let efficiency_bonus = 1.0 + (owned as f64 * 0.03);
            base_mult * efficiency_bonus
        }
        _ => base_mult,
    }
}

/// Calculates the cost multiplier for an upgrade based on faction.
/// Blue faction pays 15% more for all upgrades.
pub fn get_cost_multiplier(faction: Faction) -> f64 {
    FactionInfo::from_faction(faction).cost_mult
}

/// Calculates the offline progress multiplier based on faction.
/// Green faction gets 50% more offline progress.
pub fn get_offline_multiplier(faction: Faction) -> f64 {
    FactionInfo::from_faction(faction).offline_mult
}

/// Gets the sync interval in milliseconds for the faction.
/// Yellow faction syncs faster (1.5s vs 2s) for more responsive feel.
pub fn get_sync_interval(faction: Faction) -> u32 {
    FactionInfo::from_faction(faction).sync_interval_ms
}

/// Calculates bonus energy from meditation mechanic (Green faction).
/// After 2 minutes of idle, generates 0.5% of max energy per minute.
pub fn calculate_meditation_bonus(current_energy: f64, idle_seconds: f64) -> f64 {
    if idle_seconds < 120.0 {
        return 0.0;
    }
    let bonus_minutes = (idle_seconds - 120.0) / 60.0;
    let max_possible = current_energy * 100.0;
    (max_possible * 0.005).min(bonus_minutes * current_energy * 0.005)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_orbital_mirror_bonus() {
        let mult = get_upgrade_multiplier(Faction::Red, UpgradeType::OrbitalMirror, 10);
        assert!((mult - 1.25).abs() < 0.001);
    }

    #[test]
    fn test_yellow_early_bonus() {
        let mult = get_upgrade_multiplier(Faction::Yellow, UpgradeType::SolarSail, 3);
        assert!((mult - 1.3 * 1.3).abs() < 0.001);
    }

    #[test]
    fn test_yellow_late_penalty() {
        let mult = get_upgrade_multiplier(Faction::Yellow, UpgradeType::SolarSail, 10);
        assert!((mult - 1.3 * 0.9).abs() < 0.001);
    }

    #[test]
    fn test_blue_efficiency_stacks() {
        let mult_5 = get_upgrade_multiplier(Faction::Blue, UpgradeType::SolarSail, 5);
        let mult_10 = get_upgrade_multiplier(Faction::Blue, UpgradeType::SolarSail, 10);
        assert!(mult_10 > mult_5);
    }

    #[test]
    fn test_purple_plasma_bonus() {
        let mult = get_upgrade_multiplier(Faction::Purple, UpgradeType::PlasmaTether, 5);
        assert!((mult - 1.75).abs() < 0.001);
    }

    #[test]
    fn test_green_offline_bonus() {
        let mult = get_offline_multiplier(Faction::Green);
        assert!((mult - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_yellow_sync_faster() {
        let interval = get_sync_interval(Faction::Yellow);
        assert_eq!(interval, 1500);
    }

    #[test]
    fn test_meditation_not_active_early() {
        let bonus = calculate_meditation_bonus(1000.0, 60.0);
        assert!((bonus - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_meditation_active_after_2min() {
        let bonus = calculate_meditation_bonus(1000.0, 180.0);
        assert!(bonus > 0.0);
    }
}
