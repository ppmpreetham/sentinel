use sqlx::PgPool;

use crate::messages::{
    bus::EventBus,
    event::{self, Event},
};

pub async fn run(bus: EventBus, pool: PgPool) {
    let mut rx = bus.subscribe();
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Attacked(e) => insert_attack(&pool, e).await,
            _ => {}
        }
    }
}

async fn insert_attack(pool: &sqlx::Pool<sqlx::Postgres>, e: crate::agent::models::AttackEvent) {
    todo!()
}
