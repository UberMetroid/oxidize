use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv().ok();

    let app = Router::new().fallback_service(ServeDir::new("dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9531")
        .await
        .expect("Failed to bind to port 9531");

    println!("OXIDIZE SERVER LISTENING ON 9531");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server failed to start");
}
