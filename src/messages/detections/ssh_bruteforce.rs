use tokio::time::Instant;

use crate::messages::{bus::EventBus, event::Event};
use std::{collections::HashMap, time::Duration};

// 5 reqs in a sec
const BRUTE_FORCE_THRESHOLD: usize = 5;
const WINDOW_TIME: u64 = 1;
const EVICTION_TIME: u64 = 5;

struct Incident {
    count: usize,
    last_seen: Instant,
    alerted: bool,
}

pub async fn run(bus: EventBus) {
    let mut rx = bus.subscribe();
    let mut attempts: HashMap<String, Incident> = HashMap::new();

    while let Ok(event) = rx.recv().await {
        let Event::Attacked(attack) = event else {
            continue;
        };

        let now = Instant::now();
        attempts.retain(|_, inc| {
            now.duration_since(inc.last_seen) < Duration::from_secs(EVICTION_TIME)
        });

        let incident = attempts.entry(attack.ip.clone()).or_insert(Incident {
            count: 0,
            last_seen: now,
            alerted: false,
        });

        if now.duration_since(incident.last_seen) >= Duration::from_secs(WINDOW_TIME) {
            incident.count = 1;
            incident.alerted = false;
        } else {
            incident.count += 1;
        }

        incident.last_seen = now;

        if incident.count >= BRUTE_FORCE_THRESHOLD && !incident.alerted {
            incident.alerted = true;
            bus.publish(Event::BruteForced {
                ip: attack.ip.clone(),
                attempts: incident.count,
            });
        }
    }
}
