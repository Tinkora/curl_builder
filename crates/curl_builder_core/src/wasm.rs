use crate::error::CoreError;
use crate::generate::{self, language_list};
use crate::request::HttpRequest;

fn public_error(error: CoreError) -> String {
    serde_json::json!({
        "code": error.code(),
        "message": error.to_string(),
    })
    .to_string()
}

fn serialization_error(error: impl std::fmt::Display) -> String {
    public_error(CoreError::SerializationError(error.to_string()))
}

/// WASM-exported: Generate code for a given request JSON and language.
/// `request_json` is a JSON-serialized `HttpRequest`.
/// Returns the generated code string.
pub fn wasm_generate_code(request_json: &str, language: &str) -> Result<String, String> {
    let request: HttpRequest = serde_json::from_str(request_json).map_err(serialization_error)?;

    generate::generate(&request, language).map_err(public_error)
}

/// WASM-exported: Parse a cURL command into an HttpRequest JSON.
pub fn wasm_parse_curl(curl_command: &str) -> Result<String, String> {
    let request = generate::parse_curl(curl_command).map_err(public_error)?;

    serde_json::to_string(&request).map_err(serialization_error)
}

/// WASM-exported: Returns the list of supported languages as a JSON array.
pub fn wasm_list_languages() -> String {
    serde_json::to_string(language_list()).unwrap_or_else(|_| "[]".to_string())
}

/// WASM-exported: Validate a URL string.
pub fn wasm_validate_url(url: &str) -> Result<String, String> {
    HttpRequest::new(crate::request::HttpMethod::GET, url)
        .validate_url()
        .map_err(public_error)?;
    Ok(serde_json::json!({"valid": true, "url": url}).to_string())
}

/// WASM-exported: Serialize an HttpRequest to JSON.
pub fn wasm_serialize_request(request: &HttpRequest) -> Result<String, String> {
    request.validate().map_err(public_error)?;
    serde_json::to_string(request).map_err(serialization_error)
}

/// WASM-exported: Deserialize an HttpRequest from JSON.
pub fn wasm_deserialize_request(json: &str) -> Result<HttpRequest, String> {
    let request: HttpRequest = serde_json::from_str(json).map_err(serialization_error)?;
    request.validate().map_err(public_error)?;
    Ok(request)
}
