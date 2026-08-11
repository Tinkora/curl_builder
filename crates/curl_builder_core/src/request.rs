use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::CoreError;

pub(crate) const MAX_INPUT_BYTES: usize = 1024 * 1024;
const MAX_URL_BYTES: usize = 8 * 1024;
const MAX_HEADERS: usize = 100;
const MAX_HEADER_NAME_BYTES: usize = 256;
const MAX_HEADER_VALUE_BYTES: usize = 16 * 1024;

/// An HTTP request model for code generation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    #[serde(default)]
    pub body_type: BodyType,
}

impl HttpRequest {
    /// Creates a minimal request with GET method and empty URL.
    pub fn empty() -> Self {
        Self {
            method: HttpMethod::GET,
            url: String::new(),
            headers: Vec::new(),
            body: None,
            body_type: BodyType::None,
        }
    }

    /// Creates a new request with the given method and URL.
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: None,
            body_type: BodyType::None,
        }
    }

    /// Validates the URL and request fields at the shared trust boundary.
    pub fn validate(&self) -> Result<(), CoreError> {
        self.validate_url()?;

        if self.headers.len() > MAX_HEADERS {
            return Err(CoreError::LimitExceeded(format!(
                "at most {MAX_HEADERS} headers are allowed"
            )));
        }

        let mut header_names = HashSet::with_capacity(self.headers.len());
        for (name, value) in &self.headers {
            validate_header(name, value)?;
            if !header_names.insert(name.to_ascii_lowercase()) {
                return Err(CoreError::InvalidHeader(
                    "duplicate header names are not supported".to_string(),
                ));
            }
        }

        if let Some(body) = &self.body {
            if body.len() > MAX_INPUT_BYTES {
                return Err(CoreError::LimitExceeded(format!(
                    "request body must not exceed {MAX_INPUT_BYTES} bytes"
                )));
            }

            if !body.is_empty() {
                if matches!(self.method, HttpMethod::GET | HttpMethod::HEAD) {
                    return Err(CoreError::UnsupportedBodyType(
                        "GET and HEAD requests cannot include a body".to_string(),
                    ));
                }
                if self.body_type == BodyType::None {
                    return Err(CoreError::UnsupportedBodyType(
                        "a non-empty body requires an explicit body type".to_string(),
                    ));
                }
                if body.contains('\0') {
                    return Err(CoreError::InvalidBody(
                        "request bodies must not contain NUL".to_string(),
                    ));
                }
                self.validate_content_type()?;

                if self.body_type == BodyType::Json {
                    serde_json::from_str::<serde_json::Value>(body)
                        .map_err(|error| CoreError::InvalidJson(error.to_string()))?;
                }
            }
        }

        let serialized = serde_json::to_vec(self)
            .map_err(|error| CoreError::SerializationError(error.to_string()))?;
        if serialized.len() > MAX_INPUT_BYTES {
            return Err(CoreError::LimitExceeded(format!(
                "serialized request must not exceed {MAX_INPUT_BYTES} bytes"
            )));
        }

        Ok(())
    }

    /// Validates an absolute HTTP(S) URL.
    pub fn validate_url(&self) -> Result<(), CoreError> {
        if self.url.is_empty() {
            return Err(CoreError::EmptyUrl);
        }

        if self.url.len() > MAX_URL_BYTES {
            return Err(CoreError::LimitExceeded(format!(
                "URL must not exceed {MAX_URL_BYTES} bytes"
            )));
        }

        let parsed =
            url::Url::parse(&self.url).map_err(|error| CoreError::InvalidUrl(error.to_string()))?;
        if !matches!(parsed.scheme(), "http" | "https") || !parsed.has_host() {
            return Err(CoreError::InvalidUrl(
                "only absolute HTTP(S) URLs are supported".to_string(),
            ));
        }
        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(CoreError::InvalidUrl(
                "URL userinfo is not supported; use an Authorization header".to_string(),
            ));
        }

        Ok(())
    }

    /// Returns the Content-Type header value if present.
    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
            .map(|(_, v)| v.as_str())
    }

    /// Returns the Authorization header value if present.
    pub fn authorization(&self) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("authorization"))
            .map(|(_, v)| v.as_str())
    }

    fn validate_content_type(&self) -> Result<(), CoreError> {
        let Some(content_type) = self.content_type() else {
            return Ok(());
        };
        let media_type = content_type.split(';').next().unwrap_or_default().trim();
        if media_type.eq_ignore_ascii_case("multipart/form-data") {
            return Err(CoreError::UnsupportedBodyType(
                "multipart/form-data is outside the Alpha contract".to_string(),
            ));
        }

        let Some(expected) = self.body_type.content_type_header() else {
            return Ok(());
        };
        if self.body_type != BodyType::Raw && !media_type.eq_ignore_ascii_case(expected) {
            return Err(CoreError::UnsupportedBodyType(format!(
                "{} bodies require Content-Type {expected}",
                self.body_type.label()
            )));
        }

        Ok(())
    }
}

