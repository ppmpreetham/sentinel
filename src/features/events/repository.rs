use serde::Serialize;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::{self, PgPool};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AttackEventDBModel {
    pub id: i64,
    pub ip: IpNetwork,
    pub service: String,
    #[serde(skip_serializing)]
    pub username: Option<String>,
    pub event_type: String,
    pub timestamp: i64,
}

// get all events
pub async fn select_events(
    pool: &PgPool,
    limit: i64,
    cursor: Option<i64>,
) -> Result<Vec<AttackEventDBModel>, sqlx::Error> {
    let events = sqlx::query_as!(
        AttackEventDBModel,
        "SELECT id, ip, service, username, event_type, timestamp
        FROM attack_events
        WHERE $1::bigint IS NULL OR id < $1
        ORDER BY id DESC
        LIMIT $2",
        cursor,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(events)
}

// select single event
pub async fn select_event(
    pool: &PgPool,
    event_id: i64,
) -> Result<Option<AttackEventDBModel>, sqlx::Error> {
    sqlx::query_as!(
        AttackEventDBModel,
        "SELECT id, ip, service, username, event_type, timestamp FROM attack_events WHERE id = $1",
        event_id
    )
    .fetch_optional(pool)
    .await
}
