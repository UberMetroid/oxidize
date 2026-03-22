use axum::{extract::State, Json};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use oxidize_engine::{Faction, PlayerState};

use oxidize_server::handlers::{get_global_stats, get_leaderboard, get_player, sync_player};
use oxidize_server::models::*;
use oxidize_server::AppState;

async fn create_test_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE players (
            uuid TEXT PRIMARY KEY,
            faction TEXT NOT NULL DEFAULT 'orange',
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE player_scores (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            player_uuid TEXT NOT NULL,
            total_energy_generated REAL NOT NULL,
            solar_sails INTEGER NOT NULL DEFAULT 0,
            plasma_tethers INTEGER NOT NULL DEFAULT 0,
            orbital_mirrors INTEGER NOT NULL DEFAULT 0,
            recorded_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            FOREIGN KEY (player_uuid) REFERENCES players(uuid)
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE global_sphere (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            total_energy_generated REAL NOT NULL DEFAULT 0,
            total_solar_sails INTEGER NOT NULL DEFAULT 0,
            total_plasma_tethers INTEGER NOT NULL DEFAULT 0,
            total_orbital_mirrors INTEGER NOT NULL DEFAULT 0,
            last_updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(r#"INSERT INTO global_sphere (id) VALUES (1)"#)
        .execute(&pool)
        .await
        .unwrap();

    pool
}

#[tokio::test]
async fn test_sync_player_creates_record() {
    let pool = create_test_pool().await;
    let pool_verify = pool.clone();
    let state = AppState { db: pool };
    let uuid = "test-player-1";

    let state_obj = PlayerState {
        faction: Faction::Orange,
        energy: 100.0,
        total_energy_generated: 500.0,
        solar_sails: 5,
        plasma_tethers: 1,
        orbital_mirrors: 0,
        last_sync_time: 1234567890,
        last_synced_total_energy: 0.0,
    };

    let request = SyncRequest {
        uuid: uuid.to_string(),
        state: state_obj,
    };

    let response = sync_player(State(state), Json(request)).await;
    assert!(response.is_ok());

    let row: (String, String) =
        sqlx::query_as("SELECT uuid, faction FROM players WHERE uuid = ?1")
            .bind(uuid)
            .fetch_one(&pool_verify)
            .await
            .unwrap();
    assert_eq!(row.0, uuid);
    assert_eq!(row.1, "orange");
}

#[tokio::test]
async fn test_sync_updates_global_sphere() {
    let pool = create_test_pool().await;
    let pool_verify = pool.clone();
    let state = AppState { db: pool };

    let state_obj = PlayerState {
        faction: Faction::Blue,
        energy: 200.0,
        total_energy_generated: 1000.0,
        solar_sails: 10,
        plasma_tethers: 2,
        orbital_mirrors: 1,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
    };

    let request = SyncRequest {
        uuid: "player-global".to_string(),
        state: state_obj,
    };

    let _ = sync_player(State(state), Json(request)).await;

    let row: (f64, i64, i64, i64) = sqlx::query_as(
        "SELECT total_energy_generated, total_solar_sails, total_plasma_tethers, total_orbital_mirrors FROM global_sphere WHERE id = 1",
    )
    .fetch_one(&pool_verify)
    .await
    .unwrap();

    assert!((row.0 - 1000.0).abs() < 0.001);
    assert_eq!(row.1, 10);
    assert_eq!(row.2, 2);
    assert_eq!(row.3, 1);
}

#[tokio::test]
async fn test_get_player_not_found() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let response = get_player(State(state), axum::extract::Path("nonexistent".to_string())).await;
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_player_returns_data() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let state_obj = PlayerState {
        faction: Faction::Purple,
        energy: 300.0,
        total_energy_generated: 1500.0,
        solar_sails: 15,
        plasma_tethers: 3,
        orbital_mirrors: 1,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
    };

    let sync_request = SyncRequest {
        uuid: "player-get-test".to_string(),
        state: state_obj,
    };
    let _ = sync_player(State(state.clone()), Json(sync_request)).await;

    let response = get_player(State(state), axum::extract::Path("player-get-test".to_string())).await;
    assert!(response.is_ok());

    let player = response.unwrap().0;
    assert_eq!(player.uuid, "player-get-test");
    assert_eq!(player.faction, Faction::Purple);
    assert!((player.total_energy_generated - 1500.0).abs() < 0.001);
    assert_eq!(player.solar_sails, 15);
}

#[tokio::test]
async fn test_leaderboard_returns_ordered_entries() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    for i in 0..3 {
        let state_obj = PlayerState {
            faction: Faction::Orange,
            energy: 100.0 * (i + 1) as f64,
            total_energy_generated: 1000.0 * (i + 1) as f64,
            solar_sails: (i + 1) as u32,
            plasma_tethers: 0,
            orbital_mirrors: 0,
            last_sync_time: 0,
            last_synced_total_energy: 0.0,
        };

        let request = SyncRequest {
            uuid: format!("player-{}", i),
            state: state_obj,
        };
        let _ = sync_player(State(state.clone()), Json(request)).await;
    }

    let response = get_leaderboard(State(state)).await;
    assert!(response.is_ok());

    let body = response.unwrap().0;
    assert_eq!(body.entries.len(), 3);
    assert_eq!(body.entries[0].rank, 1);
    assert_eq!(body.entries[1].rank, 2);
    assert_eq!(body.entries[2].rank, 3);
}

#[tokio::test]
async fn test_global_stats_returns_aggregate() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let state_obj = PlayerState {
        faction: Faction::Red,
        energy: 50.0,
        total_energy_generated: 250.0,
        solar_sails: 2,
        plasma_tethers: 1,
        orbital_mirrors: 0,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
    };

    let request = SyncRequest {
        uuid: "stats-player".to_string(),
        state: state_obj,
    };
    let _ = sync_player(State(state.clone()), Json(request)).await;

    let response = get_global_stats(State(state)).await;
    assert!(response.is_ok());

    let stats = response.unwrap().0;
    assert_eq!(stats.total_players, 1);
    assert_eq!(stats.total_solar_sails, 2);
    assert_eq!(stats.total_plasma_tethers, 1);
    assert_eq!(stats.total_orbital_mirrors, 0);
}