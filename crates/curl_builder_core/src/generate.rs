use base64::Engine as _;

use crate::error::CoreError;
use crate::request::{BodyType, HttpMethod, HttpRequest, MAX_INPUT_BYTES};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate a cURL command string from an HTTP request.
pub fn to_curl(request: &HttpRequest) -> String {
    let mut parts: Vec<String> = Vec::new();
    let headers = effective_headers(request);
    parts.push("curl".to_string());

    // Method (skip for GET which is default)
    if request.method != HttpMethod::GET {
        parts.push("-X".to_string());
        parts.push(request.method.as_str().to_string());
    }

    // URL
    parts.push(shell_quote(&request.url));

    // Headers
    for (key, value) in headers {
        parts.push("-H".to_string());
        parts.push(shell_quote(&format!("{key}: {value}")));
    }

    // Body
    if let Some(ref body) = request.body
        && !body.is_empty()
    {
        parts.push("--data-raw".to_string());
        parts.push(shell_quote(body));
    }

    parts.join(" ")
}

/// Generate a JavaScript `fetch()` code snippet.
pub fn to_fetch(request: &HttpRequest) -> String {
    let mut out = String::new();
    let headers = effective_headers(request);

    // Build options object
    let mut opts = Vec::new();
    opts.push(format!(
        "  method: {}",
        js_string_literal(request.method.as_str())
    ));

    // Headers
    if !headers.is_empty() {
        let hdr_lines: Vec<String> = headers
            .iter()
            .map(|(key, value)| {
                format!(
                    "    {}: {}",
                    js_string_literal(key),
                    js_string_literal(value)
                )
            })
            .collect();
        opts.push(format!("  headers: {{\n{}\n  }}", hdr_lines.join(",\n")));
    }

    // Body
    if let Some(ref body) = request.body
        && !body.is_empty()
    {
        match request.body_type {
            BodyType::Json => {
                opts.push(format!(
                    "  body: JSON.stringify(JSON.parse({}))",
                    js_string_literal(body)
                ));
            }
            _ => {
                opts.push(format!("  body: {}", js_string_literal(body)));
            }
        }
    }

    out.push_str(&format!(
        "fetch({}, {{\n{}\n}})",
        js_string_literal(&request.url),
        opts.join(",\n")
    ));

    // Append .then chain for completeness
    out.push_str("\n  .then(response => response.text())\n  .then(body => console.log(body))\n  .catch(error => console.error('Error:', error));");

    out
}

/// Generate a Python `requests` code snippet.
pub fn to_python_requests(request: &HttpRequest) -> String {
    let mut out = String::new();
    let headers = effective_headers(request);
    if request.body_type == BodyType::Json
        && request.body.as_ref().is_some_and(|body| !body.is_empty())
    {
        out.push_str("import json\n");
    }
    out.push_str("import requests\n\n");

    // Build kwargs
    let mut kwargs: Vec<String> = Vec::new();
    kwargs.push(format!("    {}", py_string_literal(&request.url)));

    // Headers
    if !headers.is_empty() {
        let hdr_items: Vec<String> = headers
            .iter()
            .map(|(key, value)| {
                format!(
                    "        {}: {}",
                    py_string_literal(key),
                    py_string_literal(value)
                )
            })
            .collect();
        kwargs.push(format!("    headers={{\n{}\n    }}", hdr_items.join(",\n")));
    }

    // Body
    if let Some(ref body) = request.body
        && !body.is_empty()
    {
        match request.body_type {
            BodyType::Json => {
                kwargs.push(format!("    json=json.loads({})", py_string_literal(body)));
            }
            BodyType::FormUrlEncoded => {
                kwargs.push(format!("    data={}", py_string_literal(body)));
            }
            _ => {
                kwargs.push(format!("    data={}", py_string_literal(body)));
            }
        }
    }

    let method = request.method.as_str().to_lowercase();
    out.push_str(&format!(
        "response = requests.{}(\n{}\n)",
        method,
        kwargs.join(",\n")
    ));

    out.push_str("\n\nprint(response.status_code)\nprint(response.text)");

    out
}

