# Curl Builder 产品规格

[English](PRODUCT_SPEC.md)

## 问题

开发者和 AI 辅助工作流经常需要在 cURL 与不同语言的客户端代码之间转换同一个 HTTP
请求。完整 API 客户端能解决更广泛的问题，但通常需要账号、集合、工作区或大型桌面
应用。Curl Builder 提供一个聚焦单个请求、在浏览器本地运行的转换工作区。

## 目标用户

- 需要在不同语言间转换示例的 API 开发者。
- 需要生成一致请求片段的技术文档作者。
- 在转换为应用代码前需要审查 cURL 命令的用户。
- 需要确定、可检查输出的 Agent 辅助编码工作流。

## Alpha 契约

```text
HttpRequest {
  method: GET | POST | PUT | PATCH | DELETE | HEAD | OPTIONS
  url: 绝对 HTTP(S) URL
  headers: 名称不重复的有序键值对列表
  body: 可选文本
  body_type: none | json | form_urlencoded | raw | xml
}
```

原生 Rust、WASM 和浏览器调用方共享同一套校验。公开失败使用稳定错误码。解析器把
cURL 当作不可信文本，不会执行 shell、读取文件、发送请求或访问浏览器存储。

## 输入限制

| 输入 | 限制 |
| --- | ---: |
| 序列化请求或 cURL 命令 | 1 MiB |
| URL | 8 KiB |
| 请求头数量 | 100 |
| 请求头名称 | 256 bytes |
| 请求头值 | 16 KiB |
| 正文 | 1 MiB |

请求头名称必须使用 HTTP token 字符。值不能包含 tab 以外的 ASCII 控制字符。JSON
正文必须在生成前成功解析。GET/HEAD 正文、重复请求头名称、NUL byte 和 multipart
正文会被拒绝。

## 导入契约

cURL 导入器接受 `-X`/`--request`、`-H`/`--header`、受支持的数据参数、
`-u`/`--user`、`--compressed` 和一个 HTTP(S) URL。它支持为解析自身输出所需的、
边界明确的 POSIX 风格引号和续行。

未知参数、缺失值、多个 URL、文件加载、配置文件、代理、证书、Cookie、重定向、
shell 操作符、命令替换、通配符、brace expansion 和未引用的 shell 元字符会明确失败。

## 生成契约

一个经过校验的请求生成六种代码。字符串使用目标语言安全的字面量；JSON 在嵌入前
解析；不同目标的 Content-Type 行为保持一致。生成的 cURL 使用 `--data-raw`，并且
必须能够解析回相同的请求模型。

Basic Authentication 遵循 cURL 的显式 `user:password` 形式，使用 RFC 4648 Base64。
凭据仍会出现在本地输入和生成代码中；工具会提醒用户分享前复核代码。

## 浏览器体验

首屏就是双栏实际工具，提供方法、URL、请求头、正文、cURL 导入、六个输出页签、复制、
行内校验以及完整中英文切换。不可信输入只通过纯文本 DOM 节点呈现。

页面必须在 375、768、1024 和 1440 像素宽度下可用，不出现页面级横向溢出、控制台
问题、外部运行时请求或键盘不可访问控件，并尊重 reduced-motion 偏好。

## 非目标

- 发送 HTTP 请求或充当 API 客户端。
- 账号、集合、持久化、同步、历史或遥测。
- Multipart 文件、OAuth 流程、GraphQL、WebSocket、gRPC 或完整 cURL 参数支持。
- 托管 API、Package、CLI、MCP server 或其他 Agent transport。

## 成功标准

- 用户能在一分钟内转换一个有代表性的请求。
- 支持范围内的 cURL 能 round-trip，且不改变请求契约。
- 代表性的 JavaScript、Python、Go 和 Rust 输出通过语法工具。
- 不支持或不安全的输入以稳定错误码失败，不产生执行副作用。
- Core、WASM、浏览器、文档、安全和发布门禁在 CI 中通过。
