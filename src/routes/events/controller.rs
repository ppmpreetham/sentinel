use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use std::result::Result;

use super::repository::AttackEventDBModel;
use crate::{
    routes::events::repository::{select_event, select_events},
    utils::paginate::{PaginatedJson, PaginatedResponse, PaginationQuery},
};

// GET /events
pub async fn get_events_handler(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> PaginatedJson<AttackEventDBModel> {
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let events = select_events(&pool, limit + 1, query.cursor)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let response = PaginatedResponse::new(events, limit, |event| event.id);
    Ok(Json(response))
}

// GET /event/420
pub async fn get_event_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<AttackEventDBModel>, Response> {
    let event = select_event(&pool, id).await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Service Error").into_response()
    })?;

    match event {
        Some(item) => Ok(Json(item)),
        None => Err((StatusCode::NOT_FOUND, "Event not found").into_response()),
    }
}

// POST /event/420
// TODO
