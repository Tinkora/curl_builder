# Maturity and Evidence

[简体中文](MATURITY.zh-CN.md)

## Current level

Curl Builder is an Alpha. Its supported conversion contract is tested and useful,
but the cURL subset and compatibility evidence are intentionally narrower than a
general-purpose API client.

## Verified evidence

- Core tests cover validation, stable errors, limits, quoting, Unicode, Basic
  authentication, JSON, content types, duplicate headers, and cURL round trips.
- Generated JavaScript and Node.js are checked with `node --check`; Python with
  `ast.parse`; Go with `gofmt`; Rust with `rustfmt`.
- WASM compilation and browser behavior are checked against the real generated
  package.
- Chromium tests run at 375, 768, 1024, and 1440 pixel widths.
- CI pins third-party actions and reuses immutable organization workflow commits.
- Release archives receive checksums, SPDX SBOM and license evidence, and GitHub
  build provenance attestations.

## Known limits

- Compatibility is verified against representative snippets, not every runtime
  or remote service.
- The importer implements a documented subset, not shell or full cURL semantics.
- Credentials are not redacted from generated output.
- Browser-local processing does not make copied or executed snippets safe.

## Promotion criteria

A stable `1.0.0` requires sustained real-world use, no unresolved high-severity
security findings, published compatibility evidence across supported targets, and
a stable request and error contract. Features without user evidence should remain
deferred rather than expanding the Alpha surface.
