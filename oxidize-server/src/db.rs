use oxidize_engine::Achievement;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::collections::HashSet;
use std::path::Path;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn init_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            uuid TEXT PRIMARY KEY,
            faction TEXT NOT NULL DEFAULT 'orange',
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            consecutive_days INTEGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("ALTER TABLE players ADD COLUMN consecutive_days INTEGER NOT NULL DEFAULT 1")
        .execute(pool)
        .await
        .ok();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player_scores (
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
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_scores_energy 
        ON player_scores(total_energy_generated DESC)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_scores_player 
        ON player_scores(player_uuid)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS global_sphere (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            total_energy_generated REAL NOT NULL DEFAULT 0,
            total_solar_sails INTEGER NOT NULL DEFAULT 0,
            total_plasma_tethers INTEGER NOT NULL DEFAULT 0,
            total_orbital_mirrors INTEGER NOT NULL DEFAULT 0,
            last_updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO global_sphere (id) VALUES (1)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player_achievements (
            player_uuid TEXT NOT NULL,
            achievement TEXT NOT NULL,
            unlocked_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            PRIMARY KEY (player_uuid, achievement),
            FOREIGN KEY (player_uuid) REFERENCES players(uuid)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub fn ensure_data_dir(data_dir: &Path) -> std::io::Result<()> {
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }
    Ok(())
}

pub async fn get_player_achievements(
    pool: &SqlitePool,
    player_uuid: &str,
) -> Result<HashSet<Achievement>, sqlx::Error> {
    let rows: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT achievement FROM player_achievements WHERE player_uuid = ?1
        "#,
    )
    .bind(player_uuid)
    .fetch_all(pool)
    .await?;

    let mut achievements = HashSet::new();
    for row in rows {
        if let Ok(achievement) = serde_json::from_str::<Achievement>(&format!("\"{}\"", row)) {
            achievements.insert(achievement);
        }
    }
    Ok(achievements)
}

pub async fn insert_player_achievement(
    pool: &SqlitePool,
    player_uuid: &str,
    achievement: &Achievement,
) -> Result<(), sqlx::Error> {
    let achievement_str = serde_json::to_string(achievement)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO player_achievements (player_uuid, achievement) VALUES (?1, ?2)
        "#,
    )
    .bind(player_uuid)
    .bind(achievement_str)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_player_streak(
    pool: &SqlitePool,
    player_uuid: &str,
) -> Result<(u32, i64), sqlx::Error> {
    let row: Option<(u32, i64)> = sqlx::query_as(
        r#"
        SELECT consecutive_days, last_seen_at FROM players WHERE uuid = ?1
        "#,
    )
    .bind(player_uuid)
    .fetch_optional(pool)
    .await?;

    Ok(row.unwrap_or((1, 0)))
}

pub async fn update_player_streak(
    pool: &SqlitePool,
    player_uuid: &str,
    consecutive_days: u32,
    now_seconds: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE players SET consecutive_days = ?1, last_seen_at = ?2 WHERE uuid = ?3
        "#,
    )
    .bind(consecutive_days)
    .bind(now_seconds as i64)
    .bind(player_uuid)
    .execute(pool)
    .await?;

    Ok(())
}
