use crate::{
    agent::agent::checker,
    db::pg::pg_pool,
    messages::{
        bus::EventBus,
        consumers::{
            detection::{self},
            storage,
        },
    },
};

pub async fn init_mpmc() {
    let bus = EventBus::new();
    let pool = pg_pool().await;
    tokio::spawn(storage::run(bus.clone(), pool.clone()));
    tokio::spawn(detection::run(bus.clone()));
    tokio::task::spawn_blocking(move || {
        checker(bus);
    });
}
