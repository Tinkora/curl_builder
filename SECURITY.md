# Security Policy

[简体中文](SECURITY.zh-CN.md)

## Supported versions

Security fixes are provided for the latest published pre-release only.

| Version | Supported |
| --- | --- |
| `0.1.0-alpha.1` | Yes |
| Older or unreleased snapshots | No |

## Report a vulnerability

Use GitHub's **Report a vulnerability** flow in the repository Security tab.
Do not disclose suspected vulnerabilities in public issues, discussions, pull
requests, or generated snippets.

Include a minimal reproduction, affected version or commit, impact, and any
suggested mitigation. Remove real credentials and personal or proprietary data.
You can expect acknowledgement within 72 hours and a status update within seven
days. Timelines for fixes and coordinated disclosure depend on severity.

## Security boundaries

In scope:

- Shell interpretation or file access caused by cURL parsing or generation.
- Injection or unsafe escaping in generated snippets.
- URL, header, body, size-limit, or stable-error validation bypasses.
- Exposure of input through network requests, storage, telemetry, logs, or DOM sinks.
- Workflow privilege, dependency, release integrity, or artifact provenance flaws.

The browser tool parses and generates text but never executes snippets or sends
the represented HTTP request. Generated code can contain credentials by design.
Review and redact it before sharing or execution.

Out of scope:

- Remote service behavior after a user executes generated code.
- Social engineering, physical access, or unsupported browsers.
- Availability attacks requiring inputs above documented limits.
- Dependency issues without a project-specific exploit or impact.

## Release integrity

Release assets include SHA-256 checksums, an SPDX SBOM, license evidence, and
GitHub attestations. Verify these before using downloaded archives.
