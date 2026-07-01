use tokio::time::Instant;

use crate::messages::{bus::EventBus, event::Event};
use std::{collections::HashMap, time::Duration};

// 5 reqs in a sec
static BRUTELIMIT: usize = 5;
static RATELIMIT: u64 = 1;

struct Incident {
    count: usize,
    first_seen: Instant,
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

        // reset
        let incident = attempts.entry(attack.ip.clone()).or_insert(Incident {
            count: 0,
            first_seen: now,
            alerted: false,
        });

        // reset window
        if now.duration_since(incident.first_seen) >= Duration::from_secs(RATELIMIT) {
            incident.count = 1;
            incident.first_seen = now;
            incident.alerted = false;
        } else {
            incident.count += 1;
        }

        if incident.count >= BRUTELIMIT && !incident.alerted {
            incident.alerted = true;
            bus.publish(Event::BruteForced {
                ip: attack.ip.clone(),
                attempts: incident.count,
            });
        }
    }
}
