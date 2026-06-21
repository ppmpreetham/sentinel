use axum::{Router, routing::get};
use tokio;

#[tokio::main]
async fn router() {
    let routes = Router::<()>::new()
        .route("/n", get(|| async { "no" }))
        .route("/y", get(|| async { "yes" }));
}
