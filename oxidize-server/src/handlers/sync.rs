use axum::{extract::State, Json};
use chrono::Utc;

use crate::db::{get_player_achievements, get_player_streak, insert_player_achievement};
use crate::models::{AchievementInfo, SyncRequest, SyncResponse};
use crate::AppState;

pub async fn sync_player(
    State(state): State<AppState>,
    Json(payload): Json<SyncRequest>,
) -> Result<Json<SyncResponse>, axum::http::StatusCode> {
    let now_millis = Utc::now().timestamp_millis();
    let now_seconds = now_millis / 1000;

    let (consecutive_days, last_seen_at) = get_player_streak(&state.db, &payload.uuid)
        .await
        .unwrap_or((1, 0));

    let days_since_last_seen = if last_seen_at > 0 {
        ((now_seconds - last_seen_at) as f64 / 86400.0).floor() as u32
    } else {
        0
    };

    let new_consecutive_days = match days_since_last_seen {
        0 => consecutive_days,
        1 => consecutive_days + 1,
        _ => 1,
    };

    sqlx::query(
        r#"
        INSERT INTO players (uuid, last_seen_at, consecutive_days)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(uuid) DO UPDATE SET
            last_seen_at = excluded.last_seen_at,
            consecutive_days = excluded.consecutive_days
        "#,
    )
    .bind(&payload.uuid)
    .bind(now_seconds)
    .bind(new_consecutive_days as i64)
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
    .bind(now_seconds)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let delta_energy =
        payload.state.total_energy_generated - payload.state.last_synced_total_energy;

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
    .bind(now_seconds)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing_achievements = get_player_achievements(&state.db, &payload.uuid)
        .await
        .unwrap_or_default();

    let newly_unlocked = oxidize_engine::Achievement::check_unlockables(
        &existing_achievements,
        payload.state.total_energy_generated,
        payload.state.solar_sails,
        payload.state.plasma_tethers,
        payload.state.orbital_mirrors,
        payload.state.dyson_collectors,
        payload.state.quantum_arrays,
        payload.state.stellar_engines,
        payload.state.last_sync_time,
        now_millis as u64,
        new_consecutive_days,
    );

    let achievement_infos: Vec<AchievementInfo> = newly_unlocked
        .iter()
        .map(|a| AchievementInfo {
            name: a.name().to_string(),
            description: a.description().to_string(),
            achievement: *a,
        })
        .collect();

    for achievement in &newly_unlocked {
        if let Err(e) = insert_player_achievement(&state.db, &payload.uuid, achievement).await {
            eprintln!("Failed to insert achievement: {}", e);
        }
    }

    Ok(Json(SyncResponse {
        success: true,
        server_time: now_millis,
        newly_unlocked_achievements: achievement_infos,
    }))
}
