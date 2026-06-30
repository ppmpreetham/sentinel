use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type DBResult<T> = Result<T, AppError>;

pub enum AppError {
    Sqlx(sqlx::Error),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Sqlx(err) => {
                tracing::error!(database_error = %err, "db query execution failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A cow stepped on our power cord. we're trying to shoo it off",
                )
                    .into_response()
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}
