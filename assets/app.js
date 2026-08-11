import init, {
  generate_all,
  list_languages,
  parse_curl_command,
  validate_url,
} from "../pkg/curl_builder_web.js";

const METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
const BODY_TYPES = ["none", "json", "form_urlencoded", "raw", "xml"];
const LANGUAGE_LABELS = {
  curl: "cURL",
  fetch: "JavaScript fetch",
  python: "Python requests",
  go: "Go net/http",
  rust: "Rust reqwest",
  node: "Node.js http",
};

const translations = {
  en: {
    actions: "Workspace actions",
    privacy: "Local only · no requests sent",
    clear: "Clear",
    example: "Example",
    importCurl: "Import cURL",
    copyCurrent: "Copy current snippet",
    copy: "Copy",
    request: "Request",
    composeRequest: "Compose HTTP request",
    method: "Method",
    requestUrl: "Request URL",
    urlPlaceholder: "https://api.example.com/v1/items",
    headers: "Headers",
    headerName: "Name",
    headerValue: "Value",
    addHeader: "Add header",
    body: "Body",
    bodyContent: "Body content",
    bodyPlaceholder: "Enter request body",
    output: "Output",
    generatedCode: "Generated code",
    copyWarning: "Generated code can include credentials. Review before copying.",
    waitingUrl: "Waiting for a URL",
    outputLanguage: "Output language",
    emptyOutput: "Add a valid HTTP(S) URL to generate snippets.",
    boundary: "One request · HTTP(S) · browser-local",
    docs: "Docs",
    importTitle: "Import one cURL request",
    closeDialog: "Close dialog",
    curlCommand: "cURL command",
    importBoundary: "Supported options are parsed locally. Files and shell expansion are rejected.",
    cancel: "Cancel",
    import: "Import",
    removeHeader: "Remove header {number}",
    headerNameNumber: "Header {number} name",
    headerValueNumber: "Header {number} value",
    copyLanguage: "Copy {language} snippet",
    bodyNone: "None",
    bodyJson: "JSON",
    bodyForm: "Form URL-encoded",
    bodyRaw: "Raw",
    bodyXml: "XML",
    hintNone: "No request body will be generated.",
    hintJson: "Valid JSON is required.",
    hintForm: "Use key=value&other=value format.",
    hintRaw: "Plain text is preserved exactly.",
    hintXml: "XML is emitted as text.",
    hintMethodNoBody: "GET and HEAD do not support a request body in this tool.",
    urlValid: "Valid HTTP(S) URL",
    urlInvalid: "Enter an absolute HTTP(S) URL.",
    generating: "Generating…",
    ready: "6 snippets ready",
    copied: "Copied. Review the snippet for secrets before sharing.",
    copyFailed: "Clipboard access failed.",
    nothingToCopy: "There is no generated snippet to copy.",
    cleared: "Request cleared.",
    exampleLoaded: "Example request loaded.",
    imported: "cURL request imported.",
    importEmpty: "Paste a cURL command first.",
    wasmError: "The WebAssembly module could not be loaded.",
    errorDefault: "The request could not be generated.",
    errorInvalidUrl: "Only absolute HTTP(S) URLs are supported.",
    errorInvalidHeader: "Check header names and remove line breaks from values.",
    errorInvalidJson: "The request body is not valid JSON.",
    errorInvalidBody: "The request body is not valid for this method or body type.",
    errorUnsupportedBody: "The body type and Content-Type do not match.",
    errorLimit: "The request exceeds an Alpha input limit.",
    errorCurlOption: "This cURL option is outside the Alpha subset.",
    errorShell: "Shell expansion and command operators are not supported.",
    errorParse: "The cURL command could not be parsed.",
  },
  zh: {
    actions: "工作区操作",
    privacy: "仅本地处理 · 不发送请求",
    clear: "清空",
    example: "示例",
    importCurl: "导入 cURL",
    copyCurrent: "复制当前代码",
    copy: "复制",
    request: "请求",
    composeRequest: "构建 HTTP 请求",
    method: "方法",
    requestUrl: "请求 URL",
    urlPlaceholder: "https://api.example.com/v1/items",
    headers: "请求头",
    headerName: "名称",
    headerValue: "值",
    addHeader: "添加请求头",
    body: "请求体",
    bodyContent: "请求体内容",
    bodyPlaceholder: "输入请求体内容",
    output: "输出",
    generatedCode: "生成的代码",
    copyWarning: "生成代码可能包含凭据，复制前请先检查。",
    waitingUrl: "等待输入 URL",
    outputLanguage: "输出语言",
    emptyOutput: "输入有效的 HTTP(S) URL 后生成代码。",
    boundary: "单个请求 · HTTP(S) · 浏览器本地处理",
    docs: "文档",
    importTitle: "导入单个 cURL 请求",
    closeDialog: "关闭对话框",
    curlCommand: "cURL 命令",
    importBoundary: "仅在本地解析支持的参数；文件读取和 shell expansion 会被拒绝。",
    cancel: "取消",
    import: "导入",
    removeHeader: "删除第 {number} 个请求头",
    headerNameNumber: "第 {number} 个请求头名称",
    headerValueNumber: "第 {number} 个请求头值",
    copyLanguage: "复制 {language} 代码",
    bodyNone: "无",
    bodyJson: "JSON",
    bodyForm: "URL 编码表单",
    bodyRaw: "原始文本",
    bodyXml: "XML",
    hintNone: "不会生成请求体。",
    hintJson: "必须输入有效的 JSON。",
    hintForm: "使用 key=value&other=value 格式。",
    hintRaw: "原始文本将保持不变。",
    hintXml: "XML 将作为文本输出。",
    hintMethodNoBody: "本工具不支持为 GET 和 HEAD 添加请求体。",
    urlValid: "有效的 HTTP(S) URL",
    urlInvalid: "请输入绝对 HTTP(S) URL。",
    generating: "正在生成…",
    ready: "6 份代码已生成",
    copied: "已复制；分享前请检查代码中是否包含密钥。",
    copyFailed: "无法访问剪贴板。",
    nothingToCopy: "当前没有可复制的代码。",
    cleared: "请求已清空。",
    exampleLoaded: "示例请求已加载。",
    imported: "cURL 请求已导入。",
    importEmpty: "请先粘贴 cURL 命令。",
    wasmError: "无法加载 WebAssembly 模块。",
    errorDefault: "无法生成这个请求。",
    errorInvalidUrl: "仅支持绝对 HTTP(S) URL。",
    errorInvalidHeader: "请检查请求头名称，并删除值中的换行符。",
    errorInvalidJson: "请求体不是有效的 JSON。",
    errorInvalidBody: "请求体不适用于当前方法或正文类型。",
    errorUnsupportedBody: "正文类型与 Content-Type 不匹配。",
    errorLimit: "请求超出了 Alpha 的输入限制。",
    errorCurlOption: "Alpha 暂不支持这个 cURL 参数。",
    errorShell: "不支持 shell expansion 和命令操作符。",
    errorParse: "无法解析这个 cURL 命令。",
  },
};

