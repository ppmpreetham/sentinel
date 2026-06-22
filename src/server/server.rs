use crate::config::config;

use super::router::router;
use axum::serve;
use tokio::net::TcpListener;

pub async fn run() {
    let addr = &config().server_url;
    let app = router().await;
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("http://{}", addr);
    serve(listener, app)
        .await
        .expect("Server error encountered");
}
