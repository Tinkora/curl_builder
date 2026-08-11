# Repository Guide for AI Agents

## Purpose

Curl Builder converts one validated HTTP request into cURL, JavaScript Fetch,
Python Requests, Go `net/http`, Rust Reqwest, and Node.js `http`/`https`
snippets. It also imports a deliberately limited, non-executing cURL subset.
All processing is browser-local through WebAssembly.

## Public Contract

- The request model and validation live in `crates/curl_builder_core`.
- The WASM boundary lives in `crates/curl_builder_web`.
- The browser source is `index.html` plus `assets/`.
- `multipart/form-data`, file reads, shell expansion, request execution,
  persistence, telemetry, and any Agent transport are outside the Alpha scope.
- Public failures must retain stable machine-readable error codes.

## Required Checks

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

## Repository Conventions

- README and user-facing documentation default to English and link to complete
  Simplified Chinese counterparts.
- Public code comments are English only.
- Commit subjects and bodies are English Conventional Commits. This repository
  rule overrides global commit-language preferences.
- Do not claim an MCP server, Agent transport, network isolation stronger than
  "no data upload or telemetry," or support that is not tested.
- Do not add legacy organization names, migration history, credentials, or
  internal planning documents to public files.

## Frontend Requirement

- Invoke `ui-ux-pro-max` before creating, modifying, reviewing, or debugging
  user-facing frontend code.
- Run its required design-system search and relevant stack or UX searches.
- Verify 375, 768, 1024, and 1440 pixel widths in a real browser, including
  keyboard navigation, console output, reduced motion, and horizontal overflow.
- If the skill is unavailable, stop frontend work and report the missing
  prerequisite.
