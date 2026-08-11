# 为 Curl Builder 贡献

[English](CONTRIBUTING.md)

感谢你帮助提高请求转换的可靠性。相比扩大功能范围，我们更倾向于小型、有证据支持的
改动。

## 提议功能之前

请先创建 feature request，说明用户工作流、发生频率、当前替代方案、已存在的成熟工具，
以及最小可用结果。新增 cURL 参数或输出目标需要真实示例和清晰的维护理由。

## 开发环境

需要 Rust 1.95.0、`wasm32-unknown-unknown`、`wasm-pack` 0.15.0、Node.js 24 和 npm。

```bash
git clone https://github.com/Tinkora/curl_builder.git
cd curl_builder
rustup target add wasm32-unknown-unknown
cargo test --workspace --locked
```

构建并测试浏览器版本：

```bash
wasm-pack build --target web --release --out-dir ../../pkg crates/curl_builder_web -- --locked
cd crates/curl_builder_web
npm ci --ignore-scripts
npx --no-install playwright install chromium
npm run test:wasm-smoke
```

## 改动要求

- Parser、校验或 generator 改动需要添加面向结果的回归测试。
- 不支持的语法必须明确失败，不能静默近似 shell 或 cURL 行为。
- 公开错误码保持稳定，错误消息不能泄露密钥。
- 英文和中文文档必须同步更新。
- 公开代码中的注释只使用英文。
- 修改或审查前端前必须运行 `ui-ux-pro-max`，随后验证四个受支持浏览器宽度。
- 使用英文 Conventional Commits，例如 `fix: reject duplicate header names`。

## Pull request

1. Fork 仓库并创建短期分支。
2. 每个 commit 和 pull request 只处理一个逻辑改动。
3. 在 PR 模板中说明行为、风险和验证证据。
4. 确保格式、Clippy、测试、WASM、浏览器、文档和安全检查通过。
5. 处理 review 意见时，不要用 force push 覆盖其他贡献者的工作。

维护者可能关闭没有用户证据的假想功能、扩大不安全解析面的改动，或无法验证的生成代码。

## 报告安全问题

疑似漏洞不能创建公开 issue，请遵循 [SECURITY.zh-CN.md](SECURITY.zh-CN.md)。

参与本项目即表示你同意遵守[行为准则](CODE_OF_CONDUCT.zh-CN.md)。
