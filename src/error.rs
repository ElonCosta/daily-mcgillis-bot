use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

#[derive(Debug)]
pub enum AppError {
    JsonRejection(JsonRejection),
    SqlxError(sqlx::Error),
    IoError(std::io::Error),
    MigrateError(sqlx::migrate::MigrateError),
    ImageError(image::ImageError),
    TurboJpegError(turbojpeg::Error),
}

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message): (StatusCode, String) = match self {
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::SqlxError(err) => match err {
                sqlx::Error::Database(e) => (StatusCode::BAD_REQUEST, format!("{e:?}")),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            },
            AppError::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            AppError::MigrateError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Couldn't create, cause: {err:?}"),
            ),
            AppError::ImageError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            AppError::TurboJpegError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::MigrateError(value)
    }
}

impl From<image::ImageError> for AppError {
    fn from(value: image::ImageError) -> Self {
        Self::ImageError(value)
    }
}


impl From<turbojpeg::Error> for AppError {
    fn from(value: turbojpeg::Error) -> Self {
        Self::TurboJpegError(value)
    }
}


