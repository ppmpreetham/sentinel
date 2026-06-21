use axum::{Router, routing::get};
use tokio;

use crate::{config::config, services::pg::pg_pool};

#[tokio::main]
async fn router() {
    let pool = pg_pool().await;

    let routes = Router::new()
        .route("/n", get(|| async { "no" }))
        .route("/y", get(|| async { "yes" }))
        // .with_state(config())
        ;
}
