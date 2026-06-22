#[allow(unused)]
mod agent;
mod config;
mod db;
mod routes;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    config::config();
    server::server::run().await;
}
