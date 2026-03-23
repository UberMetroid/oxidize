use axum::{extract::State, Json};
use chrono::Utc;
use oxidize_engine::{Achievement, Faction, PlayerState};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::collections::HashSet;

use oxidize_server::handlers::{
    get_global_stats, get_leaderboard, get_player, get_player_achievements_handler, sync_player,
};
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
            last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            consecutive_days INTEGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("ALTER TABLE players ADD COLUMN consecutive_days INTEGER NOT NULL DEFAULT 1")
        .execute(&pool)
        .await
        .ok();

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

    sqlx::query(
        r#"
        CREATE TABLE player_achievements (
            player_uuid TEXT NOT NULL,
            achievement TEXT NOT NULL,
            unlocked_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            PRIMARY KEY (player_uuid, achievement),
            FOREIGN KEY (player_uuid) REFERENCES players(uuid)
        )
        "#,
    )
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
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let request = SyncRequest {
        uuid: uuid.to_string(),
        state: state_obj,
    };

    let response = sync_player(State(state), Json(request)).await;
    assert!(response.is_ok());

    let row: (String, String) = sqlx::query_as("SELECT uuid, faction FROM players WHERE uuid = ?1")
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
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
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
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let sync_request = SyncRequest {
        uuid: "player-get-test".to_string(),
        state: state_obj,
    };
    let _ = sync_player(State(state.clone()), Json(sync_request)).await;

    let response = get_player(
        State(state),
        axum::extract::Path("player-get-test".to_string()),
    )
    .await;
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
            last_purchase_time: 0,
            dyson_collectors: 0,
            quantum_arrays: 0,
            stellar_engines: 0,
            achievements: HashSet::new(),
            consecutive_days: 1,
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
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
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

#[tokio::test]
async fn test_consecutive_days_increments_on_consecutive_sync() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool.clone() };

    let state_obj = PlayerState {
        faction: Faction::Orange,
        energy: 100.0,
        total_energy_generated: 500.0,
        solar_sails: 5,
        plasma_tethers: 1,
        orbital_mirrors: 0,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let request = SyncRequest {
        uuid: "streak-player".to_string(),
        state: state_obj,
    };

    let response = sync_player(State(state.clone()), Json(request)).await;
    assert!(response.is_ok());

    let row: (i64,) =
        sqlx::query_as("SELECT consecutive_days FROM players WHERE uuid = 'streak-player'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row.0, 1);
}

#[tokio::test]
async fn test_achievement_unlocks_on_energy_milestone() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let mut achievements = HashSet::new();
    achievements.insert(Achievement::FirstSail);

    let state_obj = PlayerState {
        faction: Faction::Orange,
        energy: 200.0,
        total_energy_generated: 150.0,
        solar_sails: 1,
        plasma_tethers: 0,
        orbital_mirrors: 0,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements,
        consecutive_days: 1,
    };

    let request = SyncRequest {
        uuid: "achievement-player".to_string(),
        state: state_obj,
    };

    let response = sync_player(State(state.clone()), Json(request)).await;
    assert!(response.is_ok());

    let resp_body = response.unwrap().0;
    assert!(resp_body
        .newly_unlocked_achievements
        .iter()
        .any(|a| a.achievement == Achievement::Energy100));
}

#[tokio::test]
async fn test_player_achievements_endpoint() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool.clone() };

    sqlx::query(
        "INSERT INTO players (uuid, faction) VALUES ('achievements-test-player', 'orange')",
    )
    .execute(&pool)
    .await
    .unwrap();

    let achievements_to_insert = vec![Achievement::FirstSail, Achievement::Energy100];
    for achievement in achievements_to_insert {
        let achievement_str = serde_json::to_string(&achievement)
            .unwrap()
            .trim_matches('"')
            .to_string();
        sqlx::query("INSERT INTO player_achievements (player_uuid, achievement) VALUES ('achievements-test-player', ?1)")
            .bind(&achievement_str)
            .execute(&pool)
            .await
            .unwrap();
    }

    let response = get_player_achievements_handler(
        State(state),
        axum::extract::Path("achievements-test-player".to_string()),
    )
    .await;
    assert!(response.is_ok());

    let achievement_list = response.unwrap().0;
    assert_eq!(achievement_list.len(), 2);
    assert!(achievement_list
        .iter()
        .any(|a| a.achievement == Achievement::FirstSail));
    assert!(achievement_list
        .iter()
        .any(|a| a.achievement == Achievement::Energy100));
}

