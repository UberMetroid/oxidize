use axum::extract::State;

use crate::AppState;
use crate::models::GlobalStatsResponse;

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

pub async fn health_check() -> &'static str {
    "Oxidize Server Alive"
}