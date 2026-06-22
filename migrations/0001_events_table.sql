CREATE TABLE attack_events (
    id BIGSERIAL PRIMARY KEY,
    ip VARCHAR(45) NOT NULL,
    service VARCHAR(50) NOT NULL,
    country VARCHAR(50),
    username VARCHAR(255),
    event_type VARCHAR(50) NOT NULL,
    timestamp BIGINT NOT NULL,
    payload JSONB
);

-- indexes for ip n timestamp
CREATE INDEX idx_attack_events_ip
ON attack_events(ip);

CREATE INDEX idx_attack_events_timestamp
ON attack_events(timestamp);

-- partial index when attackers search up for root/admin
CREATE INDEX idx_critical_users
ON attack_events(username)
WHERE username IN ('root', 'admin');
