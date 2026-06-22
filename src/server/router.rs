use axum::{Router, routing::get};

use super::state::AppState;
use crate::db::pg::pg_pool;
use crate::routes::events::events_router;
use crate::routes::ip::ip_router;

pub async fn router() -> Router {
    let pool = pg_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await;

    let state = AppState { db_pool: pool };
    let routes: Router = Router::new()
        .route("/meow", get(|| async { "meow meow meow?" }))
        .route("/n", get(|| async { "no" }))
        .route("/y", get(|| async { "yes" }))
        .merge(events_router())
        .merge(ip_router())
        .with_state(state);

    routes
}
