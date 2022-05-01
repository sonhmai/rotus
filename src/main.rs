use axum::{
    routing::{get},
    Router,
};
use std::net::SocketAddr;

mod server;

use crate::server::handlers::kv::get_value;
use crate::server::handlers::health::get_health;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/health", get(get_health))
        .route("/api", get(get_value));

    // TODO - separate server into server mod
    // TODO - config layer and lib

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

