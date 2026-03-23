use axum::{extract::Path, extract::State, Json};
use oxidize_engine::Faction;

use crate::models::{LeaderboardEntry, LeaderboardResponse, PlayerResponse};
use crate::AppState;

pub async fn get_player(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<Json<PlayerResponse>, axum::http::StatusCode> {
    let row = sqlx::query_as::<_, (String, String, f64, u32, u32, u32, i64)>(
        r#"
        SELECT 
            p.uuid, p.faction,
            COALESCE(ps.total_energy_generated, 0),
            COALESCE(ps.solar_sails, 0),
            COALESCE(ps.plasma_tethers, 0),
            COALESCE(ps.orbital_mirrors, 0),
            (SELECT COUNT(*) + 1 FROM player_scores ps2 
             WHERE ps2.total_energy_generated > COALESCE(ps.total_energy_generated, 0)) as rank
        FROM players p
        LEFT JOIN player_scores ps ON p.uuid = ps.player_uuid
        WHERE p.uuid = ?1
        ORDER BY ps.recorded_at DESC
        LIMIT 1
        "#,
    )
    .bind(&uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some((uuid, faction, energy, solar_sails, plasma_tethers, orbital_mirrors, rank)) => {
            Ok(Json(PlayerResponse {
                uuid,
                faction: faction.parse().unwrap_or(Faction::Orange),
                energy,
                total_energy_generated: energy,
                solar_sails,
                plasma_tethers,
                orbital_mirrors,
                rank: Some(rank),
            }))
        }
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

pub async fn get_leaderboard(
    State(state): State<AppState>,
) -> Result<Json<LeaderboardResponse>, axum::http::StatusCode> {
    let rows = sqlx::query_as::<_, (i64, String, String, f64, u32, u32, u32)>(
        r#"
        SELECT 
            ROW_NUMBER() OVER (ORDER BY ps.total_energy_generated DESC) as rank,
            p.uuid,
            p.faction,
            ps.total_energy_generated,
            ps.solar_sails,
            ps.plasma_tethers,
            ps.orbital_mirrors
        FROM players p
        INNER JOIN player_scores ps ON p.uuid = ps.player_uuid
        ORDER BY ps.total_energy_generated DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let entries: Vec<LeaderboardEntry> = rows
        .into_iter()
        .map(
            |(rank, uuid, faction, energy, solar_sails, plasma_tethers, orbital_mirrors)| {
                LeaderboardEntry {
                    rank,
                    uuid,
                    faction: faction.parse().unwrap_or(Faction::Orange),
                    total_energy: energy,
                    solar_sails,
                    plasma_tethers,
                    orbital_mirrors,
                }
            },
        )
        .collect();

    Ok(Json(LeaderboardResponse { entries }))
}