const state = {
  locale: "en",
  method: "GET",
  url: "",
  headers: [],
  body: "",
  bodyType: "none",
  activeLanguage: "curl",
  languages: [],
  snippets: {},
  outputStatusKey: "waitingUrl",
  outputStatusError: false,
  debounceTimer: 0,
  toastTimer: 0,
  dialogReturnFocus: null,
};

const dom = {
  requestForm: document.querySelector("#request-form"),
  methodControl: document.querySelector("#method-control"),
  urlInput: document.querySelector("#url-input"),
  urlStatus: document.querySelector("#url-status"),
  headerList: document.querySelector("#header-list"),
  headerCounter: document.querySelector("#header-counter"),
  addHeaderButton: document.querySelector("#add-header-button"),
  bodyControl: document.querySelector("#body-control"),
  bodyInput: document.querySelector("#body-input"),
  bodyHint: document.querySelector("#body-hint"),
  languageTabs: document.querySelector("#language-tabs"),
  codeViewport: document.querySelector("#code-viewport"),
  emptyState: document.querySelector("#empty-state"),
  outputStatus: document.querySelector("#output-status"),
  clearButton: document.querySelector("#clear-button"),
  exampleButton: document.querySelector("#example-button"),
  importButton: document.querySelector("#import-button"),
  copyButton: document.querySelector("#copy-button"),
  localeButton: document.querySelector("#locale-button"),
  importDialog: document.querySelector("#import-dialog"),
  importInput: document.querySelector("#import-input"),
  importError: document.querySelector("#import-error"),
  dialogCloseButton: document.querySelector("#dialog-close-button"),
  dialogCancelButton: document.querySelector("#dialog-cancel-button"),
  dialogImportButton: document.querySelector("#dialog-import-button"),
  toast: document.querySelector("#toast"),
};

