# Contributing to Curl Builder

[简体中文](CONTRIBUTING.zh-CN.md)

Thank you for helping make request conversion more reliable. Small, evidence-led
changes are preferred over broad feature expansion.

## Before proposing a feature

Open a feature request that identifies the user workflow, frequency, current
workaround, established alternatives, and the smallest useful outcome. New cURL
options or output targets need real examples and a clear maintenance case.

## Development setup

Required versions are Rust 1.95.0, `wasm32-unknown-unknown`, `wasm-pack` 0.15.0,
Node.js 24, and npm.

```bash
git clone https://github.com/Tinkora/curl_builder.git
cd curl_builder
rustup target add wasm32-unknown-unknown
cargo test --workspace --locked
```

Build and test the browser:

```bash
wasm-pack build --target web --release --out-dir ../../pkg crates/curl_builder_web -- --locked
cd crates/curl_builder_web
npm ci --ignore-scripts
npx --no-install playwright install chromium
npm run test:wasm-smoke
```

## Change requirements

- Add outcome-focused regression tests for parser, validation, or generator changes.
- Keep unsupported syntax explicit; never silently approximate shell or cURL behavior.
- Keep public errors stable and free of secrets.
- Update English and Chinese documentation together.
- Use English comments only in public code.
- Run `ui-ux-pro-max` before changing or reviewing frontend code, then verify all
  four supported browser widths.
- Use English Conventional Commits such as `fix: reject duplicate header names`.

## Pull requests

1. Fork the repository and create a short-lived branch.
2. Keep one logical change per commit and pull request.
3. Complete the PR template with behavior, risk, and verification evidence.
4. Ensure formatting, Clippy, tests, WASM, browser, documentation, and security
   checks pass.
5. Address review feedback without force-pushing over another contributor's work.

Maintainers may close speculative features without user evidence, changes that
expand the unsafe parsing surface, or generated snippets that cannot be verified.

## Reporting security issues

Do not open a public issue for a suspected vulnerability. Follow
[SECURITY.md](SECURITY.md).

By participating, you agree to the [Code of Conduct](CODE_OF_CONDUCT.md).
