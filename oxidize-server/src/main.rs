use std::net::SocketAddr;
use std::path::PathBuf;

use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use oxidize_server::{create_pool, ensure_data_dir, init_database, create_router, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = PathBuf::from("./data");
    ensure_data_dir(&data_dir)?;

    let database_url = "sqlite://data/oxidize.db";
    let pool = create_pool(database_url).await?;
    init_database(&pool).await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = AppState { db: pool };
    let app = create_router(state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Oxidize Server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}