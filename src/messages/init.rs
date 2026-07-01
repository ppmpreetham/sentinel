use crate::{
    agent::agent::checker,
    db::pg::pg_pool,
    messages::{
        bus::EventBus,
        consumers::storage,
        detections::{port_scan, sqli, ssh_bruteforce},
    },
};

pub async fn init_mpmc() {
    let bus = EventBus::new();
    let pool = pg_pool().await;
    tokio::spawn(storage::run(bus.clone(), pool.clone()));
    tokio::spawn(reputation::run(bus.clone()));
    tokio::spawn(analytics::run(bus.clone()));
    tokio::spawn(ssh_bruteforce::run(bus.clone()));
    tokio::spawn(port_scan::run(bus.clone()));
    tokio::spawn(sqli::run(bus.clone()));
    tokio::task::spawn_blocking(move || {
        checker(bus);
    });
}
