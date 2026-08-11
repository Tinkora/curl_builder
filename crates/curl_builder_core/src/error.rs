use thiserror::Error;

/// Stable error type for curl_builder operations.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("URL is empty")]
    EmptyUrl,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid HTTP header: {0}")]
    InvalidHeader(String),

    #[error("Invalid request body: {0}")]
    InvalidBody(String),

    #[error("Invalid JSON body: {0}")]
    InvalidJson(String),

    #[error("Input limit exceeded: {0}")]
    LimitExceeded(String),

    #[error("Failed to parse cURL command: {0}")]
    ParseError(String),

    #[error("Unsupported cURL option: {0}")]
    UnsupportedCurlOption(String),

    #[error("Unsupported shell syntax in cURL command")]
    UnsupportedShellSyntax,

    #[error("JSON serialization error: {0}")]
    SerializationError(String),

    #[error("Unsupported body type for this language: {0}")]
    UnsupportedBodyType(String),

    #[error("Unsupported output language: {0}")]
    UnsupportedLanguage(String),
}

impl CoreError {
    /// Returns a stable machine error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::EmptyUrl => "EMPTY_URL",
            Self::InvalidUrl(_) => "INVALID_URL",
            Self::InvalidHeader(_) => "INVALID_HEADER",
            Self::InvalidBody(_) => "INVALID_BODY",
            Self::InvalidJson(_) => "INVALID_JSON",
            Self::LimitExceeded(_) => "LIMIT_EXCEEDED",
            Self::ParseError(_) => "PARSE_ERROR",
            Self::UnsupportedCurlOption(_) => "UNSUPPORTED_CURL_OPTION",
            Self::UnsupportedShellSyntax => "UNSUPPORTED_SHELL_SYNTAX",
            Self::SerializationError(_) => "SERIALIZATION_ERROR",
            Self::UnsupportedBodyType(_) => "UNSUPPORTED_BODY_TYPE",
            Self::UnsupportedLanguage(_) => "UNSUPPORTED_LANGUAGE",
        }
    }
}
