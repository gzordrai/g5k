use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, G5KError>;

#[derive(Debug, Error)]
pub enum G5KError {
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API returned {status}: {body}")]
    Api { status: StatusCode, body: String },

    #[error("response did not contain a Location header")]
    MissingLocation,

    #[error("Location header was not valid UTF-8")]
    InvalidLocation,
}
