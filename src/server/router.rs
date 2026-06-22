use axum::{
    Router,
    routing::{Route, get},
};
use tokio;

use super::state::AppState;
use crate::db::pg::pg_pool;
use crate::routes::events::events_router;

pub async fn router() -> Router {
    let pool = pg_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await;

    let state = AppState { db_pool: pool };
    let routes: Router = Router::new()
        .route("/n", get(|| async { "no" }))
        .route("/y", get(|| async { "yes" }))
        .merge(events_router())
        .with_state(state);

    routes
}
