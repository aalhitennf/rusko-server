use actix_web::{
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "Bad request: {}", message)]
    BadRequest { message: String },
    #[display(fmt = "Configuration error: {}", message)]
    ConfigError { message: String },
    #[display(fmt = "Decryption error: {}", message)]
    DecryptError { message: String },
    #[display(fmt = "Encryption error: {}", message)]
    EncryptError { message: String },
    #[display(fmt = "Internal error: {}", message)]
    InternalError { message: String },
    #[display(fmt = "Pairing error: {}", message)]
    PairingError { message: String },
    #[display(fmt = "Unauthorized: {}", message)]
    Unauthorized { message: String },
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InternalError { .. }
            | ServerError::EncryptError { .. }
            | ServerError::ConfigError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::DecryptError { .. } | ServerError::BadRequest { .. } => {
                StatusCode::BAD_REQUEST
            }
            ServerError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ServerError::PairingError { .. } => StatusCode::FORBIDDEN,
        }
    }
}
