use axum::{Router, routing::get};
use tokio;

use super::state::AppState;
use crate::db::pg::pg_pool;

#[tokio::main]
async fn router() {
    let state = AppState {
        db_pool: pg_pool().await,
    };

    let routes: Router = Router::new()
        .route("/n", get(|| async { "no" }))
        .route("/y", get(|| async { "yes" }))
        .with_state(state);
}
