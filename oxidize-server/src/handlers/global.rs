use axum::extract::State;
use serde::Serialize;

use crate::db::get_player_achievements;
use crate::models::{AchievementInfo, GlobalStatsResponse};
use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub version: String,
}

pub async fn get_global_stats(
    State(state): State<AppState>,
) -> Result<axum::Json<GlobalStatsResponse>, axum::http::StatusCode> {
    let row = sqlx::query_as::<_, (f64, i64, i64, i64, i64)>(
        r#"
        SELECT 
            gs.total_energy_generated,
            gs.total_solar_sails,
            gs.total_plasma_tethers,
            gs.total_orbital_mirrors,
            (SELECT COUNT(*) FROM players) as total_players
        FROM global_sphere gs
        WHERE gs.id = 1
        "#,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::Json(GlobalStatsResponse {
        total_energy: row.0,
        total_players: row.4,
        total_solar_sails: row.1,
        total_plasma_tethers: row.2,
        total_orbital_mirrors: row.3,
    }))
}

pub async fn health_check(State(state): State<AppState>) -> axum::Json<HealthResponse> {
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map(|_| "connected")
        .unwrap_or_else(|_| "disconnected");

    axum::Json(HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn get_player_achievements_handler(
    State(state): State<AppState>,
    axum::extract::Path(uuid): axum::extract::Path<String>,
) -> Result<axum::Json<Vec<AchievementInfo>>, axum::http::StatusCode> {
    let achievements = get_player_achievements(&state.db, &uuid)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let infos: Vec<AchievementInfo> = achievements
        .iter()
        .map(|a| AchievementInfo {
            name: a.name().to_string(),
            description: a.description().to_string(),
            achievement: *a,
        })
        .collect();

    Ok(axum::Json(infos))
}
