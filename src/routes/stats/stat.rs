use axum::http::StatusCode;
use sqlx::{PgPool, types::Json};

struct Stats {
    total_events: i64,
    unique_ips: i64,
    top_service: Option<String>,
}

// GET /stats
pub async fn get_stats(pool: &PgPool) -> Result<Json<Stats>, (StatusCode, String)> {
    let query = sqlx::query_as!(
        Stats,
        r#"
            WITH top_service_cte AS (
                SELECT service
                FROM attack_events
                GROUP BY service
                ORDER BY COUNT(*) DESC
                LIMIT 1
            )
            SELECT
                COUNT(*) AS "total_events!",
                COUNT(DISTINCT ip) AS "unique_ips!",
                (SELECT service FROM top_service_cte) AS "top_service"
            FROM attack_events;
            "#
    )
    .fetch_one(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(query))
}