/// Generate a Go `http.NewRequest` code snippet.
pub fn to_go_http(request: &HttpRequest) -> String {
    let mut out = String::new();
    let headers = effective_headers(request);
    out.push_str("package main\n\n");
    out.push_str("import (\n");
    out.push_str("    \"fmt\"\n");
    out.push_str("    \"io\"\n");
    out.push_str("    \"net/http\"\n");

    if request.body.as_ref().is_some_and(|body| !body.is_empty()) {
        out.push_str("    \"strings\"\n");
    }

    out.push_str(")\n\n");
    out.push_str("func main() {\n");

    // Body variable if present
    if let Some(ref body) = request.body {
        if !body.is_empty() {
            out.push_str(&format!(
                "    body := strings.NewReader({})\n",
                go_string_literal(body)
            ));
            out.push_str(&format!(
                "    req, err := http.NewRequest({}, {}, body)\n",
                go_string_literal(request.method.as_str()),
                go_string_literal(&request.url)
            ));
        } else {
            out.push_str(&format!(
                "    req, err := http.NewRequest({}, {}, nil)\n",
                go_string_literal(request.method.as_str()),
                go_string_literal(&request.url)
            ));
        }
    } else {
        out.push_str(&format!(
            "    req, err := http.NewRequest({}, {}, nil)\n",
            go_string_literal(request.method.as_str()),
            go_string_literal(&request.url)
        ));
    }

    out.push_str("    if err != nil {\n        panic(err)\n    }\n");

    // Headers
    for (key, value) in headers {
        out.push_str(&format!(
            "    req.Header.Set({}, {})\n",
            go_string_literal(key),
            go_string_literal(value)
        ));
    }

    out.push_str("\n    resp, err := http.DefaultClient.Do(req)\n");
    out.push_str("    if err != nil {\n        panic(err)\n    }\n");
    out.push_str("    defer resp.Body.Close()\n\n");
    out.push_str("    bodyBytes, _ := io.ReadAll(resp.Body)\n");
    out.push_str("    fmt.Println(string(bodyBytes))\n");
    out.push_str("}\n");

    out
}

/// Generate a Rust `reqwest` code snippet.
pub fn to_rust_reqwest(request: &HttpRequest) -> String {
    let mut out = String::new();
    let headers = effective_headers(request);

    out.push_str("#[tokio::main]\n");
    out.push_str("async fn main() -> Result<(), Box<dyn std::error::Error>> {\n");
    out.push_str("    let client = reqwest::Client::new();\n");

    let has_json_body = request.body_type == BodyType::Json
        && request.body.as_ref().is_some_and(|body| !body.is_empty());
    if has_json_body {
        let body = request.body.as_deref().expect("JSON body checked above");
        out.push_str(&format!(
            "    let json_body: serde_json::Value = serde_json::from_str({})?;\n",
            rust_string_literal(body)
        ));
    }

    out.push_str(&format!(
        "    let resp = client.request(reqwest::Method::{}, {})\n",
        request.method.as_str(),
        rust_string_literal(&request.url)
    ));

    // Headers
    for (key, value) in headers {
        out.push_str(&format!(
            "        .header({}, {})\n",
            rust_string_literal(key),
            rust_string_literal(value)
        ));
    }

    // Body
    if let Some(ref body) = request.body
        && !body.is_empty()
    {
        match request.body_type {
            BodyType::Json => {
                out.push_str("        .json(&json_body)\n");
            }
            BodyType::FormUrlEncoded => {
                out.push_str(&format!("        .body({})\n", rust_string_literal(body)));
            }
            _ => {
                out.push_str(&format!("        .body({})\n", rust_string_literal(body)));
            }
        }
    }

    out.push_str("        .send()\n");
    out.push_str("        .await?;\n\n");
    out.push_str("    println!(\"{:?}\", resp.text().await?);\n");
    out.push_str("    Ok(())\n");
    out.push_str("}\n");

    out
}

