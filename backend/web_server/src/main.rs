pub mod handlers;

use std::env;
use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("../.env")?;

    let port = env::var("BACKEND_PORT").expect("env: BACKEND_PORT should be set");

    let app = Router::new()
        .route("/", get(handlers::todos::get_todos));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
