use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use axum::{routing::get, routing::post, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use oxidize_server::{create_pool, ensure_data_dir, init_database, AppState};

const DEFAULT_PORT: u16 = 7412;
const DEFAULT_HOST: &str = "0.0.0.0";

fn load_config() {
    if let Err(e) = dotenv() {
        info!("No .env file found: {}", e);
    }
}

fn get_env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_data_dir() -> PathBuf {
    PathBuf::from(get_env_or_default("DATA_DIR", "./data"))
}

fn get_database_url() -> String {
    let data_dir = get_data_dir();
    ensure_data_dir(&data_dir).ok();
    let db_path = std::env::current_dir()
        .unwrap_or_default()
        .join(&data_dir)
        .join("oxidize.db");
    std::fs::File::create(&db_path).ok();
    let path = db_path.to_string_lossy().replace("/./", "/").replace("./", "");
    path
}

fn get_server_host() -> String {
    get_env_or_default("HOST", DEFAULT_HOST)
}

fn get_server_port() -> u16 {
    get_env_or_default("PORT", "")
        .parse::<u16>()
        .unwrap_or(DEFAULT_PORT)
}

fn get_log_level() -> Level {
    let level_str = get_env_or_default("RUST_LOG", "info");
    level_str.parse::<Level>().unwrap_or(Level::INFO)
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received SIGINT, initiating graceful shutdown...");
        }
        _ = terminate => {
            info!("Received SIGTERM, initiating graceful shutdown...");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = get_log_level();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    load_config();

    let data_dir = get_data_dir();
    ensure_data_dir(&data_dir)?;

    let database_url = get_database_url();
    let pool = create_pool(&database_url).await?;
    init_database(&pool).await?;

    let host = get_server_host();
    let port = get_server_port();
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], port)));
    let listener = TcpListener::bind(addr).await?;

    info!("Oxidize Server running on http://{}:{}", host, port);
    info!("Health endpoint: http://{}:{}/health", host, port);

    let state = AppState { db: pool };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(86400));

    let app = Router::new()
        .route("/", get(oxidize_server::handlers::health_check))
        .route("/health", get(oxidize_server::handlers::health_check))
        .route("/api/sync", post(oxidize_server::handlers::sync_player))
        .route(
            "/api/player/:uuid",
            get(oxidize_server::handlers::get_player),
        )
        .route(
            "/api/player/:uuid/achievements",
            get(oxidize_server::handlers::get_player_achievements_handler),
        )
        .route(
            "/api/leaderboard",
            get(oxidize_server::handlers::get_leaderboard),
        )
        .route(
            "/api/global-stats",
            get(oxidize_server::handlers::get_global_stats),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let server = axum::serve(listener, app);

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_signal() => {
            info!("Closing database connections...");
        }
    };

    info!("Shutdown complete.");
    Ok(())
}
