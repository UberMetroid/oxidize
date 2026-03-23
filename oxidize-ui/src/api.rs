use gloo_net::http::Request as GlooRequest;
use oxidize_engine::{Achievement, PlayerState};
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://localhost:7412";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub uuid: String,
    pub state: PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub server_time: i64,
    #[serde(default)]
    pub newly_unlocked_achievements: Vec<AchievementInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementInfo {
    pub achievement: Achievement,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub uuid: String,
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
pub struct GlobalStats {
    pub total_energy: f64,
    pub total_players: i64,
    pub total_solar_sails: i64,
    pub total_plasma_tethers: i64,
    pub total_orbital_mirrors: i64,
}

pub async fn sync_state(
    uuid: &str,
    state: &PlayerState,
) -> Result<SyncResponse, wasm_bindgen::JsValue> {
    let request = SyncRequest {
        uuid: uuid.to_string(),
        state: state.clone(),
    };

    let resp = GlooRequest::post(&format!("{}/api/sync", API_BASE))
        .json(&request)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?
        .send()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    let response: SyncResponse = resp
        .json()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    Ok(response)
}

pub async fn fetch_leaderboard() -> Result<LeaderboardResponse, wasm_bindgen::JsValue> {
    let resp = GlooRequest::get(&format!("{}/api/leaderboard", API_BASE))
        .send()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    let response: LeaderboardResponse = resp
        .json()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    Ok(response)
}

pub async fn fetch_global_stats() -> Result<GlobalStats, wasm_bindgen::JsValue> {
    let resp = GlooRequest::get(&format!("{}/api/global-stats", API_BASE))
        .send()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    let response: GlobalStats = resp
        .json()
        .await
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    Ok(response)
}
