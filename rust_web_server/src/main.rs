use axum::{
    routing::{get, post},
    Router,
    extract::Path,
    response::{Json, IntoResponse},
};
use hyper::server::Server;  // Correct import for hyper::Server
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define the routes
    let app = Router::new()
        .route("/", get(root))
        .route("/greet/:name", get(greet))
        .route("/json", post(json_response));

    // Define the address for the server to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Use hyper's Server to run the app
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for "/"
async fn root() -> &'static str {
    "Hello, World!"
}

// Handler for "/greet/:name"
async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

// Handler for "/json"
async fn json_response() -> impl IntoResponse {
    Json(json!({"message": "Hello from Rust!", "status": "success"}))
}