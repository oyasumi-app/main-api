use axum::http::StatusCode;

pub struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let status_code = if let Some(error) = self.0.downcast_ref::<ApiError>() {
            match error {
                ApiError::DatabaseErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::NotFound => StatusCode::NOT_FOUND,
                ApiError::Forbidden => StatusCode::FORBIDDEN,
                ApiError::NeverError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status_code, self.0.to_string()).into_response()
    }
}

impl From<ApiError> for ResponseError {
    fn from(err: ApiError) -> Self {
        Self(err.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseErr(sea_orm::DbErr),

    #[error("this should never happen (please report a bug!): {0}")]
    NeverError(String),

    #[error("entity not found")]
    NotFound,

    #[error("you do not have access to this entity")]
    Forbidden,
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::DatabaseErr(err)
    }
}

impl From<sea_orm::DbErr> for ResponseError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::from(ApiError::from(err))
    }
}

pub type ResultResponse<T> = std::result::Result<T, ResponseError>;
