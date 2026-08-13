# Curl Builder

[English](README.md)

<!-- markdownlint-disable MD033 -->
<p align="center">
  <a href="https://ko-fi.com/tinkora" target="_blank" rel="noopener noreferrer">
    <img
      src="https://ko-fi.com/img/githubbutton_sm.svg"
      alt="在 Ko-fi 上支持 Tinkora"
      width="520"
    >
  </a>
</p>
<!-- markdownlint-enable MD033 -->

[打开浏览器工具](https://tinkora.github.io/curl_builder/)

Curl Builder 是一个在浏览器本地运行的 HTTP 请求转换器。你可以构建或导入单个请求，
再复制等价的 cURL、JavaScript Fetch、Python Requests、Go `net/http`、Rust Reqwest
或 Node.js `http`/`https` 代码；工具本身不会发送这个请求。

> 状态：`0.1.0-alpha.1`。支持的请求和 cURL 契约有意保持窄小。执行生成代码前仍需
> 人工复核，尤其是代码中包含凭据时。

## 为什么有用

在 API 文档、终端命令和不同客户端库之间转换同一个请求，是高频、重复且容易出现
细微错误的工作。Curl Builder 使用一个经过校验的请求模型生成六种可审查的表示。
它不要求账号和项目配置，也没有后端、存储、遥测或数据上传。

## 当前能力

- 使用方法、URL、有序请求头、正文和正文类型构建一个 HTTP(S) 请求。
- 生成 cURL、Fetch、Python、Go、Rust 和 Node.js 代码。
- 在不启动 shell、不读取文件的前提下导入安全的 cURL 子集。
- 校验 URL、HTTP 请求头、JSON、Content-Type、重复项和输入大小限制。
- 从显式 `--user` 输入生成符合 RFC 4648 的 Basic Authentication。
- 在完整英文和简体中文浏览器界面之间切换。
- 通过 Rust 和 WebAssembly 完全在浏览器中运行。

## 支持的 cURL 子集

导入器接受一条 `curl` 命令、一个绝对 HTTP(S) URL，以及以下参数：

- `-X`、`--request`
- `-H`、`--header`
- `-d`、`--data`、`--data-raw`、`--data-binary`
- `-u`、`--user`
- `--compressed`

未知参数、多个 URL、shell 操作符、命令替换、通配符、brace expansion、配置文件和
`@file` 正文会被拒绝。导入过程只解析文本，绝不会执行命令。

## 浏览器快速开始

前置条件：Rust 1.95.0、`wasm32-unknown-unknown`、`wasm-pack` 0.15.0，以及
Python 3 等静态文件服务器。

```bash
git clone https://github.com/Tinkora/curl_builder.git
cd curl_builder
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release --out-dir ../../pkg crates/curl_builder_web -- --locked
python3 -m http.server 4173 --bind 127.0.0.1
```

打开 `http://127.0.0.1:4173/`。

## 请求契约

```json
{
  "method": "POST",
  "url": "https://api.example.com/v1/items",
  "headers": [["Content-Type", "application/json"]],
  "body": "{\"name\":\"Notebook\"}",
  "body_type": "json"
}
```

方法支持 `GET`、`POST`、`PUT`、`PATCH`、`DELETE`、`HEAD` 和 `OPTIONS`。
正文类型支持 `none`、`json`、`form_urlencoded`、`raw` 和 `xml`。GET/HEAD 正文、
重复请求头名称和 multipart 正文会被拒绝。

## 明确限制

- Curl Builder 只生成代码，不发送请求。
- 不保存历史、集合、凭据或浏览器数据。
- 不支持文件上传、multipart 正文、OAuth 流程、GraphQL、WebSocket、gRPC 或完整的
  cURL 参数集合。
- 当前没有发布 API、Package、CLI、MCP server 或 Agent transport。
- 语法检查不能证明远程服务一定接受生成的代码。

## 开发

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

浏览器测试会在 375、768、1024 和 1440 像素宽度下，对真实 WASM package 运行
Chromium，并检查代码生成、cURL 导入、文件读取拒绝、语言状态、键盘页签、reduced
motion、外部请求、控制台和横向溢出。

## 文档

- [产品规格](docs/PRODUCT_SPEC.zh-CN.md)
- [成熟度与证据](docs/MATURITY.zh-CN.md)
- [贡献指南](CONTRIBUTING.zh-CN.md)
- [安全策略](SECURITY.zh-CN.md)
- [支持](SUPPORT.zh-CN.md)
- [变更记录](CHANGELOG.md)

## 许可证

[MIT](LICENSE)，版权归 Tinkora 贡献者所有。
