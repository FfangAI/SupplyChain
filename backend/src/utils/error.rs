use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    NotFoundError(String),
    ValidationError(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseError(err) => write!(f, "Database error: {}", err),
            Self::NotFoundError(msg) => write!(f, "Not found: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::DatabaseError(err) => {
                log::error!("Database error: {:?}", err);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "error".into(),
                    message: "An internal server error occurred".into(),
                })
            }
            Self::NotFoundError(msg) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    status: "error".into(),
                    message: msg.clone(),
                })
            }
            Self::ValidationError(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    status: "error".into(),
                    message: msg.clone(),
                })
            }
            Self::InternalServerError(msg) => {
                log::error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    status: "error".into(),
                    message: "An internal server error occurred".into(),
                })
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFoundError("Resource not found".into()),
            _ => Self::DatabaseError(err),
        }
    }
}