use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    BadRequest(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        let msg = err.to_string();

        if msg.contains("not found") {
            AppError::NotFound(msg)
        } else if msg.contains("cannot be empty") {
            AppError::BadRequest(msg)
        } else {
            AppError::DatabaseError(msg)
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => {
                eprintln!("Detailed Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        };

        (status, message).into_response()
    }
}
