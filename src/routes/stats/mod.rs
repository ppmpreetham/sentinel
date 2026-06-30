use axum::{Router, routing::get};

use crate::{
    routes::stats::stat::{get_stats_services, get_usernames},
    server::state::AppState,
};
use stat::get_stats;
mod stat;

pub fn stats_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_stats))
        .route("/services", get(get_stats_services))
        .route("/usernames", get(get_usernames))
}
