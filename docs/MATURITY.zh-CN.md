# 成熟度与证据

[English](MATURITY.md)

## 当前级别

Curl Builder 当前是 Alpha。受支持的转换契约已经测试并具有实际价值，但 cURL 子集和
兼容性证据有意保持得比通用 API 客户端更窄。

## 已验证证据

- Core 测试覆盖校验、稳定错误码、限制、引号、Unicode、Basic Authentication、JSON、
  Content-Type、重复请求头和 cURL round-trip。
- JavaScript 和 Node.js 使用 `node --check`，Python 使用 `ast.parse`，Go 使用
  `gofmt`，Rust 使用 `rustfmt` 检查代表性生成代码。
- WASM 编译和浏览器行为针对真实生成 package 验证。
- Chromium 测试覆盖 375、768、1024 和 1440 像素宽度。
- CI 固定第三方 Action，并通过不可变 commit 复用组织工作流。
- Release archive 生成 checksum、SPDX SBOM、许可证证据和 GitHub build provenance
  attestation。

## 已知限制

- 兼容性针对代表性代码验证，不覆盖所有运行时或远程服务。
- 导入器实现文档化子集，不实现 shell 或完整 cURL 语义。
- 生成代码中的凭据不会被脱敏。
- 浏览器本地处理不能保证被复制或执行的代码一定安全。

## 晋级标准

稳定版 `1.0.0` 需要持续真实使用、没有未解决的高严重性安全问题、覆盖各受支持目标的
兼容性证据，以及稳定的请求和错误契约。缺少用户证据的功能应继续延后，而不是扩大
Alpha 范围。
