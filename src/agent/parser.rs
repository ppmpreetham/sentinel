use crate::agent::agent::AttackEvent;
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AttackParser {
    ssh: Regex,
    ftp: Regex,
    http: Regex,
}

impl AttackParser {
    pub fn new() -> Self {
        Self {
            ssh: Regex::new(r"Failed password for (?:invalid user )?(\S+) from (\S+)").unwrap(),
            ftp: Regex::new(r"\[(\S+)\] FAIL LOGIN: Client \x22([0-9.]+)\x22").unwrap(),
            http: Regex::new(r"^([0-9.]+) - (\S+) \[.+\] \x22.+\x22 (?:401|403)").unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> Option<AttackEvent> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();

        // ssh
        if let Some(caps) = self.ssh.captures(line) {
            return Some(AttackEvent {
                ip: caps.get(2)?.as_str().to_string(),
                service: "ssh".to_string(),
                username: caps.get(1)?.as_str().to_string(),
                timestamp,
                event_type: "brute_force".to_string(),
            });
        }

        // ftp
        if let Some(caps) = self.ftp.captures(line) {
            return Some(AttackEvent {
                ip: caps.get(2)?.as_str().to_string(),
                service: "ftp".to_string(),
                username: caps.get(1)?.as_str().to_string(),
                timestamp,
                event_type: "brute_force".to_string(),
            });
        }

        // http
        if let Some(caps) = self.http.captures(line) {
            return Some(AttackEvent {
                ip: caps.get(1)?.as_str().to_string(),
                service: "http".to_string(),
                username: caps.get(2)?.as_str().to_string(),
                timestamp,
                event_type: "unauthorized_access".to_string(),
            });
        }
        None
    }
}
