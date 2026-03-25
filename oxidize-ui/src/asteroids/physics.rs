pub const THRUST: f64 = 0.15;
pub const ROTATE: f64 = 0.08;
pub const DRAG: f64 = 0.99;

#[derive(Default, Clone, Copy)]
pub struct Ship { pub x: f64, pub y: f64, pub vx: f64, pub vy: f64, pub angle: f64 }
