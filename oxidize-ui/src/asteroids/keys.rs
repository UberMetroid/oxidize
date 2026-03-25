//! Keyboard input tracker for Asteroids mode.

use leptos::*;

#[derive(Default, Clone, Copy)]
pub struct Keys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool,
}

pub fn create_key_tracker() -> (ReadSignal<Keys>, WriteSignal<Keys>) {
    create_signal(Keys::default())
}
