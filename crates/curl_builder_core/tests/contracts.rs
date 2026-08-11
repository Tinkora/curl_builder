use curl_builder_core::{
    BodyType, HttpMethod, HttpRequest, generate, parse_curl, wasm_deserialize_request,
};

fn request_with_json(body: &str) -> HttpRequest {
    HttpRequest {
        method: HttpMethod::POST,
        url: "https://api.example.test/items".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: Some(body.to_string()),
        body_type: BodyType::Json,
    }
}

#[test]
fn curl_user_option_generates_rfc_4648_basic_auth() {
    let request = parse_curl("curl --user admin:secret https://example.test").unwrap();

    assert_eq!(request.authorization(), Some("Basic YWRtaW46c2VjcmV0"));
}

#[test]
fn curl_import_rejects_unknown_options() {
    let error = parse_curl("curl --proxy http://localhost:8080 https://example.test").unwrap_err();

    assert_eq!(error.code(), "UNSUPPORTED_CURL_OPTION");
}

#[test]
fn curl_import_rejects_missing_option_values() {
    let error = parse_curl("curl https://example.test --header").unwrap_err();

    assert_eq!(error.code(), "PARSE_ERROR");
}

#[test]
fn curl_import_rejects_unterminated_quotes() {
    let error = parse_curl("curl 'https://example.test").unwrap_err();

    assert_eq!(error.code(), "PARSE_ERROR");
}

#[test]
fn curl_import_rejects_multiple_urls() {
    let error = parse_curl("curl https://one.example.test https://two.example.test").unwrap_err();

    assert_eq!(error.code(), "PARSE_ERROR");
}

#[test]
fn curl_import_does_not_claim_to_load_body_files() {
    let error = parse_curl("curl --data @payload.json https://example.test").unwrap_err();

    assert_eq!(error.code(), "UNSUPPORTED_CURL_OPTION");
}

#[test]
fn code_generation_rejects_invalid_json_bodies() {
    let error = generate(&request_with_json("{not-json}"), "fetch").unwrap_err();

    assert_eq!(error.code(), "INVALID_JSON");
}

#[test]
fn request_validation_accepts_only_http_and_https_urls() {
    for url in ["file:///tmp/secret", "ftp://example.test/archive"] {
        let request = HttpRequest::new(HttpMethod::GET, url);
        let error = generate(&request, "curl").unwrap_err();

        assert_eq!(error.code(), "INVALID_URL");
    }
}

#[test]
fn request_validation_rejects_url_userinfo() {
    for url in [
        "https://user@example.test/items",
        "https://user:secret@example.test/items",
    ] {
        let error = generate(&HttpRequest::new(HttpMethod::GET, url), "curl").unwrap_err();

        assert_eq!(error.code(), "INVALID_URL");
    }
}

#[test]
fn request_validation_rejects_header_injection() {
    let mut request = HttpRequest::new(HttpMethod::GET, "https://example.test");
    request.headers = vec![("X-Test".to_string(), "safe\r\nInjected: true".to_string())];

    let error = generate(&request, "curl").unwrap_err();

    assert_eq!(error.code(), "INVALID_HEADER");
}

#[test]
fn request_validation_rejects_invalid_header_names() {
    let mut request = HttpRequest::new(HttpMethod::GET, "https://example.test");
    request.headers = vec![("Bad Header".to_string(), "value".to_string())];

    let error = generate(&request, "curl").unwrap_err();

    assert_eq!(error.code(), "INVALID_HEADER");
}

#[test]
fn request_validation_enforces_documented_limits() {
    let too_long_url = format!("https://example.test/{}", "a".repeat(8 * 1024));
    let error = generate(&HttpRequest::new(HttpMethod::GET, too_long_url), "curl").unwrap_err();
    assert_eq!(error.code(), "LIMIT_EXCEEDED");

    let mut too_many_headers = HttpRequest::new(HttpMethod::GET, "https://example.test");
    too_many_headers.headers = (0..101)
        .map(|index| (format!("X-Test-{index}"), "value".to_string()))
        .collect();
    let error = generate(&too_many_headers, "curl").unwrap_err();
    assert_eq!(error.code(), "LIMIT_EXCEEDED");

    let mut too_large_body = HttpRequest::new(HttpMethod::POST, "https://example.test");
    too_large_body.body = Some("a".repeat(1024 * 1024 + 1));
    too_large_body.body_type = BodyType::Raw;
    let error = generate(&too_large_body, "curl").unwrap_err();
    assert_eq!(error.code(), "LIMIT_EXCEEDED");
}

#[test]
fn curl_import_rejects_shell_expansion() {
    for command in [
        "curl https://example.test/$TOKEN",
        "curl \"https://example.test/$(whoami)\"",
        "curl https://example.test/`whoami`",
    ] {
        let error = parse_curl(command).unwrap_err();

        assert_eq!(error.code(), "UNSUPPORTED_SHELL_SYNTAX");
    }
}

#[test]
fn curl_import_supports_documented_long_option_forms() {
    let request = parse_curl(
        "curl --request=PATCH --header='Content-Type: application/json' \
         --data-raw='{\"enabled\":true}' --user=admin:secret https://example.test/items/1",
    )
    .unwrap();

    assert_eq!(request.method, HttpMethod::PATCH);
    assert_eq!(request.body_type, BodyType::Json);
    assert_eq!(request.body.as_deref(), Some("{\"enabled\":true}"));
    assert_eq!(request.authorization(), Some("Basic YWRtaW46c2VjcmV0"));
}

