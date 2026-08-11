# Curl Builder Product Specification

[简体中文](PRODUCT_SPEC.zh-CN.md)

## Problem

Developers and AI-assisted workflows repeatedly translate the same HTTP request
between cURL and language-specific client code. Full API clients solve broader
problems but often require accounts, collections, workspaces, or a large desktop
application. Curl Builder provides a focused, browser-local conversion workspace
for one request at a time.

## Audience

- API developers converting examples between languages.
- Documentation authors producing consistent request snippets.
- People reviewing cURL commands before turning them into application code.
- Agent-assisted coding workflows that need deterministic, inspectable output.

## Alpha contract

```text
HttpRequest {
  method: GET | POST | PUT | PATCH | DELETE | HEAD | OPTIONS
  url: absolute HTTP(S) URL
  headers: ordered list of unique name/value pairs
  body: optional text
  body_type: none | json | form_urlencoded | raw | xml
}
```

Validation is shared by native Rust, WASM, and browser callers. Public failures
use stable codes. The parser treats cURL as untrusted text and never executes a
shell, reads a file, sends a request, or accesses browser storage.

## Input limits

| Input | Limit |
| --- | ---: |
| Serialized request or cURL command | 1 MiB |
| URL | 8 KiB |
| Headers | 100 |
| Header name | 256 bytes |
| Header value | 16 KiB |
| Body | 1 MiB |

Header names must use HTTP token characters. Header values cannot contain ASCII
control characters other than tab. JSON bodies must parse before generation.
GET and HEAD bodies, duplicate header names, NUL bytes, and multipart bodies are
rejected.

## Import contract

The cURL importer accepts `-X`/`--request`, `-H`/`--header`, supported data
options, `-u`/`--user`, `--compressed`, and one HTTP(S) URL. It supports bounded
POSIX-style quoting and line continuation needed for its own output.

Unknown options, missing values, multiple URLs, file-loading forms, config files,
proxies, certificates, cookies, redirects, shell operators, command substitution,
globbing, brace expansion, and unquoted shell metacharacters fail explicitly.

## Generation contract

One validated request generates six snippets. String values use target-language
safe literals. JSON is parsed before embedding. Content-Type behavior is kept
consistent across targets. Generated cURL uses `--data-raw` and must parse back
to the same supported request model.

Basic authentication follows cURL's explicit `user:password` form and uses RFC
4648 Base64. Credentials remain visible in local input and generated output; the
tool warns users to review snippets before sharing.

## Browser experience

The first screen is the actual two-pane tool. It provides method, URL, headers,
body, cURL import, six output tabs, copying, inline validation, and complete
English/Chinese switching. User input is rendered through text-only DOM sinks.

The page must remain usable at 375, 768, 1024, and 1440 pixel widths with no
page-level horizontal overflow, console problems, external runtime requests, or
keyboard-inaccessible controls. Reduced-motion preferences are respected.

## Non-goals

- Sending HTTP requests or acting as an API client.
- Accounts, collections, persistence, sync, history, or telemetry.
- Multipart files, OAuth flows, GraphQL, WebSocket, gRPC, or every cURL option.
- A hosted API, package, CLI, MCP server, or other Agent transport.

## Success criteria

- Users can convert a representative request in under one minute.
- Supported cURL output round-trips without changing the request contract.
- Representative generated JavaScript, Python, Go, and Rust passes syntax tools.
- Unsupported or unsafe input fails with a stable code and no execution side effect.
- Core, WASM, browser, documentation, security, and release gates pass in CI.
