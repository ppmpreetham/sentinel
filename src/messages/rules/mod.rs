mod brute_force;

use super::event::Event;

pub trait Rule {
    fn process(&mut self, event: &Event) -> Option<Event>;
}

pub struct RuleEngine;

impl RuleEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&mut self, event: Event) -> Option<Event> {
        let _ = event;
        None
    }
}
