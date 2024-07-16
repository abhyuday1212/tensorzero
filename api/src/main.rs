#![forbid(unsafe_code)]

use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, router)
        .await
        .expect("Failed to start server")
}

async fn hello_world() -> impl IntoResponse {
    "HELL0 W0RLD"
}
