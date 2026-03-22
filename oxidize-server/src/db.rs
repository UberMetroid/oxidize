use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
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
            last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

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

    Ok(())
}

pub fn ensure_data_dir(data_dir: &Path) -> std::io::Result<()> {
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }
    Ok(())
}