use sqlx::PgPool;

use crate::messages::{
    bus::EventBus,
    event::{self, Event},
};

pub async fn run(bus: EventBus, pool: PgPool) {
    let mut rx = bus.subscribe();
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Attacked(event) => {}
            _ => {}
        }
    }
}
