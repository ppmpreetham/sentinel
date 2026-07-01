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

Now for the APIs

### /events

#### KeySet pagination

When first tried with `OFFSET`, it caused **Data Drifting** during inserts.
So, I switched to Keyset pagination. Instead of calculating where you are in the list, it looks at the boundaries of the data.

```sql
WHERE $1::bigint IS NULL OR id < $1
LIMIT $2
```

basically operates as `WHERE [First Page Check] OR [Next Page Bookmark]`

also made a couple endpoints
GET /ip/:ip

GET /stats

GET /stats/services

GET /stats/usernames

there's no need to index everything, only the ones we query the most

## Event Orchestration

for mpmc, using a message broker is optimal choice.
so using tokio::broadcast to make a channel of 1024 events to hold in ring buffer is best strategy.
there's a publish and there's a subscribe method
