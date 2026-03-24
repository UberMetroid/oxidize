
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Achievement {
    FirstSail,
    FirstTether,
    FirstMirror,
    FirstDyson,
    FirstQuantum,
    FirstStellar,
    SailCollector10,
    TetherCollector5,
    MirrorCollector2,
    DysonCollector1,
    QuantumArray1,
    StellarEngine1,
    Energy100,
    Energy1K,
    Energy10K,
    Energy100K,
    Energy1M,
    Energy1B,
    ReturnVisitor,
    ZenMaster,
    Dedicated,
}

impl Achievement {
    pub fn name(&self) -> &'static str {
        match self {
            Achievement::FirstSail => "First Light",
            Achievement::FirstTether => "Plasma Pioneer",
            Achievement::FirstMirror => "Mirror Master",
            Achievement::FirstDyson => "Dyson Dawn",
            Achievement::FirstQuantum => "Quantum Leap",
            Achievement::FirstStellar => "Stellar Pioneer",
            Achievement::SailCollector10 => "Sailor",
            Achievement::TetherCollector5 => "Tether Lord",
            Achievement::MirrorCollector2 => "Mirror Baron",
            Achievement::DysonCollector1 => "Dyson Builder",
            Achievement::QuantumArray1 => "Quantum Architect",
            Achievement::StellarEngine1 => "Stellar Engineer",
            Achievement::Energy100 => "Century",
            Achievement::Energy1K => "Millennium",
            Achievement::Energy10K => "Dekamillennium",
            Achievement::Energy100K => "Hectomegalith",
            Achievement::Energy1M => "Megastar",
            Achievement::Energy1B => "Gigarist",
            Achievement::ReturnVisitor => "Back From The Brink",
            Achievement::ZenMaster => "Meditation Master",
            Achievement::Dedicated => "Dedicated Builder",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Achievement::FirstSail => "Purchase your first Solar Sail",
            Achievement::FirstTether => "Purchase your first Plasma Tether",
            Achievement::FirstMirror => "Purchase your first Orbital Mirror",
            Achievement::FirstDyson => "Purchase your first Dyson Collector",
            Achievement::FirstQuantum => "Purchase your first Quantum Array",
            Achievement::FirstStellar => "Purchase your first Stellar Engine",
            Achievement::SailCollector10 => "Own 10 Solar Sails",
            Achievement::TetherCollector5 => "Own 5 Plasma Tethers",
            Achievement::MirrorCollector2 => "Own 2 Orbital Mirrors",
            Achievement::DysonCollector1 => "Own 1 Dyson Collector",
            Achievement::QuantumArray1 => "Own 1 Quantum Array",
            Achievement::StellarEngine1 => "Own 1 Stellar Engine",
            Achievement::Energy100 => "Generate 100 total MW",
            Achievement::Energy1K => "Generate 1,000 total MW",
            Achievement::Energy10K => "Generate 10,000 total MW",
            Achievement::Energy100K => "Generate 100,000 total MW",
            Achievement::Energy1M => "Generate 1,000,000 total MW",
            Achievement::Energy1B => "Generate 1,000,000,000 total MW",
            Achievement::ReturnVisitor => "Return after 24 hours offline",
            Achievement::ZenMaster => "Return after 7 days offline",
            Achievement::Dedicated => "Play for 7 consecutive days",
        }
    }

    pub fn check_unlockables(
        current_achievements: &std::collections::HashSet<Achievement>,
        total_energy: f64,
        solar_sails: u32,
        plasma_tethers: u32,
        orbital_mirrors: u32,
        dyson_collectors: u32,
        quantum_arrays: u32,
        stellar_engines: u32,
        last_sync_time: u64,
        now: u64,
        consecutive_days: u32,
    ) -> Vec<Achievement> {
        let mut newly_unlocked = Vec::new();

        if !current_achievements.contains(&Achievement::FirstSail) && solar_sails >= 1 {
            newly_unlocked.push(Achievement::FirstSail);
        }
        if !current_achievements.contains(&Achievement::FirstTether) && plasma_tethers >= 1 {
            newly_unlocked.push(Achievement::FirstTether);
        }
        if !current_achievements.contains(&Achievement::FirstMirror) && orbital_mirrors >= 1 {
            newly_unlocked.push(Achievement::FirstMirror);
        }
        if !current_achievements.contains(&Achievement::FirstDyson) && dyson_collectors >= 1 {
            newly_unlocked.push(Achievement::FirstDyson);
        }
        if !current_achievements.contains(&Achievement::FirstQuantum) && quantum_arrays >= 1 {
            newly_unlocked.push(Achievement::FirstQuantum);
        }
        if !current_achievements.contains(&Achievement::FirstStellar) && stellar_engines >= 1 {
            newly_unlocked.push(Achievement::FirstStellar);
        }
        if !current_achievements.contains(&Achievement::SailCollector10) && solar_sails >= 10 {
            newly_unlocked.push(Achievement::SailCollector10);
        }
        if !current_achievements.contains(&Achievement::TetherCollector5) && plasma_tethers >= 5 {
            newly_unlocked.push(Achievement::TetherCollector5);
        }
        if !current_achievements.contains(&Achievement::MirrorCollector2) && orbital_mirrors >= 2 {
            newly_unlocked.push(Achievement::MirrorCollector2);
        }
        if !current_achievements.contains(&Achievement::DysonCollector1) && dyson_collectors >= 1 {
            newly_unlocked.push(Achievement::DysonCollector1);
        }
        if !current_achievements.contains(&Achievement::QuantumArray1) && quantum_arrays >= 1 {
            newly_unlocked.push(Achievement::QuantumArray1);
        }
        if !current_achievements.contains(&Achievement::StellarEngine1) && stellar_engines >= 1 {
            newly_unlocked.push(Achievement::StellarEngine1);
        }
        if !current_achievements.contains(&Achievement::Energy100) && total_energy >= 100.0 {
            newly_unlocked.push(Achievement::Energy100);
        }
        if !current_achievements.contains(&Achievement::Energy1K) && total_energy >= 1_000.0 {
            newly_unlocked.push(Achievement::Energy1K);
        }
        if !current_achievements.contains(&Achievement::Energy10K) && total_energy >= 10_000.0 {
            newly_unlocked.push(Achievement::Energy10K);
        }
        if !current_achievements.contains(&Achievement::Energy100K) && total_energy >= 100_000.0 {
            newly_unlocked.push(Achievement::Energy100K);
        }
        if !current_achievements.contains(&Achievement::Energy1M) && total_energy >= 1_000_000.0 {
            newly_unlocked.push(Achievement::Energy1M);
        }
        if !current_achievements.contains(&Achievement::Energy1B) && total_energy >= 1_000_000_000.0
        {
            newly_unlocked.push(Achievement::Energy1B);
        }

        let hours_since_sync = (now - last_sync_time) as f64 / (1000.0 * 60.0 * 60.0);
        if !current_achievements.contains(&Achievement::ReturnVisitor) && hours_since_sync >= 24.0 {
            newly_unlocked.push(Achievement::ReturnVisitor);
        }
        if !current_achievements.contains(&Achievement::ZenMaster) && hours_since_sync >= 24.0 * 7.0
        {
            newly_unlocked.push(Achievement::ZenMaster);
        }
        if !current_achievements.contains(&Achievement::Dedicated) && consecutive_days >= 7 {
            newly_unlocked.push(Achievement::Dedicated);
        }

        newly_unlocked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_first_sail_unlocks() {
        let achievements: HashSet<Achievement> = HashSet::new();
        let unlocked =
            Achievement::check_unlockables(&achievements, 0.0, 1, 0, 0, 0, 0, 0, 0, 0, 1);
        assert!(unlocked.contains(&Achievement::FirstSail));
    }

    #[test]
    fn test_energy_milestones() {
        let achievements: HashSet<Achievement> = HashSet::new();
        let unlocked =
            Achievement::check_unlockables(&achievements, 150.0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
        assert!(unlocked.contains(&Achievement::Energy100));
        assert!(!unlocked.contains(&Achievement::Energy1K));
    }

    #[test]
    fn test_first_dyson_unlocks() {
        let achievements: HashSet<Achievement> = HashSet::new();
        let unlocked =
            Achievement::check_unlockables(&achievements, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 1);
        assert!(unlocked.contains(&Achievement::FirstDyson));
    }
}
