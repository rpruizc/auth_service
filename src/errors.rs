use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "BadId")]
    BadId,

    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),

    // For those cases where resources are not found
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    #[display(fmt = "ProcessFailed: {}", _0)]
    ProcessError(String),

}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),
            AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),
            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            AuthError::ProcessError(ref message) => HttpResponse::InternalServerError().json(message),
        }
    }
}

impl From<UuidError> for AuthError {
    fn from(_: UuidError) -> AuthError {
        AuthError::BadId
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        // We only care about UniqueViolations
        match error {
            DBError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();

                match kind {
                    DatabaseErrorKind::UniqueViolation => AuthError::DuplicateValue(message),
                    _ => AuthError::GenericError(message)
                }
            }
            _ => AuthError::GenericError(String::from("Some database error occurred")),
        }
    }
}