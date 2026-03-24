
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{Faction, PlayerState, UpgradeType};

pub use crate::quips::generate_quip;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Milestone {
    FirstSolarSail,
    FirstPlasmaTether,
    FirstOrbitalMirror,
    Energy100,
    Energy1000,
    Energy10000,
    Energy100000,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architect {
    pub faction: Faction,
    pub last_purchase_time: u64,
    pub milestones_reached: HashSet<Milestone>,
}

impl Architect {
    pub fn new(faction: Faction) -> Self {
        Self {
            faction,
            last_purchase_time: 0,
            milestones_reached: HashSet::new(),
        }
    }

    pub fn record_purchase(&mut self, upgrade: UpgradeType, current_time: u64) {
        self.last_purchase_time = current_time;
        match upgrade {
            UpgradeType::SolarSail => {
                self.milestones_reached.insert(Milestone::FirstSolarSail);
            }
            UpgradeType::PlasmaTether => {
                self.milestones_reached.insert(Milestone::FirstPlasmaTether);
            }
            UpgradeType::OrbitalMirror => {
                self.milestones_reached
                    .insert(Milestone::FirstOrbitalMirror);
            }
            UpgradeType::DysonCollector
            | UpgradeType::QuantumArray
            | UpgradeType::StellarEngine => {}
        }
    }

    pub fn check_milestones(&mut self, state: &PlayerState) {
        if state.total_energy_generated >= 100.0 {
            self.milestones_reached.insert(Milestone::Energy100);
        }
        if state.total_energy_generated >= 1000.0 {
            self.milestones_reached.insert(Milestone::Energy1000);
        }
        if state.total_energy_generated >= 10000.0 {
            self.milestones_reached.insert(Milestone::Energy10000);
        }
        if state.total_energy_generated >= 100000.0 {
            self.milestones_reached.insert(Milestone::Energy100000);
        }
    }

    pub fn should_trigger_idle(&self, current_time: u64) -> bool {
        if self.last_purchase_time == 0 {
            return false;
        }
        let seconds_idle = (current_time - self.last_purchase_time) as f64 / 1000.0;
        seconds_idle >= 60.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuipTrigger {
    Idle,
    Purchase(UpgradeType),
    Milestone(Milestone),
    FactionChange,
    FirstVisit,
}
