# Curl Builder

[简体中文](README.zh-CN.md)

[![Support Tinkora on Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/tinkora)

[![Pages](https://github.com/Tinkora/curl_builder/actions/workflows/pages.yml/badge.svg)](https://github.com/Tinkora/curl_builder/actions/workflows/pages.yml)
[![Supply chain](https://github.com/Tinkora/curl_builder/actions/workflows/supply-chain.yml/badge.svg)](https://github.com/Tinkora/curl_builder/actions/workflows/supply-chain.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-176b55.svg)](LICENSE)

[Open the browser tool](https://tinkora.github.io/curl_builder/)

A browser-local HTTP request converter. Build or import one request, then copy
equivalent cURL, JavaScript Fetch, Python Requests, Go `net/http`, Rust Reqwest,
or Node.js `http`/`https` code without sending the request anywhere.

> Status: `0.1.0-alpha.1`. The supported request and cURL contracts are narrow
> by design. Generated code must still be reviewed before execution, especially
> when it contains credentials.

## Why it is useful

Translating a request between documentation, a terminal, and client libraries
is repetitive and easy to get subtly wrong. Curl Builder keeps one validated
request model and generates six reviewable representations from it. It works
without an account, project setup, backend, storage, telemetry, or data upload.

## Current capabilities

- Build one HTTP(S) request with method, URL, ordered headers, body, and body type.
- Generate cURL, Fetch, Python, Go, Rust, and Node.js snippets.
- Import a safe subset of cURL without invoking a shell or reading files.
- Validate URLs, HTTP headers, JSON, content types, duplicates, and input limits.
- Generate RFC 4648 Basic authentication from explicit `--user` input.
- Switch between complete English and Simplified Chinese browser interfaces.
- Run entirely in the browser through Rust and WebAssembly.

## Supported cURL subset

The importer accepts one `curl` command with one absolute HTTP(S) URL and these
options:

- `-X`, `--request`
- `-H`, `--header`
- `-d`, `--data`, `--data-raw`, `--data-binary`
- `-u`, `--user`
- `--compressed`

Unknown options, multiple URLs, shell operators, command substitution, globbing,
brace expansion, config files, and `@file` bodies are rejected. Import parses
text only; it never executes the command.

## Browser quick start

Prerequisites: Rust 1.95.0, `wasm32-unknown-unknown`, `wasm-pack` 0.15.0, and a
static file server such as Python 3.

```bash
git clone https://github.com/Tinkora/curl_builder.git
cd curl_builder
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release --out-dir ../../pkg crates/curl_builder_web -- --locked
python3 -m http.server 4173 --bind 127.0.0.1
```

Open `http://127.0.0.1:4173/`.

## Request contract

```json
{
  "method": "POST",
  "url": "https://api.example.com/v1/items",
  "headers": [["Content-Type", "application/json"]],
  "body": "{\"name\":\"Notebook\"}",
  "body_type": "json"
}
```

Methods are `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, and `OPTIONS`.
Body types are `none`, `json`, `form_urlencoded`, `raw`, and `xml`. GET and HEAD
bodies, duplicate header names, and multipart bodies are rejected.

## Deliberate limits

- Curl Builder generates code but does not send requests.
- It does not store history, collections, credentials, or browser data.
- It does not support file uploads, multipart bodies, OAuth flows, GraphQL,
  WebSocket, gRPC, or the full cURL option set.
- It does not publish an API, package, CLI, MCP server, or Agent transport.
- Syntax checks cannot prove that a remote service will accept generated code.

## Development

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p curl_builder_web --target wasm32-unknown-unknown --locked

wasm-pack build --target web --release --out-dir ../../pkg crates/curl_builder_web -- --locked
cd crates/curl_builder_web
npm ci --ignore-scripts
npm run test:wasm-smoke
```

The browser suite runs Chromium at 375, 768, 1024, and 1440 pixel widths against
the real WASM package. It checks generation, cURL import, rejected file reads,
locale state, keyboard tabs, reduced motion, external traffic, console output,
and horizontal overflow.

## Documentation

- [Product specification](docs/PRODUCT_SPEC.md)
- [Maturity and evidence](docs/MATURITY.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Support](SUPPORT.md)
- [Changelog](CHANGELOG.md)

## License

[MIT](LICENSE) Copyright Tinkora contributors.
