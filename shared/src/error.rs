use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    EntityNotFound(String),
    #[error("An error occurred while executing the database operation")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
            AppError::ConvertToUuidError(_) => StatusCode::BAD_REQUEST,
            e @ AppError::SpecificOperationError(_) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}
