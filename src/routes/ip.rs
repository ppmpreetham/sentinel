use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
    types::{Json, ipnetwork::IpNetwork},
};

#[derive(Serialize, Deserialize)]
struct IPRoute {
    ip: IpNetwork,
    attempts: Option<i64>,
    first_seen: Option<i64>,
    last_seen: Option<i64>,
}

pub async fn get_ip(
    State(pool): State<PgPool>,
    Path(ip): Path<String>,
) -> Result<Json<IPRoute>, Response> {
    let target_ip: IpNetwork = ip.parse().map_err(|_| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            "Invalid IP address format",
        )
            .into_response()
    })?;

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
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    Ok(Json(route_data))
}
