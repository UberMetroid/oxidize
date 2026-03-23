use axum::{routing::get, routing::post, Router};

use crate::handlers::{
    get_global_stats, get_leaderboard, get_player, get_player_achievements_handler, health_check,
    sync_player,
};
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/api/sync", post(sync_player))
        .route("/api/player/:uuid", get(get_player))
        .route(
            "/api/player/:uuid/achievements",
            get(get_player_achievements_handler),
        )
        .route("/api/leaderboard", get(get_leaderboard))
        .route("/api/global-stats", get(get_global_stats))
        .with_state(state)
}
