#[allow(unused)]
mod agent;
mod config;
mod db;
mod features;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    config::config();
    server::server::run().await;
}
