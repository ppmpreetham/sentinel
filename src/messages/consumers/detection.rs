use sqlx::PgPool;

use crate::messages::{bus::EventBus, event::Event, rules::RuleEngine};

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut engine = RuleEngine::new();

    while let Ok(event) = rx.recv().await {
        if let Some(alert) = engine.process(event) {
            bus.publish(alert);
        }
    }
}
