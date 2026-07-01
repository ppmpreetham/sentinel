use crate::agent::models::AttackEvent;

#[derive(Clone)]
pub enum Event {
    Attacked(AttackEvent),
    AttackStored {
        id: i64,
        event: AttackEvent,
    },
    PortScanDetected {
        ip: String,
        ports: Vec<u16>,
    },
    BruteForced {
        ip: String,
        attempts: usize,
    },
    CredentialStuffingDetected {
        ip: String,
        usernames: Vec<String>,
    },

    IPScoreUpdated {
        ip: String,
        old_score: u32,
        new_score: u32,
    },
    ThreatLevelChanged {
        ip: String,
        old: ThreatLevel,
        new: ThreatLevel,
    },

    AlertGenerated(Alert),

    AlertAcknowledged {
        alert_id: Uuid,
    },

    ConsumerFailed {
        consumer: &'static str,
        reason: String,
    },

    ConsumerRetried {
        consumer: &'static str,
        retries: usize,
    },
    NotificationSent,
}