/// Generate a Node.js `http`/`https` code snippet.
pub fn to_node_http(request: &HttpRequest) -> String {
    let mut out = String::new();
    let headers = effective_headers(request);

    let is_https = url::Url::parse(&request.url)
        .is_ok_and(|parsed| parsed.scheme().eq_ignore_ascii_case("https"));
    let module = if is_https { "https" } else { "http" };

    // Parse URL to extract hostname, path, port
    let (hostname, path, port) = parse_url_parts(&request.url);

    out.push_str(&format!(
        "const {} = require({});\n\n",
        module,
        js_string_literal(module)
    ));

    // Body data variable if present
    let has_body = request.body.as_ref().is_some_and(|body| !body.is_empty());
    let data_var = if has_body { "data" } else { "null" };

    if has_body {
        let body = request.body.as_ref().unwrap();
        match request.body_type {
            BodyType::Json => {
                out.push_str(&format!(
                    "const data = JSON.stringify(JSON.parse({}));\n\n",
                    js_string_literal(body)
                ));
            }
            _ => {
                out.push_str(&format!("const data = {};\n\n", js_string_literal(body)));
            }
        }
    }

    // Options object
    out.push_str("const options = {\n");
    out.push_str(&format!("  hostname: {},\n", js_string_literal(&hostname)));
    out.push_str(&format!("  path: {},\n", js_string_literal(&path)));
    out.push_str(&format!(
        "  method: {},\n",
        js_string_literal(request.method.as_str())
    ));

    if !port.is_empty() {
        out.push_str(&format!("  port: {},\n", port));
    }

    // Headers
    if !headers.is_empty() || has_body {
        out.push_str("  headers: {\n");
        for (key, value) in &headers {
            out.push_str(&format!(
                "    {}: {},\n",
                js_string_literal(key),
                js_string_literal(value)
            ));
        }
        if has_body
            && !headers
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        {
            out.push_str(&format!(
                "    'Content-Length': Buffer.byteLength({}),\n",
                data_var
            ));
        }
        out.push_str("  },\n");
    }

    out.push_str("};\n\n");

    // Request
    out.push_str(&format!(
        "const req = {}.request(options, (res) => {{\n",
        module
    ));
    out.push_str("  let body = '';\n");
    out.push_str("  res.on('data', (chunk) => { body += chunk; });\n");
    out.push_str("  res.on('end', () => { console.log(body); });\n");
    out.push_str("});\n\n");
    out.push_str("req.on('error', (error) => { console.error(error); });\n");

    if has_body {
        out.push_str(&format!("req.write({});\n", data_var));
    }

    out.push_str("req.end();\n");

    out
}

/// List of supported language identifiers.
pub fn language_list() -> &'static [&'static str] {
    &["curl", "fetch", "python", "go", "rust", "node"]
}

/// Generate code for a specific language.
pub fn generate(request: &HttpRequest, language: &str) -> Result<String, CoreError> {
    request.validate()?;
    generate_validated(request, language)
}

fn generate_validated(request: &HttpRequest, language: &str) -> Result<String, CoreError> {
    match language {
        "curl" => Ok(to_curl(request)),
        "fetch" => Ok(to_fetch(request)),
        "python" => Ok(to_python_requests(request)),
        "go" => Ok(to_go_http(request)),
        "rust" => Ok(to_rust_reqwest(request)),
        "node" => Ok(to_node_http(request)),
        other => Err(CoreError::UnsupportedLanguage(other.to_string())),
    }
}

/// Generate all supported snippets after validating the request once.
pub fn generate_all(request: &HttpRequest) -> Result<Vec<(&'static str, String)>, CoreError> {
    request.validate()?;
    language_list()
        .iter()
        .map(|language| generate_validated(request, language).map(|source| (*language, source)))
        .collect()
}

// ---------------------------------------------------------------------------
// cURL Parser
// ---------------------------------------------------------------------------

