//! Game signals - all reactive state for the game
//! 
//! Creates and holds all the game signals including planet/moon angles,
//! spaceship position, flight state, upgrade effects, and trail/flight
//! path history.

use leptos::*;
use oxidize_engine::PlayerState;

use crate::types::UpgradeEffect;

/// Holds all game signals in a single struct for clean passing
pub struct GameSignals {
    pub state: (ReadSignal<PlayerState>, WriteSignal<PlayerState>),
    pub show_how_to_play: (ReadSignal<bool>, WriteSignal<bool>),
    pub architect_message: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub upgrade_effects: (ReadSignal<Vec<UpgradeEffect>>, WriteSignal<Vec<UpgradeEffect>>),
    pub last_purchase_time: (ReadSignal<u64>, WriteSignal<u64>),
    pub planet_angles: (ReadSignal<Vec<f64>>, WriteSignal<Vec<f64>>),
    pub moon_angles: (ReadSignal<Vec<f64>>, WriteSignal<Vec<f64>>),
    pub spaceship_angle: (ReadSignal<f64>, WriteSignal<f64>),
    pub target_planet_idx: (ReadSignal<Option<usize>>, WriteSignal<Option<usize>>),
    pub is_flying: (ReadSignal<bool>, WriteSignal<bool>),
    pub fly_progress: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_from_x: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_from_y: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_to_x: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_to_y: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_x: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_y: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_vx: (ReadSignal<f64>, WriteSignal<f64>),
    pub fly_vy: (ReadSignal<f64>, WriteSignal<f64>),
    pub planet_offset: (ReadSignal<f64>, WriteSignal<f64>),
    pub trail_positions: (ReadSignal<Vec<(f64, f64, f64)>>, WriteSignal<Vec<(f64, f64, f64)>>),
    pub flight_path: (ReadSignal<Vec<(f64, f64)>>, WriteSignal<Vec<(f64, f64)>>),
    pub flight_prediction: (ReadSignal<Vec<(f64, f64)>>, WriteSignal<Vec<(f64, f64)>>),
    pub is_arriving: (ReadSignal<bool>, WriteSignal<bool>),
    pub arrival_time: (ReadSignal<f64>, WriteSignal<f64>),
}

/// Creates all game signals and returns them as a struct
pub fn create_game_signals(initial_state: PlayerState) -> GameSignals {
    GameSignals {
        state: create_signal(initial_state),
        show_how_to_play: create_signal(false),
        architect_message: create_signal(None),
        upgrade_effects: create_signal(Vec::new()),
        last_purchase_time: create_signal(0u64),
        planet_angles: create_signal(vec![0.0f64; 8]),
        moon_angles: create_signal(vec![0.0f64; 14]),
        spaceship_angle: create_signal(0.0),
        target_planet_idx: create_signal(None),
        is_flying: create_signal(false),
        fly_progress: create_signal(1.0),
        fly_from_x: create_signal(0.0f64),
        fly_from_y: create_signal(0.0f64),
        fly_to_x: create_signal(0.0f64),
        fly_to_y: create_signal(0.0f64),
        fly_x: create_signal(0.0f64),
        fly_y: create_signal(0.0f64),
        fly_vx: create_signal(0.0f64),
        fly_vy: create_signal(0.0f64),
        planet_offset: create_signal(0.0f64),
        trail_positions: create_signal(Vec::new()),
        flight_path: create_signal(Vec::new()),
        flight_prediction: create_signal(Vec::new()),
        is_arriving: create_signal(false),
        arrival_time: create_signal(0.0),
    }
}
