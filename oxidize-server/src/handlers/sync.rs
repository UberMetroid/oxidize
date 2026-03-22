use axum::{extract::State, Json};
use chrono::Utc;

use crate::AppState;
use crate::models::{SyncRequest, SyncResponse};

pub async fn sync_player(
    State(state): State<AppState>,
    Json(payload): Json<SyncRequest>,
) -> Result<Json<SyncResponse>, axum::http::StatusCode> {
    let now = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO players (uuid, faction, last_seen_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(uuid) DO UPDATE SET
            faction = excluded.faction,
            last_seen_at = excluded.last_seen_at
        "#,
    )
    .bind(&payload.uuid)
    .bind(format!("{:?}", payload.state.faction).to_lowercase())
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"
        INSERT INTO player_scores 
        (player_uuid, total_energy_generated, solar_sails, plasma_tethers, orbital_mirrors, recorded_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
    )
    .bind(&payload.uuid)
    .bind(payload.state.total_energy_generated)
    .bind(payload.state.solar_sails)
    .bind(payload.state.plasma_tethers)
    .bind(payload.state.orbital_mirrors)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let delta_energy = payload.state.total_energy_generated - payload.state.last_synced_total_energy;

    sqlx::query(
        r#"
        UPDATE global_sphere SET
            total_energy_generated = total_energy_generated + ?1,
            total_solar_sails = total_solar_sails + ?2,
            total_plasma_tethers = total_plasma_tethers + ?3,
            total_orbital_mirrors = total_orbital_mirrors + ?4,
            last_updated_at = ?5
        WHERE id = 1
        "#,
    )
    .bind(delta_energy)
    .bind(payload.state.solar_sails)
    .bind(payload.state.plasma_tethers)
    .bind(payload.state.orbital_mirrors)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SyncResponse {
        success: true,
        server_time: now,
    }))
}