#[test]
fn generated_curl_round_trips_quotes_and_unicode() {
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://example.test/it's?q=%E4%BD%A0%E5%A5%BD".to_string(),
        headers: vec![("X-Label".to_string(), "O'Reilly".to_string())],
        body: Some("It's ready: 你好".to_string()),
        body_type: BodyType::Raw,
    };

    let curl = generate(&request, "curl").unwrap();
    let reparsed = parse_curl(&curl).unwrap();

    assert_eq!(
        reparsed,
        HttpRequest {
            headers: vec![
                ("X-Label".to_string(), "O'Reilly".to_string()),
                ("Content-Type".to_string(), "text/plain".to_string()),
            ],
            ..request
        }
    );
}

#[test]
fn generated_curl_never_interprets_body_as_a_file() {
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://example.test/upload".to_string(),
        headers: vec![],
        body: Some("@/etc/passwd".to_string()),
        body_type: BodyType::Raw,
    };

    let curl = generate(&request, "curl").unwrap();
    let reparsed = parse_curl(&curl).unwrap();

    assert!(curl.contains("--data-raw '@/etc/passwd'"));
    assert_eq!(
        reparsed,
        HttpRequest {
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            ..request
        }
    );
}

#[test]
fn request_validation_rejects_get_and_head_bodies() {
    for method in [HttpMethod::GET, HttpMethod::HEAD] {
        let request = HttpRequest {
            method,
            url: "https://example.test".to_string(),
            headers: vec![],
            body: Some("payload".to_string()),
            body_type: BodyType::Raw,
        };

        let error = generate(&request, "curl").unwrap_err();
        assert_eq!(error.code(), "UNSUPPORTED_BODY_TYPE");
    }
}

#[test]
fn node_generator_uses_parsed_url_scheme() {
    let request = HttpRequest::new(HttpMethod::GET, "HTTPS://example.test/items");

    let source = generate(&request, "node").unwrap();

    assert!(source.contains("const https = require(\"https\")"));
    assert!(source.contains("https.request("));
}

#[test]
fn body_types_produce_consistent_content_types() {
    for (body_type, body, expected) in [
        (BodyType::Json, "{}", "application/json"),
        (
            BodyType::FormUrlEncoded,
            "name=value",
            "application/x-www-form-urlencoded",
        ),
        (BodyType::Raw, "hello", "text/plain"),
        (BodyType::Xml, "<item/>", "application/xml"),
    ] {
        let request = HttpRequest {
            method: HttpMethod::POST,
            url: "https://example.test/items".to_string(),
            headers: vec![],
            body: Some(body.to_string()),
            body_type,
        };

        let curl = generate(&request, "curl").unwrap();
        assert!(curl.contains(&format!("-H 'Content-Type: {expected}'")));

        let reparsed = parse_curl(&curl).unwrap();
        assert_eq!(reparsed.body_type, body_type);
    }
}

#[test]
fn request_validation_rejects_conflicting_content_types() {
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: "https://example.test/items".to_string(),
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: Some("{}".to_string()),
        body_type: BodyType::Json,
    };

    let error = generate(&request, "curl").unwrap_err();

    assert_eq!(error.code(), "UNSUPPORTED_BODY_TYPE");
}

#[test]
fn request_validation_rejects_duplicate_headers() {
    let mut request = HttpRequest::new(HttpMethod::GET, "https://example.test");
    request.headers = vec![
        ("Accept".to_string(), "application/json".to_string()),
        ("accept".to_string(), "text/plain".to_string()),
    ];

    let error = generate(&request, "curl").unwrap_err();

    assert_eq!(error.code(), "INVALID_HEADER");
}

#[test]
fn curl_import_uses_content_type_instead_of_body_shape() {
    let raw = parse_curl(
        "curl -H 'Content-Type: application/octet-stream' --data-raw '{not-json}' \
         https://example.test",
    )
    .unwrap();
    assert_eq!(raw.body_type, BodyType::Raw);

    let multipart = parse_curl(
        "curl -H 'Content-Type: multipart/form-data' --data-raw 'value' \
         https://example.test",
    )
    .unwrap_err();
    assert_eq!(multipart.code(), "UNSUPPORTED_BODY_TYPE");
}

#[test]
fn curl_import_rejects_unquoted_shell_expansion_forms() {
    for command in [
        "curl https://example.test/{one,two}",
        "curl https://example.test/*.json",
        "curl https://example.test/?page=1",
        "curl https://example.test/[ab]",
        "curl https://example.test/(item)",
        "curl\nhttps://example.test",
    ] {
        let error = parse_curl(command).unwrap_err();

        assert_eq!(error.code(), "UNSUPPORTED_SHELL_SYNTAX");
    }
}

#[test]
fn request_json_uses_the_versioned_wire_names() {
    let request = request_with_json("{}");
    let json = serde_json::to_string(&request).unwrap();

    assert!(json.contains("\"body_type\":\"json\""));
    assert_eq!(
        serde_json::from_str::<HttpRequest>(
            r#"{"method":"POST","url":"https://example.test","headers":[],"body":"a=1","body_type":"form_urlencoded"}"#,
        )
        .unwrap()
        .body_type,
        BodyType::FormUrlEncoded
    );
}

#[test]
fn wasm_deserialization_returns_stable_errors_and_validates() {
    let error = wasm_deserialize_request(
        r#"{"method":"GET","url":"file:///tmp/secret","headers":[],"body":null,"body_type":"none"}"#,
    )
    .unwrap_err();
    let error: serde_json::Value = serde_json::from_str(&error).unwrap();

    assert_eq!(error["code"], "INVALID_URL");
    assert!(error["message"].is_string());
}

#[test]
fn basic_auth_encodes_utf8_and_colons_without_logging_credentials() {
    let request = parse_curl("curl --user '用户:p:a:ss' https://example.test").unwrap();

    assert_eq!(request.authorization(), Some("Basic 55So5oi3OnA6YTpzcw=="));
}
