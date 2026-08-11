use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::{Command, Output};

use curl_builder_core::{BodyType, HttpMethod, HttpRequest, generate};

fn hostile_json_request() -> HttpRequest {
    HttpRequest {
        method: HttpMethod::POST,
        url: "https://api.example.test/items?q=O%27Reilly&lang=zh".to_string(),
        headers: vec![(
            "X-Client".to_string(),
            "quote=' backslash=\\ unicode=你好".to_string(),
        )],
        body: Some(
            r#"{"enabled":true,"value":null,"tick":"`","escaped":"\u2028","unicode":"你好"}"#
                .to_string(),
        ),
        body_type: BodyType::Json,
    }
}

fn run_tool(program: &str, args: &[&str]) -> Option<Output> {
    match Command::new(program).args(args).output() {
        Ok(output) => Some(output),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            if std::env::var_os("CURL_BUILDER_REQUIRE_SYNTAX_TOOLS").is_some() {
                panic!("required syntax validator {program} is not installed");
            }
            eprintln!("skipping {program} syntax validation because it is not installed");
            None
        }
        Err(error) => panic!("failed to execute {program}: {error}"),
    }
}

fn write_fixture(path: &Path, source: &str) {
    fs::write(path, source).unwrap_or_else(|error| {
        panic!(
            "failed to write generated fixture {}: {error}",
            path.display()
        )
    });
}

fn assert_success(program: &str, output: Output) {
    assert!(
        output.status.success(),
        "{program} rejected generated source:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn generated_language_snippets_are_syntactically_valid() {
    let request = hostile_json_request();
    let fixture_dir = std::env::temp_dir().join(format!(
        "curl-builder-syntax-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::create_dir_all(&fixture_dir).unwrap();

    let fetch_source = generate(&request, "fetch").unwrap();
    assert!(fetch_source.contains("JSON.parse("));
    let fetch_path = fixture_dir.join("fetch.js");
    write_fixture(&fetch_path, &fetch_source);
    if let Some(output) = run_tool("node", &["--check", fetch_path.to_str().unwrap()]) {
        assert_success("node --check (fetch)", output);
    }

    let node_source = generate(&request, "node").unwrap();
    assert!(node_source.contains("JSON.parse("));
    let node_path = fixture_dir.join("node.js");
    write_fixture(&node_path, &node_source);
    if let Some(output) = run_tool("node", &["--check", node_path.to_str().unwrap()]) {
        assert_success("node --check (Node.js)", output);
    }

    let python_source = generate(&request, "python").unwrap();
    assert!(python_source.contains("import json"));
    assert!(python_source.contains("json=json.loads("));
    let python_path = fixture_dir.join("request.py");
    write_fixture(&python_path, &python_source);
    let python_check = concat!(
        "import ast,pathlib,sys; ",
        "ast.parse(pathlib.Path(sys.argv[1]).read_text(encoding='utf-8'))"
    );
    if let Some(output) = run_tool(
        "python3",
        &["-c", python_check, python_path.to_str().unwrap()],
    ) {
        assert_success("python3 ast.parse", output);
    }

    let go_source = generate(&request, "go").unwrap();
    assert!(!go_source.contains("strings.NewReader(`"));
    let go_path = fixture_dir.join("main.go");
    write_fixture(&go_path, &go_source);
    if let Some(output) = run_tool("gofmt", &["-w", go_path.to_str().unwrap()]) {
        assert_success("gofmt", output);
    }

    let rust_source = generate(&request, "rust").unwrap();
    assert!(rust_source.contains("serde_json::from_str"));
    let rust_path = fixture_dir.join("main.rs");
    write_fixture(&rust_path, &rust_source);
    if let Some(output) = run_tool(
        "rustfmt",
        &["--edition", "2024", rust_path.to_str().unwrap()],
    ) {
        assert_success("rustfmt", output);
    }

    fs::remove_dir_all(fixture_dir).unwrap();
}

#[test]
fn rust_generator_supports_options_and_encoded_forms() {
    let mut request = HttpRequest::new(HttpMethod::OPTIONS, "https://example.test/items");
    request.body = Some("name=O%27Reilly&enabled=true".to_string());
    request.body_type = BodyType::FormUrlEncoded;

    let source = generate(&request, "rust").unwrap();

    assert!(source.contains("reqwest::Method::OPTIONS"));
    assert!(source.contains(".body(\"name=O%27Reilly&enabled=true\")"));
    assert!(!source.contains(".options("));
    assert!(!source.contains(".form(&["));
}
