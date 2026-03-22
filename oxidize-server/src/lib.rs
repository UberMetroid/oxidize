pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;

pub use db::{create_pool, ensure_data_dir, init_database};
pub use routes::create_router;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}