#[tokio::test]
async fn test_global_sphere_delta_calculation() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool.clone() };

    let state_obj1 = PlayerState {
        faction: Faction::Blue,
        energy: 100.0,
        total_energy_generated: 1000.0,
        solar_sails: 5,
        plasma_tethers: 1,
        orbital_mirrors: 0,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let request1 = SyncRequest {
        uuid: "delta-player".to_string(),
        state: state_obj1,
    };
    let _ = sync_player(State(state.clone()), Json(request1)).await;

    let state_obj2 = PlayerState {
        faction: Faction::Blue,
        energy: 200.0,
        total_energy_generated: 1500.0,
        solar_sails: 10,
        plasma_tethers: 2,
        orbital_mirrors: 1,
        last_sync_time: 0,
        last_synced_total_energy: 1000.0,
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let request2 = SyncRequest {
        uuid: "delta-player".to_string(),
        state: state_obj2,
    };
    let _ = sync_player(State(state.clone()), Json(request2)).await;

    let row: (f64,) =
        sqlx::query_as("SELECT total_energy_generated FROM global_sphere WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!((row.0 - 1500.0).abs() < 0.001);
}

#[tokio::test]
async fn test_leaderboard_ordering_by_energy() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let energies = vec![500.0, 1500.0, 1000.0];

    for energy in energies {
        let state_obj = PlayerState {
            faction: Faction::Orange,
            energy,
            total_energy_generated: energy,
            solar_sails: 1,
            plasma_tethers: 0,
            orbital_mirrors: 0,
            last_sync_time: 0,
            last_synced_total_energy: 0.0,
            last_purchase_time: 0,
            dyson_collectors: 0,
            quantum_arrays: 0,
            stellar_engines: 0,
            achievements: HashSet::new(),
            consecutive_days: 1,
        };

        let request = SyncRequest {
            uuid: format!("player-{}", energy),
            state: state_obj,
        };
        let _ = sync_player(State(state.clone()), Json(request)).await;
    }

    let response = get_leaderboard(State(state)).await;
    assert!(response.is_ok());

    let body = response.unwrap().0;
    assert_eq!(body.entries.len(), 3);
    assert_eq!(body.entries[0].total_energy, 1500.0);
    assert_eq!(body.entries[1].total_energy, 1000.0);
    assert_eq!(body.entries[2].total_energy, 500.0);
}

#[tokio::test]
async fn test_sync_response_includes_server_time() {
    let pool = create_test_pool().await;
    let state = AppState { db: pool };

    let now_millis = Utc::now().timestamp_millis();

    let state_obj = PlayerState {
        faction: Faction::Orange,
        energy: 100.0,
        total_energy_generated: 500.0,
        solar_sails: 5,
        plasma_tethers: 1,
        orbital_mirrors: 0,
        last_sync_time: 0,
        last_synced_total_energy: 0.0,
        last_purchase_time: 0,
        dyson_collectors: 0,
        quantum_arrays: 0,
        stellar_engines: 0,
        achievements: HashSet::new(),
        consecutive_days: 1,
    };

    let request = SyncRequest {
        uuid: "time-test-player".to_string(),
        state: state_obj,
    };

    let response = sync_player(State(state), Json(request)).await;
    assert!(response.is_ok());

    let resp_body = response.unwrap().0;
    assert!(resp_body.server_time >= now_millis);
}