/// Parse a cURL command string into an `HttpRequest`.
///
/// Supports flags:
/// - `-X` / `--request` → method
/// - `-H` / `--header` → header
/// - `-d` / `--data` / `--data-raw` / `--data-binary` → body
/// - `-u` / `--user` → Basic Auth (converted to Authorization header)
/// - `--compressed` → Accept-Encoding header
///
/// The URL is taken as the last positional argument (or the argument before
/// the first flag, if the command starts with `curl`).
pub fn parse_curl(input: &str) -> Result<HttpRequest, CoreError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(CoreError::LimitExceeded(format!(
            "cURL input must not exceed {MAX_INPUT_BYTES} bytes"
        )));
    }

    let tokens = tokenize_curl(input.trim())?;
    if tokens.first().map(String::as_str) != Some("curl") {
        return Err(CoreError::ParseError(
            "command must start with curl".to_string(),
        ));
    }

    let mut method: Option<HttpMethod> = None;
    let mut url: Option<String> = None;
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut body: Option<String> = None;
    let mut body_type = BodyType::None;

    let mut i = 1;
    while i < tokens.len() {
        let tok = &tokens[i];

        match tok.as_str() {
            "-X" | "--request" => {
                let value = take_option_value(&tokens, &mut i, tok)?;
                method = Some(parse_method(value)?);
            }
            "-H" | "--header" => {
                let value = take_option_value(&tokens, &mut i, tok)?;
                headers.push(parse_header(value)?);
            }
            "-d" | "--data" | "--data-raw" | "--data-binary" => {
                let option = tok.as_str();
                let value = take_option_value(&tokens, &mut i, option)?;
                set_body(&mut body, value, option)?;
            }
            "-u" | "--user" => {
                let credentials = take_option_value(&tokens, &mut i, tok)?;
                add_basic_auth(&mut headers, credentials)?;
            }
            "--compressed" => {
                headers.push(("Accept-Encoding".to_string(), "gzip, deflate".to_string()));
            }
            option if option.starts_with("--") && option.contains('=') => {
                let (name, value) = option.split_once('=').expect("contains '='");
                if value.is_empty() {
                    return Err(CoreError::ParseError(format!("{name} requires a value")));
                }

                match name {
                    "--request" => method = Some(parse_method(value)?),
                    "--header" => headers.push(parse_header(value)?),
                    "--data" | "--data-raw" | "--data-binary" => {
                        set_body(&mut body, value, name)?;
                    }
                    "--user" => add_basic_auth(&mut headers, value)?,
                    _ => {
                        return Err(CoreError::UnsupportedCurlOption(name.to_string()));
                    }
                }
            }
            option if option.starts_with('-') => {
                return Err(CoreError::UnsupportedCurlOption(option.to_string()));
            }
            other => {
                if url.replace(other.to_string()).is_some() {
                    return Err(CoreError::ParseError(
                        "only one request URL is supported".to_string(),
                    ));
                }
            }
        }
        i += 1;
    }

    // If method wasn't specified but body exists, default to POST
    if method.is_none() && body.is_some() {
        method = Some(HttpMethod::POST);
    }

    if body.is_some() {
        body_type = body_type_from_headers(&headers)?.unwrap_or(BodyType::FormUrlEncoded);
    }

    let request = HttpRequest {
        method: method.unwrap_or(HttpMethod::GET),
        url: url.unwrap_or_default(),
        headers,
        body,
        body_type,
    };

    request.validate()?;
    Ok(request)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn effective_headers(request: &HttpRequest) -> Vec<(&str, &str)> {
    let mut headers: Vec<(&str, &str)> = request
        .headers
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();

    let has_body = request.body.as_ref().is_some_and(|body| !body.is_empty());
    let has_content_type = headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("content-type"));
    if has_body
        && !has_content_type
        && let Some(content_type) = request.body_type.content_type_header()
    {
        headers.push(("Content-Type", content_type));
    }

    headers
}

/// Quote a string for shell (single quotes, escaping any embedded single quotes).
fn shell_quote(s: &str) -> String {
    if s.is_empty() {
        return "''".to_string();
    }
    // Use single quotes; replace any internal ' with '\''
    let escaped = s.replace('\'', "'\\''");
    format!("'{escaped}'")
}

fn js_string_literal(value: &str) -> String {
    serde_json::to_string(value)
        .expect("serializing a Rust string cannot fail")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

fn py_string_literal(value: &str) -> String {
    let mut literal = String::from("'");
    for character in value.chars() {
        match character {
            '\\' => literal.push_str("\\\\"),
            '\'' => literal.push_str("\\'"),
            '\n' => literal.push_str("\\n"),
            '\r' => literal.push_str("\\r"),
            '\t' => literal.push_str("\\t"),
            '\u{0008}' => literal.push_str("\\b"),
            '\u{000c}' => literal.push_str("\\f"),
            control if control.is_control() => {
                use std::fmt::Write as _;
                write!(literal, "\\u{:04x}", control as u32)
                    .expect("writing to a String cannot fail");
            }
            other => literal.push(other),
        }
    }
    literal.push('\'');
    literal
}

fn go_string_literal(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a Rust string cannot fail")
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

fn take_option_value<'a>(
    tokens: &'a [String],
    index: &mut usize,
    option: &str,
) -> Result<&'a str, CoreError> {
    *index += 1;
    tokens
        .get(*index)
        .map(String::as_str)
        .ok_or_else(|| CoreError::ParseError(format!("{option} requires a value")))
}