function text(key, variables = {}) {
  const template = translations[state.locale][key] ?? translations.en[key] ?? key;
  return Object.entries(variables).reduce(
    (value, [name, replacement]) => value.replace(`{${name}}`, String(replacement)),
    template,
  );
}

function applyLocale() {
  document.documentElement.lang = state.locale === "zh" ? "zh-CN" : "en";
  document.title = state.locale === "zh" ? "Curl Builder | Tinkora" : "Curl Builder | Tinkora";

  document.querySelectorAll("[data-i18n]").forEach((element) => {
    element.textContent = text(element.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-placeholder]").forEach((element) => {
    element.placeholder = text(element.dataset.i18nPlaceholder);
  });
  document.querySelectorAll("[data-i18n-aria]").forEach((element) => {
    element.setAttribute("aria-label", text(element.dataset.i18nAria));
  });
  dom.localeButton.textContent = state.locale === "en" ? "中文" : "English";
  dom.localeButton.setAttribute(
    "aria-label",
    state.locale === "en" ? "切换到中文" : "Switch to English",
  );
  renderMethods();
  renderBodyTypes();
  renderHeaders();
  renderLanguageTabs();
  renderOutputs();
  updateBodyState();
  updateUrlStatus();
  setOutputStatus(state.outputStatusKey, state.outputStatusError);
  dom.dialogCloseButton.title = text("closeDialog");
}

function createIcon(symbol) {
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.classList.add("icon");
  svg.setAttribute("aria-hidden", "true");
  const use = document.createElementNS("http://www.w3.org/2000/svg", "use");
  use.setAttribute("href", `#icon-${symbol}`);
  svg.append(use);
  return svg;
}

function renderMethods() {
  const buttons = METHODS.map((method) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "segment-button";
    button.dataset.method = method;
    button.setAttribute("aria-pressed", String(state.method === method));
    button.textContent = method;
    button.addEventListener("click", () => {
      state.method = method;
      if (!methodAllowsBody(method)) {
        state.body = "";
        state.bodyType = "none";
        dom.bodyInput.value = "";
      }
      renderMethods();
      renderBodyTypes();
      updateBodyState();
      scheduleGeneration();
    });
    return button;
  });
  dom.methodControl.replaceChildren(...buttons);
}

function methodAllowsBody(method = state.method) {
  return !["GET", "HEAD"].includes(method);
}

function bodyTypeLabel(bodyType) {
  return {
    none: text("bodyNone"),
    json: text("bodyJson"),
    form_urlencoded: text("bodyForm"),
    raw: text("bodyRaw"),
    xml: text("bodyXml"),
  }[bodyType];
}

function renderBodyTypes() {
  const buttons = BODY_TYPES.map((bodyType) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "segment-button";
    button.setAttribute("aria-pressed", String(state.bodyType === bodyType));
    button.disabled = bodyType !== "none" && !methodAllowsBody();
    button.textContent = bodyTypeLabel(bodyType);
    button.addEventListener("click", () => {
      state.bodyType = bodyType;
      addSuggestedContentType(bodyType);
      renderBodyTypes();
      renderHeaders();
      updateBodyState();
      scheduleGeneration();
    });
    return button;
  });
  dom.bodyControl.replaceChildren(...buttons);
}

function addSuggestedContentType(bodyType) {
  const contentTypes = {
    json: "application/json",
    form_urlencoded: "application/x-www-form-urlencoded",
    raw: "text/plain",
    xml: "application/xml",
  };
  const expected = contentTypes[bodyType];
  if (!expected) return;

  const existing = state.headers.find(([name]) => name.toLowerCase() === "content-type");
  if (!existing) {
    state.headers.push(["Content-Type", expected]);
    return;
  }

  const managedValues = new Set(Object.values(contentTypes));
  if (managedValues.has(existing[1])) existing[1] = expected;
}

