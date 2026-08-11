use curl_builder_core::{
    BodyType, CoreError, HttpMethod, HttpRequest, generate, generate_all as generate_all_snippets,
    language_list, parse_curl,
};
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// Error conversion
// ---------------------------------------------------------------------------

/// Converts a CoreError into a JsValue with stable `code` and `message` fields.
fn core_err(e: CoreError) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &e.code().into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &e.to_string().into()).ok();
    obj.into()
}

fn serde_wasm<T: serde::Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|error| core_err(CoreError::SerializationError(error.to_string())))
}

// ---------------------------------------------------------------------------
// WASM exports
// ---------------------------------------------------------------------------

/// Generate code for a specific language from a JS HttpRequest object.
#[wasm_bindgen]
pub fn generate_code(request_js: &JsValue, language: &str) -> Result<String, JsValue> {
    let request: HttpRequest = serde_wasm_bindgen::from_value(request_js.clone())
        .map_err(|error| core_err(CoreError::SerializationError(error.to_string())))?;

    generate(&request, language).map_err(core_err)
}

/// Generate all 6 language snippets at once. Returns a JS object
/// with keys: curl, fetch, python, go, rust, node.
#[wasm_bindgen]
pub fn generate_all(request_js: &JsValue) -> Result<JsValue, JsValue> {
    let request: HttpRequest = serde_wasm_bindgen::from_value(request_js.clone())
        .map_err(|error| core_err(CoreError::SerializationError(error.to_string())))?;

    let result = js_sys::Object::new();
    for (language, source) in generate_all_snippets(&request).map_err(core_err)? {
        js_sys::Reflect::set(&result, &language.into(), &source.into()).map_err(|_| {
            core_err(CoreError::SerializationError(
                "failed to build generated output".to_string(),
            ))
        })?;
    }

    Ok(result.into())
}

/// Parse a cURL command string into an HttpRequest JS object.
#[wasm_bindgen]
pub fn parse_curl_command(curl_command: &str) -> Result<JsValue, JsValue> {
    let request = parse_curl(curl_command).map_err(core_err)?;
    serde_wasm(&request)
}

/// List supported languages as a JS array.
#[wasm_bindgen]
pub fn list_languages() -> JsValue {
    let arr = js_sys::Array::new();
    for lang in language_list() {
        arr.push(&JsValue::from_str(lang));
    }
    arr.into()
}

/// Validate a URL string. Returns `{ valid: true, url: "..." }` or error.
#[wasm_bindgen]
pub fn validate_url(url: &str) -> Result<JsValue, JsValue> {
    HttpRequest::new(HttpMethod::GET, url)
        .validate_url()
        .map_err(core_err)?;
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"valid".into(), &true.into()).ok();
    js_sys::Reflect::set(&obj, &"url".into(), &url.into()).ok();
    Ok(obj.into())
}

/// Create an empty HttpRequest as a JS object.
#[wasm_bindgen]
pub fn create_empty_request() -> JsValue {
    let req = HttpRequest::empty();
    serde_wasm(&req).unwrap_or(JsValue::NULL)
}

/// Serialize a JS HttpRequest to JSON string.
#[wasm_bindgen]
pub fn serialize_request(request_js: &JsValue) -> Result<String, JsValue> {
    let request: HttpRequest = serde_wasm_bindgen::from_value(request_js.clone())
        .map_err(|error| core_err(CoreError::SerializationError(error.to_string())))?;
    request.validate().map_err(core_err)?;
    serde_json::to_string(&request)
        .map_err(|error| core_err(CoreError::SerializationError(error.to_string())))
}

/// Get method info (all HTTP methods with labels and color classes) as a JS array.
#[wasm_bindgen]
pub fn get_methods() -> JsValue {
    let arr = js_sys::Array::new();
    for method in HttpMethod::all() {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"name".into(), &method.as_str().into()).ok();
        js_sys::Reflect::set(&obj, &"color".into(), &method.color_class().into()).ok();
        js_sys::Reflect::set(&obj, &"labelZh".into(), &method.label_zh().into()).ok();
        js_sys::Reflect::set(&obj, &"allowsBody".into(), &method.allows_body().into()).ok();
        arr.push(&obj);
    }
    arr.into()
}

/// Get body type info as a JS array.
#[wasm_bindgen]
pub fn get_body_types() -> JsValue {
    let types = [
        BodyType::None,
        BodyType::Json,
        BodyType::FormUrlEncoded,
        BodyType::Raw,
        BodyType::Xml,
    ];
    let arr = js_sys::Array::new();
    for bt in &types {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"name".into(), &bt.as_str().into()).ok();
        js_sys::Reflect::set(&obj, &"label".into(), &bt.label().into()).ok();
        js_sys::Reflect::set(&obj, &"labelZh".into(), &bt.label_zh().into()).ok();
        js_sys::Reflect::set(
            &obj,
            &"contentType".into(),
            &bt.content_type_header().unwrap_or("").into(),
        )
        .ok();
        arr.push(&obj);
    }
    arr.into()
}