fn parse_method(value: &str) -> Result<HttpMethod, CoreError> {
    HttpMethod::parse(value)
        .ok_or_else(|| CoreError::ParseError("unsupported HTTP method".to_string()))
}

fn parse_header(s: &str) -> Result<(String, String), CoreError> {
    let colon = s.find(':').ok_or_else(|| {
        CoreError::ParseError("headers must use the 'Name: Value' format".to_string())
    })?;
    let key = s[..colon].trim().to_string();
    let value = s[colon + 1..].trim().to_string();
    if key.is_empty() {
        Err(CoreError::ParseError(
            "header names must not be empty".to_string(),
        ))
    } else {
        Ok((key, value))
    }
}

fn set_body(body: &mut Option<String>, value: &str, option: &str) -> Result<(), CoreError> {
    if body.is_some() {
        return Err(CoreError::ParseError(
            "multiple request body options are not supported".to_string(),
        ));
    }

    if option != "--data-raw" && value.starts_with('@') {
        return Err(CoreError::UnsupportedCurlOption(
            "file-backed request bodies are not supported".to_string(),
        ));
    }

    *body = Some(value.to_string());
    Ok(())
}

fn add_basic_auth(headers: &mut Vec<(String, String)>, credentials: &str) -> Result<(), CoreError> {
    if !credentials.contains(':') {
        return Err(CoreError::ParseError(
            "--user requires an explicit user:password value".to_string(),
        ));
    }

    let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
    headers.push(("Authorization".to_string(), format!("Basic {encoded}")));
    Ok(())
}

fn body_type_from_headers(headers: &[(String, String)]) -> Result<Option<BodyType>, CoreError> {
    let content_type = headers
        .iter()
        .rev()
        .find(|(name, _)| name.eq_ignore_ascii_case("content-type"));
    let Some((_, content_type)) = content_type else {
        return Ok(None);
    };
    let media_type = content_type.split(';').next().unwrap_or_default().trim();

    if media_type.eq_ignore_ascii_case("multipart/form-data") {
        return Err(CoreError::UnsupportedBodyType(
            "multipart/form-data is outside the Alpha contract".to_string(),
        ));
    }

    Ok(Some(BodyType::parse(media_type).unwrap_or(BodyType::Raw)))
}

/// Tokenize a cURL command string into arguments, handling quotes.
fn tokenize_curl(input: &str) -> Result<Vec<String>, CoreError> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum QuoteMode {
        Unquoted,
        Single,
        Double,
    }

    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut current = String::new();
    let mut token_started = false;
    let mut mode = QuoteMode::Unquoted;
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        match mode {
            QuoteMode::Unquoted => match ch {
                '\n' | '\r' => return Err(CoreError::UnsupportedShellSyntax),
                c if c.is_whitespace() => {
                    if token_started {
                        tokens.push(std::mem::take(&mut current));
                        token_started = false;
                    }
                    i += 1;
                }
                '\'' => {
                    token_started = true;
                    mode = QuoteMode::Single;
                    i += 1;
                }
                '"' => {
                    token_started = true;
                    mode = QuoteMode::Double;
                    i += 1;
                }
                '\\' => {
                    let next = chars.get(i + 1).copied().ok_or_else(|| {
                        CoreError::ParseError("trailing backslash in command".to_string())
                    })?;
                    if next == '\n' {
                        i += 2;
                    } else if next == '\r' && chars.get(i + 2) == Some(&'\n') {
                        i += 3;
                    } else {
                        token_started = true;
                        current.push(next);
                        i += 2;
                    }
                }
                '$' | '`' | ';' | '|' | '&' | '<' | '>' | '{' | '}' | '*' | '?' | '[' | ']'
                | '(' | ')' | '#' => {
                    return Err(CoreError::UnsupportedShellSyntax);
                }
                _ => {
                    token_started = true;
                    current.push(ch);
                    i += 1;
                }
            },
            QuoteMode::Single => {
                if ch == '\'' {
                    mode = QuoteMode::Unquoted;
                } else {
                    current.push(ch);
                }
                i += 1;
            }
            QuoteMode::Double => match ch {
                '"' => {
                    mode = QuoteMode::Unquoted;
                    i += 1;
                }
                '\\' => {
                    let next = chars.get(i + 1).copied().ok_or_else(|| {
                        CoreError::ParseError("trailing backslash in command".to_string())
                    })?;
                    if next == '\n' {
                        i += 2;
                    } else if next == '\r' && chars.get(i + 2) == Some(&'\n') {
                        i += 3;
                    } else if matches!(next, '\\' | '"' | '$' | '`') {
                        current.push(next);
                        i += 2;
                    } else {
                        current.push('\\');
                        current.push(next);
                        i += 2;
                    }
                }
                '$' | '`' => return Err(CoreError::UnsupportedShellSyntax),
                _ => {
                    current.push(ch);
                    i += 1;
                }
            },
        }
    }

    if mode != QuoteMode::Unquoted {
        return Err(CoreError::ParseError(
            "unterminated quote in command".to_string(),
        ));
    }

    if token_started {
        tokens.push(current);
    }

    Ok(tokens)
}

