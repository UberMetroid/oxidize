use oxidize_engine::architect::{Architect, Milestone, QuipTrigger};
use oxidize_engine::quips::generate_quip;
use oxidize_engine::{Faction, PlayerState, UpgradeType};

#[test]
fn test_architect_new_has_empty_milestones() {
    let architect = Architect::new(Faction::Orange);
    assert!(architect.milestones_reached.is_empty());
    assert_eq!(architect.faction, Faction::Orange);
    assert_eq!(architect.last_purchase_time, 0);
}

#[test]
fn test_record_purchase_updates_time() {
    let mut architect = Architect::new(Faction::Red);
    architect.record_purchase(UpgradeType::SolarSail, 1000);
    assert_eq!(architect.last_purchase_time, 1000);
}

#[test]
fn test_record_purchase_adds_milestone() {
    let mut architect = Architect::new(Faction::Blue);
    architect.record_purchase(UpgradeType::SolarSail, 1000);
    assert!(architect
        .milestones_reached
        .contains(&Milestone::FirstSolarSail));

    architect.record_purchase(UpgradeType::PlasmaTether, 2000);
    assert!(architect
        .milestones_reached
        .contains(&Milestone::FirstPlasmaTether));

    architect.record_purchase(UpgradeType::OrbitalMirror, 3000);
    assert!(architect
        .milestones_reached
        .contains(&Milestone::FirstOrbitalMirror));
}

#[test]
fn test_should_trigger_idle_false_when_never_purchased() {
    let architect = Architect::new(Faction::Green);
    assert!(!architect.should_trigger_idle(1000));
}

#[test]
fn test_should_trigger_idle_false_under_60_seconds() {
    let mut architect = Architect::new(Faction::Green);
    architect.record_purchase(UpgradeType::SolarSail, 1000);
    assert!(!architect.should_trigger_idle(1050));
}

#[test]
fn test_should_trigger_idle_true_after_60_seconds() {
    let mut architect = Architect::new(Faction::Green);
    architect.record_purchase(UpgradeType::SolarSail, 1000);
    assert!(architect.should_trigger_idle(61000));
}

#[test]
fn test_check_milestones_energy_levels() {
    let mut architect = Architect::new(Faction::Purple);
    let mut state = PlayerState::new(Faction::Purple);

    state.total_energy_generated = 50.0;
    architect.check_milestones(&state);
    assert!(!architect.milestones_reached.contains(&Milestone::Energy100));

    state.total_energy_generated = 100.0;
    architect.check_milestones(&state);
    assert!(architect.milestones_reached.contains(&Milestone::Energy100));

    state.total_energy_generated = 1000.0;
    architect.check_milestones(&state);
    assert!(architect
        .milestones_reached
        .contains(&Milestone::Energy1000));
}

#[test]
fn test_generate_quip_returns_string() {
    let quip = generate_quip(Faction::Red, QuipTrigger::Idle);
    assert!(!quip.is_empty());
    assert!(quip.contains("breathing") || quip.contains("Still"));
}

#[test]
fn test_generate_quip_different_factions_different_messages() {
    let idle_red = generate_quip(Faction::Red, QuipTrigger::Idle);
    let idle_orange = generate_quip(Faction::Orange, QuipTrigger::Idle);
    let idle_yellow = generate_quip(Faction::Yellow, QuipTrigger::Idle);

    assert_ne!(idle_red, idle_orange);
    assert_ne!(idle_orange, idle_yellow);
}

#[test]
fn test_generate_quip_purchase_includes_upgrade_name() {
    let quip_sail = generate_quip(Faction::Blue, QuipTrigger::Purchase(UpgradeType::SolarSail));
    assert!(quip_sail.contains("sail") || quip_sail.contains("Sail"));

    let quip_tether = generate_quip(
        Faction::Blue,
        QuipTrigger::Purchase(UpgradeType::PlasmaTether),
    );
    assert!(quip_tether.contains("tether") || quip_tether.contains("Tether"));

    let quip_mirror = generate_quip(
        Faction::Blue,
        QuipTrigger::Purchase(UpgradeType::OrbitalMirror),
    );
    assert!(quip_mirror.contains("mirror") || quip_mirror.contains("Mirror"));
}
