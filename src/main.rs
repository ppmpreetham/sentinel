mod agent;
mod config;
mod db;
mod features;
mod server;

#[tokio::main]
async fn main() {
    config::config();
    server::server::run();
}