function updateBodyState() {
  const hintKeys = {
    none: "hintNone",
    json: "hintJson",
    form_urlencoded: "hintForm",
    raw: "hintRaw",
    xml: "hintXml",
  };
  const bodyAllowed = methodAllowsBody();
  dom.bodyInput.disabled = state.bodyType === "none" || !bodyAllowed;
  dom.bodyHint.textContent = text(bodyAllowed ? hintKeys[state.bodyType] : "hintMethodNoBody");
}

function renderHeaders() {
  const rows = state.headers.map((header, index) => {
    const row = document.createElement("div");
    row.className = "header-row";

    const nameInput = document.createElement("input");
    nameInput.className = "header-input mono";
    nameInput.type = "text";
    nameInput.value = header[0];
    nameInput.placeholder = text("headerName");
    nameInput.setAttribute("aria-label", text("headerNameNumber", { number: index + 1 }));
    nameInput.addEventListener("input", () => {
      state.headers[index][0] = nameInput.value;
      scheduleGeneration();
    });

    const valueInput = document.createElement("input");
    valueInput.className = "header-input mono";
    valueInput.type = "text";
    valueInput.value = header[1];
    valueInput.placeholder = text("headerValue");
    valueInput.setAttribute("aria-label", text("headerValueNumber", { number: index + 1 }));
    valueInput.addEventListener("input", () => {
      state.headers[index][1] = valueInput.value;
      scheduleGeneration();
    });

    const removeButton = document.createElement("button");
    removeButton.type = "button";
    removeButton.className = "remove-header-button";
    removeButton.setAttribute("aria-label", text("removeHeader", { number: index + 1 }));
    removeButton.title = text("removeHeader", { number: index + 1 });
    removeButton.append(createIcon("x"));
    removeButton.addEventListener("click", () => {
      state.headers.splice(index, 1);
      renderHeaders();
      scheduleGeneration();
    });

    row.append(nameInput, valueInput, removeButton);
    return row;
  });

  dom.headerList.replaceChildren(...rows);
  dom.headerCounter.textContent = `${state.headers.length} / 100`;
  dom.addHeaderButton.disabled = state.headers.length >= 100;
}

function requestPayload() {
  return {
    method: state.method,
    url: state.url.trim(),
    headers: state.headers
      .filter(([name]) => name.trim() !== "")
      .map(([name, value]) => [name.trim(), value.trim()]),
    body: state.bodyType === "none" || state.body === "" ? null : state.body,
    body_type: state.bodyType,
  };
}

function localizedErrorKey(error) {
  const keys = {
    EMPTY_URL: "errorInvalidUrl",
    INVALID_URL: "errorInvalidUrl",
    INVALID_HEADER: "errorInvalidHeader",
    INVALID_JSON: "errorInvalidJson",
    INVALID_BODY: "errorInvalidBody",
    LIMIT_EXCEEDED: "errorLimit",
    UNSUPPORTED_BODY_TYPE: "errorUnsupportedBody",
    UNSUPPORTED_CURL_OPTION: "errorCurlOption",
    UNSUPPORTED_SHELL_SYNTAX: "errorShell",
    PARSE_ERROR: "errorParse",
  };
  return keys[error?.code] ?? "errorDefault";
}

function localizedError(error) {
  return text(localizedErrorKey(error));
}

async function updateUrlStatus() {
  const value = state.url.trim();
  if (!value) {
    dom.urlInput.removeAttribute("aria-invalid");
    dom.urlStatus.className = "field-status";
    dom.urlStatus.textContent = "";
    return false;
  }

  try {
    await validate_url(value);
    dom.urlInput.setAttribute("aria-invalid", "false");
    dom.urlStatus.className = "field-status valid";
    dom.urlStatus.textContent = text("urlValid");
    return true;
  } catch {
    dom.urlInput.setAttribute("aria-invalid", "true");
    dom.urlStatus.className = "field-status error";
    dom.urlStatus.textContent = text("urlInvalid");
    return false;
  }
}

function setOutputStatus(key, isError = false) {
  state.outputStatusKey = key;
  state.outputStatusError = isError;
  dom.outputStatus.textContent = text(key);
  dom.outputStatus.classList.toggle("error", isError);
}

function scheduleGeneration() {
  window.clearTimeout(state.debounceTimer);
  state.debounceTimer = window.setTimeout(generateSnippets, 180);
}

