use sqlx::PgPool;

use crate::messages::{bus::EventBus, event::Event};

pub async fn run(bus: EventBus, pool: PgPool) {
    let mut rx = bus.subscribe();
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Attacked(event) => bus.publish(Event::BruteForced {
                ip: event.ip,
                // TODO: PUT THE RIGHT NUMBER HERE
                attempts: 10,
            }),
            _ => {}
        }
    }
}