/// Parse hostname, path, and port from a URL string.
fn parse_url_parts(url: &str) -> (String, String, String) {
    if let Ok(parsed) = url::Url::parse(url) {
        let host = parsed.host_str().unwrap_or("localhost").to_string();
        let path = {
            let p = parsed.path();
            if let Some(q) = parsed.query() {
                format!("{p}?{q}")
            } else {
                p.to_string()
            }
        };
        let port = parsed.port().map(|p| p.to_string()).unwrap_or_default();
        (host, path, port)
    } else {
        // Fallback: treat as hostname with path
        let without_scheme = url
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        if let Some(slash) = without_scheme.find('/') {
            let host = without_scheme[..slash].to_string();
            let path = without_scheme[slash..].to_string();
            (host, path, String::new())
        } else {
            (without_scheme.to_string(), "/".to_string(), String::new())
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::HttpMethod;

    fn make_request(
        method: HttpMethod,
        url: &str,
        headers: Vec<(&str, &str)>,
        body: Option<&str>,
        body_type: BodyType,
    ) -> HttpRequest {
        HttpRequest {
            method,
            url: url.to_string(),
            headers: headers
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            body: body.map(|s| s.to_string()),
            body_type,
        }
    }

    #[test]
    fn test_curl_simple_get() {
        let req = make_request(
            HttpMethod::GET,
            "https://example.com",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_curl(&req);
        assert_eq!(out, "curl 'https://example.com'");
    }

    #[test]
    fn test_curl_post_json() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/users",
            vec![("Content-Type", "application/json")],
            Some("{\"name\":\"John\"}"),
            BodyType::Json,
        );
        let out = to_curl(&req);
        assert!(out.contains("curl -X POST"));
        assert!(out.contains("'https://api.example.com/users'"));
        assert!(out.contains("-H 'Content-Type: application/json'"));
        assert!(out.contains("--data-raw '{\"name\":\"John\"}'"));
    }

    #[test]
    fn test_curl_with_single_quote_escape() {
        let req = make_request(
            HttpMethod::GET,
            "https://example.com/it's",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_curl(&req);
        assert!(out.contains("'https://example.com/it'\\''s'"));
    }

    #[test]
    fn test_fetch_post() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/data",
            vec![("Authorization", "Bearer abc123")],
            Some("{\"key\":\"value\"}"),
            BodyType::Json,
        );
        let out = to_fetch(&req);
        assert!(out.contains("fetch(\"https://api.example.com/data\""));
        assert!(out.contains("method: \"POST\""));
        assert!(out.contains("\"Authorization\": \"Bearer abc123\""));
        assert!(out.contains("JSON.stringify"));
    }

    #[test]
    fn test_python_get() {
        let req = make_request(
            HttpMethod::GET,
            "https://httpbin.org/get",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_python_requests(&req);
        assert!(out.contains("import requests"));
        assert!(out.contains("requests.get("));
        assert!(out.contains("'https://httpbin.org/get'"));
    }

    #[test]
    fn test_python_post_json() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/users",
            vec![("Content-Type", "application/json")],
            Some("{\"name\": \"Jane\"}"),
            BodyType::Json,
        );
        let out = to_python_requests(&req);
        assert!(out.contains("requests.post("));
        assert!(out.contains("\"name\": \"Jane\""));
    }

    #[test]
    fn test_go_get() {
        let req = make_request(
            HttpMethod::GET,
            "https://example.com",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_go_http(&req);
        assert!(out.contains("package main"));
        assert!(out.contains("\"net/http\""));
        assert!(out.contains("http.NewRequest(\"GET\", \"https://example.com\", nil)"));
    }

    #[test]
    fn test_go_post_with_body() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/data",
            vec![("Content-Type", "application/json")],
            Some("{\"foo\":\"bar\"}"),
            BodyType::Json,
        );
        let out = to_go_http(&req);
        assert!(out.contains("strings.NewReader"));
        assert!(out.contains("http.NewRequest(\"POST\""));
        assert!(out.contains("req.Header.Set(\"Content-Type\", \"application/json\")"));
    }

    #[test]
    fn test_rust_reqwest_get() {
        let req = make_request(
            HttpMethod::GET,
            "https://example.com",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_rust_reqwest(&req);
        assert!(out.contains("reqwest::Client::new()"));
        assert!(out.contains("client.request(reqwest::Method::GET, \"https://example.com\")"));
        assert!(out.contains(".send()"));
    }

    #[test]
    fn test_rust_reqwest_post_json() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/users",
            vec![],
            Some("{\"name\": \"John\"}"),
            BodyType::Json,
        );
        let out = to_rust_reqwest(&req);
        assert!(out.contains("client.request(reqwest::Method::POST"));
        assert!(out.contains("serde_json::from_str"));
        assert!(out.contains(".json(&json_body)"));
    }

    #[test]
    fn test_node_http_get() {
        let req = make_request(
            HttpMethod::GET,
            "https://example.com/api",
            vec![],
            None,
            BodyType::None,
        );
        let out = to_node_http(&req);
        assert!(out.contains("const https = require(\"https\")"));
        assert!(out.contains("hostname: \"example.com\""));
        assert!(out.contains("path: \"/api\""));
    }

    #[test]
    fn test_node_http_post() {
        let req = make_request(
            HttpMethod::POST,
            "https://api.example.com/data",
            vec![("Content-Type", "application/json")],
            Some("{\"x\":1}"),
            BodyType::Json,
        );
        let out = to_node_http(&req);
        assert!(out.contains("JSON.stringify"));
        assert!(out.contains("method: \"POST\""));
        assert!(out.contains("req.write(data)"));
    }

    // ---- cURL parser tests ----

    #[test]
    fn test_parse_curl_simple() {
        let result = parse_curl("curl https://example.com").unwrap();
        assert_eq!(result.method, HttpMethod::GET);
        assert_eq!(result.url, "https://example.com");
    }

    #[test]
    fn test_parse_curl_post_with_data() {
        let result =
            parse_curl("curl -X POST https://api.example.com -H 'Content-Type: application/json' -d '{\"key\":\"val\"}'")
                .unwrap();
        assert_eq!(result.method, HttpMethod::POST);
        assert_eq!(result.url, "https://api.example.com");
        assert_eq!(result.headers.len(), 1);
        assert_eq!(result.headers[0].0, "Content-Type");
        assert_eq!(result.headers[0].1, "application/json");
        assert!(result.body.unwrap().contains("\"key\":\"val\""));
    }

    #[test]
    fn test_parse_curl_with_user() {
        let result = parse_curl("curl -u admin:secret https://example.com").unwrap();
        assert_eq!(
            result
                .headers
                .iter()
                .find(|(k, _)| k == "Authorization")
                .map(|(_, v)| v.as_str()),
            Some("Basic YWRtaW46c2VjcmV0")
        );
    }

    #[test]
    fn test_parse_curl_with_compressed() {
        let result = parse_curl("curl --compressed https://example.com").unwrap();
        assert!(
            result
                .headers
                .iter()
                .any(|(k, v)| k == "Accept-Encoding" && v == "gzip, deflate")
        );
    }

    #[test]
    fn test_generate_all_languages() {
        let req = make_request(
            HttpMethod::POST,
            "https://example.com/api",
            vec![("Accept", "application/json")],
            Some("hello"),
            BodyType::Raw,
        );
        for lang in language_list() {
            let code = generate(&req, lang).unwrap();
            assert!(!code.is_empty(), "Empty output for language: {lang}");
        }
    }

    #[test]
    fn test_language_list() {
        let list = language_list();
        assert_eq!(list.len(), 6);
        assert!(list.contains(&"curl"));
        assert!(list.contains(&"fetch"));
        assert!(list.contains(&"python"));
        assert!(list.contains(&"go"));
        assert!(list.contains(&"rust"));
        assert!(list.contains(&"node"));
    }
}
