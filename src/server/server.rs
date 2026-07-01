use crate::{config::config, messages};

use super::router::router;
use axum::serve;
use tokio::net::TcpListener;

pub async fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    messages::init::init_mpmc().await;
    let addr = &config().server_url;
    let app = router().await;
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("http://{}", addr);
    serve(listener, app)
        .await
        .expect("Server error encountered");
}