async function generateSnippets() {
  state.url = dom.urlInput.value;
  state.body = dom.bodyInput.value;

  if (!state.url.trim()) {
    state.snippets = {};
    setOutputStatus("waitingUrl");
    renderOutputs();
    await updateUrlStatus();
    return;
  }

  setOutputStatus("generating");
  try {
    const snippets = await generate_all(requestPayload());
    state.snippets = Object.fromEntries(
      state.languages.map((language) => [language, snippets[language]]),
    );
    setOutputStatus("ready");
    renderOutputs();
    await updateUrlStatus();
  } catch (error) {
    state.snippets = {};
    setOutputStatus(localizedErrorKey(error), true);
    renderOutputs();
    await updateUrlStatus();
  }
}

function selectLanguage(language) {
  state.activeLanguage = language;
  renderLanguageTabs();
  document.querySelectorAll(".code-panel").forEach((panel) => {
    panel.hidden = panel.dataset.language !== language;
  });
}

function renderLanguageTabs() {
  const tabs = state.languages.map((language, index) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "tab-button";
    button.id = `tab-${language}`;
    button.dataset.language = language;
    button.setAttribute("role", "tab");
    button.setAttribute("aria-controls", `panel-${language}`);
    button.setAttribute("aria-selected", String(language === state.activeLanguage));
    button.tabIndex = language === state.activeLanguage ? 0 : -1;
    button.textContent = LANGUAGE_LABELS[language] ?? language;
    button.addEventListener("click", () => {
      selectLanguage(language);
      document.querySelector(`#tab-${language}`)?.focus();
    });
    button.addEventListener("keydown", (event) => handleTabKeydown(event, index));
    return button;
  });
  dom.languageTabs.replaceChildren(...tabs);
}

function handleTabKeydown(event, index) {
  let nextIndex = index;
  if (event.key === "ArrowRight") nextIndex = (index + 1) % state.languages.length;
  else if (event.key === "ArrowLeft") {
    nextIndex = (index - 1 + state.languages.length) % state.languages.length;
  } else if (event.key === "Home") nextIndex = 0;
  else if (event.key === "End") nextIndex = state.languages.length - 1;
  else return;

  event.preventDefault();
  const nextLanguage = state.languages[nextIndex];
  selectLanguage(nextLanguage);
  document.querySelector(`#tab-${nextLanguage}`)?.focus();
}

function renderOutputs() {
  const entries = state.languages.filter((language) => state.snippets[language]);
  if (entries.length === 0) {
    dom.emptyState.hidden = false;
    dom.codeViewport.replaceChildren(dom.emptyState);
    return;
  }

  dom.emptyState.hidden = true;
  const panels = entries.map((language) => {
    const panel = document.createElement("div");
    panel.className = "code-panel";
    panel.id = `panel-${language}`;
    panel.dataset.language = language;
    panel.setAttribute("role", "tabpanel");
    panel.setAttribute("aria-labelledby", `tab-${language}`);
    panel.hidden = language !== state.activeLanguage;

    const copyButton = document.createElement("button");
    copyButton.type = "button";
    copyButton.className = "panel-copy-button";
    const label = text("copyLanguage", { language: LANGUAGE_LABELS[language] ?? language });
    copyButton.setAttribute("aria-label", label);
    copyButton.title = label;
    copyButton.append(createIcon("copy"));
    copyButton.addEventListener("click", () => copyText(state.snippets[language]));

    const pre = document.createElement("pre");
    const code = document.createElement("code");
    code.textContent = state.snippets[language];
    pre.append(code);
    panel.append(copyButton, pre);
    return panel;
  });
  dom.codeViewport.replaceChildren(...panels);
}

async function copyText(value) {
  if (!value) {
    showToast(text("nothingToCopy"), true);
    return;
  }

  try {
    await navigator.clipboard.writeText(value);
    showToast(text("copied"));
  } catch {
    showToast(text("copyFailed"), true);
  }
}

function showToast(message, isError = false) {
  window.clearTimeout(state.toastTimer);
  dom.toast.textContent = message;
  dom.toast.className = `toast visible${isError ? " error" : ""}`;
  state.toastTimer = window.setTimeout(() => {
    dom.toast.className = "toast";
  }, 2600);
}