/// HTTP methods supported by the builder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    /// Returns the uppercase string representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            Self::OPTIONS => "OPTIONS",
        }
    }

    /// Whether this method typically carries a request body.
    pub const fn allows_body(self) -> bool {
        !matches!(self, Self::GET | Self::HEAD)
    }

    /// Parse from a case-insensitive string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Self::GET),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "PATCH" => Some(Self::PATCH),
            "DELETE" => Some(Self::DELETE),
            "HEAD" => Some(Self::HEAD),
            "OPTIONS" => Some(Self::OPTIONS),
            _ => None,
        }
    }

    /// All methods in display order.
    pub const fn all() -> [HttpMethod; 7] {
        [
            Self::GET,
            Self::POST,
            Self::PUT,
            Self::PATCH,
            Self::DELETE,
            Self::HEAD,
            Self::OPTIONS,
        ]
    }

    /// Color class for UI rendering.
    pub const fn color_class(self) -> &'static str {
        match self {
            Self::GET => "get",
            Self::POST => "post",
            Self::PUT => "put",
            Self::PATCH => "patch",
            Self::DELETE => "delete",
            Self::HEAD => "head",
            Self::OPTIONS => "options",
        }
    }

    /// Chinese label for UI.
    pub const fn label_zh(self) -> &'static str {
        match self {
            Self::GET => "获取",
            Self::POST => "创建",
            Self::PUT => "更新",
            Self::PATCH => "部分更新",
            Self::DELETE => "删除",
            Self::HEAD => "头部",
            Self::OPTIONS => "选项",
        }
    }
}

/// Request body content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BodyType {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "form_urlencoded")]
    FormUrlEncoded,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "xml")]
    Xml,
}

impl BodyType {
    /// Returns the stable wire name used by JSON and WASM callers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Json => "json",
            Self::FormUrlEncoded => "form_urlencoded",
            Self::Raw => "raw",
            Self::Xml => "xml",
        }
    }

    /// Returns the Content-Type header value for this body type.
    pub const fn content_type_header(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Json => Some("application/json"),
            Self::FormUrlEncoded => Some("application/x-www-form-urlencoded"),
            Self::Raw => Some("text/plain"),
            Self::Xml => Some("application/xml"),
        }
    }

    /// Human-readable name.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Json => "JSON",
            Self::FormUrlEncoded => "Form URL-Encoded",
            Self::Raw => "Raw",
            Self::Xml => "XML",
        }
    }

    /// Chinese label for UI.
    pub const fn label_zh(self) -> &'static str {
        match self {
            Self::None => "无",
            Self::Json => "JSON",
            Self::FormUrlEncoded => "表单",
            Self::Raw => "原始文本",
            Self::Xml => "XML",
        }
    }

    /// Parse from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "none" | "" => Some(Self::None),
            "json" | "application/json" => Some(Self::Json),
            "form_urlencoded"
            | "form"
            | "x-www-form-urlencoded"
            | "application/x-www-form-urlencoded" => Some(Self::FormUrlEncoded),
            "raw" | "text" | "plain" | "text/plain" => Some(Self::Raw),
            "xml" | "application/xml" | "text/xml" => Some(Self::Xml),
            _ => None,
        }
    }
}

fn validate_header(name: &str, value: &str) -> Result<(), CoreError> {
    if name.is_empty() || name.len() > MAX_HEADER_NAME_BYTES {
        return Err(CoreError::InvalidHeader(format!(
            "header names must contain 1 to {MAX_HEADER_NAME_BYTES} bytes"
        )));
    }

    if !name.bytes().all(is_http_token_byte) {
        return Err(CoreError::InvalidHeader(
            "header names must use HTTP token characters".to_string(),
        ));
    }

    if value.len() > MAX_HEADER_VALUE_BYTES {
        return Err(CoreError::LimitExceeded(format!(
            "header values must not exceed {MAX_HEADER_VALUE_BYTES} bytes"
        )));
    }

    if value
        .chars()
        .any(|character| character != '\t' && character.is_ascii_control())
    {
        return Err(CoreError::InvalidHeader(
            "header values must not contain ASCII control characters other than tab".to_string(),
        ));
    }

    Ok(())
}

const fn is_http_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}
