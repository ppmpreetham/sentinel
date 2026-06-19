use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AttackEvent {
    pub ip: String,
    pub service: String,
    pub username: String,
    pub timestamp: u64,
    pub event_type: String,
}
