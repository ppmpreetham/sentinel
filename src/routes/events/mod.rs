use axum::Router;
use axum::routing::get;
use sqlx::PgPool;

use crate::server::state::AppState;

mod controller;
mod repository;

pub fn events_router() -> Router<AppState> {
    Router::new().route("/events", get(controller::get_events_handler))
}