function clearRequest() {
  state.method = "GET";
  state.url = "";
  state.headers = [];
  state.body = "";
  state.bodyType = "none";
  state.snippets = {};
  dom.urlInput.value = "";
  dom.bodyInput.value = "";
  renderMethods();
  renderBodyTypes();
  renderHeaders();
  updateBodyState();
  renderOutputs();
  updateUrlStatus();
  setOutputStatus("waitingUrl");
  showToast(text("cleared"));
  dom.urlInput.focus();
}

function loadExample() {
  state.method = "POST";
  state.url = "https://api.example.com/v1/items";
  state.headers = [
    ["Content-Type", "application/json"],
    ["Accept", "application/json"],
  ];
  state.body = '{\n  "name": "Notebook",\n  "active": true\n}';
  state.bodyType = "json";
  dom.urlInput.value = state.url;
  dom.bodyInput.value = state.body;
  renderMethods();
  renderBodyTypes();
  renderHeaders();
  updateBodyState();
  generateSnippets();
  showToast(text("exampleLoaded"));
}

function openImportDialog() {
  state.dialogReturnFocus = document.activeElement;
  dom.importError.textContent = "";
  dom.importDialog.showModal();
  dom.importInput.focus();
}

function closeImportDialog() {
  dom.importDialog.close();
  dom.importInput.value = "";
  dom.importError.textContent = "";
  state.dialogReturnFocus?.focus();
}

function trapImportDialogFocus(event) {
  if (event.key !== "Tab" || !dom.importDialog.open) return;

  const focusable = Array.from(
    dom.importDialog.querySelectorAll("button:not(:disabled), textarea:not(:disabled)"),
  );
  const first = focusable[0];
  const last = focusable.at(-1);
  if (event.shiftKey && document.activeElement === first) {
    event.preventDefault();
    last?.focus();
  } else if (!event.shiftKey && document.activeElement === last) {
    event.preventDefault();
    first?.focus();
  }
}

async function importCurl() {
  const command = dom.importInput.value.trim();
  if (!command) {
    dom.importError.textContent = text("importEmpty");
    return;
  }

  try {
    const request = await parse_curl_command(command);
    state.method = request.method;
    state.url = request.url;
    state.headers = request.headers.map(([name, value]) => [name, value]);
    state.body = request.body ?? "";
    state.bodyType = request.body_type;
    dom.urlInput.value = state.url;
    dom.bodyInput.value = state.body;
    renderMethods();
    renderBodyTypes();
    renderHeaders();
    updateBodyState();
    closeImportDialog();
    await generateSnippets();
    showToast(text("imported"));
  } catch (error) {
    dom.importError.textContent = localizedError(error);
  }
}

function bindEvents() {
  dom.requestForm.addEventListener("submit", (event) => event.preventDefault());
  dom.urlInput.addEventListener("input", () => {
    state.url = dom.urlInput.value;
    updateUrlStatus();
    scheduleGeneration();
  });
  dom.bodyInput.addEventListener("input", () => {
    state.body = dom.bodyInput.value;
    scheduleGeneration();
  });
  dom.addHeaderButton.addEventListener("click", () => {
    if (state.headers.length >= 100) return;
    state.headers.push(["", ""]);
    renderHeaders();
    dom.headerList.querySelector(".header-row:last-child .header-input")?.focus();
  });
  dom.clearButton.addEventListener("click", clearRequest);
  dom.exampleButton.addEventListener("click", loadExample);
  dom.importButton.addEventListener("click", openImportDialog);
  dom.copyButton.addEventListener("click", () => copyText(state.snippets[state.activeLanguage]));
  dom.localeButton.addEventListener("click", () => {
    state.locale = state.locale === "en" ? "zh" : "en";
    applyLocale();
  });
  dom.dialogCloseButton.addEventListener("click", closeImportDialog);
  dom.dialogCancelButton.addEventListener("click", closeImportDialog);
  dom.dialogImportButton.addEventListener("click", importCurl);
  dom.importDialog.addEventListener("keydown", trapImportDialogFocus);
  dom.importDialog.addEventListener("close", () => state.dialogReturnFocus?.focus());
}

async function initialize() {
  bindEvents();
  renderMethods();
  renderBodyTypes();
  renderHeaders();
  updateBodyState();
  await init();
  state.languages = Array.from(list_languages());
  state.activeLanguage = state.languages[0] ?? "curl";
  applyLocale();
}

initialize().catch((error) => {
  console.error("Failed to initialize Curl Builder", error);
  setOutputStatus("wasmError", true);
});
