use oxidize_engine::{Achievement, Faction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub uuid: String,
    pub state: oxidize_engine::PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub server_time: i64,
    pub newly_unlocked_achievements: Vec<AchievementInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementInfo {
    pub achievement: Achievement,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerResponse {
    pub uuid: String,
    pub faction: Faction,
    pub energy: f64,
    pub total_energy_generated: f64,
    pub solar_sails: u32,
    pub plasma_tethers: u32,
    pub orbital_mirrors: u32,
    pub rank: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub uuid: String,
    pub faction: Faction,
    pub total_energy: f64,
    pub solar_sails: u32,
    pub plasma_tethers: u32,
    pub orbital_mirrors: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStatsResponse {
    pub total_energy: f64,
    pub total_players: i64,
    pub total_solar_sails: i64,
    pub total_plasma_tethers: i64,
    pub total_orbital_mirrors: i64,
}
