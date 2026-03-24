//! Types for the Oxidize UI

#[derive(Clone, Debug)]
pub struct LaunchEffect {
    #[allow(dead_code)]
    pub id: u64,
    pub start_time: f64,
    pub progress: f64,
    pub angle: f64,
}

impl LaunchEffect {
    pub fn new(id: u64, angle: f64) -> Self {
        Self {
            id,
            start_time: js_sys::Date::now(),
            progress: 0.0,
            angle,
        }
    }
}
