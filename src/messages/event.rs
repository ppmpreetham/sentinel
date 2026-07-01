use crate::agent::models::AttackEvent;

#[derive(Clone)]
pub enum Event {
    Attacked(AttackEvent),
    BruteForced { ip: String, attempts: usize },
}
