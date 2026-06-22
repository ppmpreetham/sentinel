use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

pub type PaginatedJson<T> = Result<Json<PaginatedResponse<T>>, (StatusCode, String)>;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub cursor: Option<i64>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<i64>,
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(mut data: Vec<T>, limit: i64, get_id: impl Fn(&T) -> i64) -> Self {
        let has_more = data.len() > limit as usize;
        if has_more {
            data.pop();
        }

        let next_cursor = data.last().map(get_id);
        Self {
            data,
            next_cursor,
            has_more,
        }
    }
}
