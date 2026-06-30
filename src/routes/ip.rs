use axum::Router;
use axum::routing::get;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::ipnetwork::IpNetwork};

use crate::db::error::{AppError, DBResult};
use crate::server::state::AppState;

#[derive(Serialize, Deserialize)]
struct IPRoute {
    ip: IpNetwork,
    attempts: Option<i64>,
    first_seen: Option<i64>,
    last_seen: Option<i64>,
}

pub fn ip_router() -> Router<AppState> {
    Router::new().route("/ip/{ip}", get(get_ip))
}

async fn get_ip(
    State(pool): State<PgPool>,
    Path(ip): Path<String>,
) -> Result<Json<IPRoute>, AppError> {
    let target_ip: IpNetwork = ip
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid IP address format".to_string()))?;

    let route_data = sqlx::query_as!(
        IPRoute,
        r#"
        SELECT
            $1::inet AS "ip!",
            COUNT(*) AS "attempts",
            MIN(timestamp) AS "first_seen",
            MAX(timestamp) AS "last_seen"
        FROM attack_events
        WHERE ip = $1;
        "#,
        target_ip
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(route_data))
}
