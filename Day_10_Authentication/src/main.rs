use axum::{
    routing::{post, get},
    Router,
};

mod auth;
mod handlers;
mod middleware;
mod model;
mod state;

use state::Db;

#[tokio::main]
async fn main() {

    let db: Db = std::sync::Arc::new(
        std::sync::Mutex::new(
            std::collections::HashMap::new()
        )
    );

    let app = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route(
            "/protected",
            get(middleware::protected)
                .route_layer(
                    axum::middleware::from_fn(
                        middleware::auth_middleware
                    )
                )
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on 3000");

    axum::serve(listener, app).await.unwrap();
}