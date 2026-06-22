```text
auth.log
   |
   v
Rust Agent
   |
   | POST /events
   v
Axum API
   |
   v
Postgres
```

for now, agent can check logs for 3 types:

- ssh
- http
- ftp

and everything is stored as

```sql
CREATE TABLE attack_events (
    id BIGSERIAL PRIMARY KEY,
    ip VARCHAR(45) NOT NULL,
    service VARCHAR(50) NOT NULL,
    country VARCHAR(50),
    event_type VARCHAR(50) NOT NULL,
    timestamp BIGINT NOT NULL,
    payload JSONB
);
```

no need for normalizing this because joins become expensive.

here, we'll index on both ip and timestamp seperately,
cuz we'll be quering them seperately and rarely both
non-clustered cuz we need high write speed.

```sql
CREATE INDEX idx_attack_events_ip
ON attack_events (source_ip);

CREATE INDEX idx_attack_events_timestamp
ON attack_events (event_timestamp);
```

using partial index for usernames like `root` and `admin` help us find abnormal anomallies faster
