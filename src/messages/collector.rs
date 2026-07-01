use crate::{
    agent::models::AttackEvent,
    messages::{bus::EventBus, event::Event},
};

pub fn collect(bus: &EventBus, event: AttackEvent) {
    bus.publish(Event::Attacked(event));
}
