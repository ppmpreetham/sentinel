use crate::{
    db::error::{AppError, DBResult},
    server::state::AppState,
};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    total_events: i64,
    unique_ips: i64,
    top_service: Option<String>,
}

// GET /stats
pub async fn get_stats(State(pool): State<PgPool>) -> Result<Json<Stats>, AppError> {
    let stats = db_stats(&pool).await?;
    Ok(axum::Json(stats))
}

pub async fn db_stats(pool: &PgPool) -> DBResult<Stats> {
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
    .await?;

    Ok(query)
}

// GET /stats/services
#[derive(Serialize, Deserialize)]
pub struct Service {
    service: String,
    count: Option<i64>,
}

// GET /stats/services
pub async fn get_stats_services(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Service>>, AppError> {
    let result = sqlx::query_as!(
        Service,
        r#"SELECT
            service,
            COUNT(*) AS count
        FROM attack_events
        GROUP BY service
        ORDER BY count"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(result))
}

#[derive(Serialize, Deserialize)]
pub struct Username {
    username: Option<String>,
    attempts: i64,
}

// GET /stats/usernames
pub async fn get_usernames(State(pool): State<PgPool>) -> Result<Json<Vec<Username>>, AppError> {
    let res = sqlx::query_as!(
        Username,
        r#"
        SELECT
            username, count(username) AS "attempts!"
        FROM attack_events
        WHERE username IN ('root', 'admin')
        GROUP BY username
        ORDER BY "attempts!" DESC
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(res))
}
