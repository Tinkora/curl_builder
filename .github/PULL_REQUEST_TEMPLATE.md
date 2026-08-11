# Pull request

## Outcome

<!-- Describe the user-visible or maintenance result. -->

## Why

<!-- Link real workflow evidence or the issue this resolves. -->

## Risk

<!-- Cover parser, generated-code, compatibility, privacy, and release risk. -->

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo clippy --workspace --all-targets --locked -- -D warnings`
- [ ] `cargo check -p curl_builder_web --target wasm32-unknown-unknown --locked`
- [ ] Browser tests pass when frontend or WASM behavior changes
- [ ] English and Simplified Chinese docs are synchronized
- [ ] No secrets, private fixtures, or unsupported capability claims were added

Closes #
