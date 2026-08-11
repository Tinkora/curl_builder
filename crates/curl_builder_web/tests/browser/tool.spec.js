import { expect, test } from "@playwright/test";

test("generates six snippets through the real WASM boundary", async ({ page }) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "Example" }).click();

  await expect(page.getByRole("status").filter({ hasText: "6 snippets ready" })).toBeVisible();
  await expect(page.getByRole("tabpanel", { name: "cURL" })).toContainText("--data-raw");
  await expect(page.getByRole("tabpanel", { name: "cURL" })).toContainText(
    "https://api.example.com/v1/items",
  );
});

test("imports a supported cURL command without exposing file reads", async ({ page }) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "Import cURL" }).click();
  await page.getByLabel("cURL command").fill(
    "curl --request POST --header 'Content-Type: application/json' " +
      "--data-raw '{\"name\":\"demo\"}' 'https://example.test/items'",
  );
  await page.getByRole("button", { name: "Import", exact: true }).click();

  await expect(page.getByLabel("Request URL")).toHaveValue("https://example.test/items");
  await expect(page.getByLabel("Body content")).toHaveValue('{"name":"demo"}');

  await page.getByRole("button", { name: "Import cURL" }).click();
  await page.getByLabel("cURL command").fill(
    "curl --data '@/etc/passwd' 'https://example.test/upload'",
  );
  await page.getByRole("button", { name: "Import", exact: true }).click();
  await expect(page.getByRole("alert")).toContainText("outside the Alpha subset");
});

test("keeps request state across locale changes and supports tab keyboard navigation", async ({
  page,
}) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "Example" }).click();
  await expect(page.getByRole("status").filter({ hasText: "6 snippets ready" })).toBeVisible();

  await page.getByRole("button", { name: "切换到中文" }).click();
  await expect(page.getByLabel("请求 URL")).toHaveValue("https://api.example.com/v1/items");
  await expect(page.getByRole("status").filter({ hasText: "6 份代码已生成" })).toBeVisible();

  const curlTab = page.getByRole("tab", { name: "cURL" });
  await curlTab.focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.getByRole("tab", { name: "JavaScript fetch" })).toBeFocused();
  await page.keyboard.press("End");
  await expect(page.getByRole("tab", { name: "Node.js http" })).toBeFocused();
  await page.keyboard.press("Home");
  await expect(curlTab).toBeFocused();
});

test("clears and disables request bodies when switching to GET or HEAD", async ({ page }) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "Example" }).click();
  await expect(page.getByLabel("Body content")).not.toBeDisabled();

  await page.getByRole("button", { name: "GET" }).click();

  await expect(page.getByLabel("Body content")).toBeDisabled();
  await expect(page.getByLabel("Body content")).toHaveValue("");
  await expect(page.getByRole("button", { name: "JSON" })).toBeDisabled();
  await expect(page.getByRole("status").filter({ hasText: "6 snippets ready" })).toBeVisible();
});

test("shows a credential warning before users copy generated code", async ({ page }) => {
  await page.goto("/static/");

  await expect(page.getByText("Generated code can include credentials. Review before copying.")).toBeVisible();
});

test("keeps keyboard focus inside the import dialog", async ({ page }) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "Import cURL" }).click();

  const close = page.getByRole("button", { name: "Close dialog" });
  const submit = page.getByRole("button", { name: "Import", exact: true });
  await submit.focus();
  await page.keyboard.press("Tab");
  await expect(close).toBeFocused();
  await page.keyboard.press("Shift+Tab");
  await expect(submit).toBeFocused();
});

test("explains body and Content-Type mismatches", async ({ page }) => {
  await page.goto("/static/");
  await page.getByRole("button", { name: "POST" }).click();
  await page.getByLabel("Request URL").fill("https://example.test/items");
  await page.getByRole("button", { name: "Add header" }).click();
  await page.getByLabel("Header 1 name").fill("Content-Type");
  await page.getByLabel("Header 1 value").fill("text/html");
  await page.getByRole("button", { name: "JSON" }).click();
  await page.getByLabel("Body content").fill("{}");

  await expect(page.getByRole("status").filter({
    hasText: "The body type and Content-Type do not match.",
  })).toBeVisible();
});

test("has no external traffic, console problems, or page-level overflow", async ({ page }) => {
  const problems = [];
  const requests = [];
  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      problems.push(`${message.type()}: ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
  page.on("request", (request) => requests.push(request.url()));

  await page.goto("/static/");
  await page.getByRole("button", { name: "Example" }).click();
  await page.waitForLoadState("networkidle");

  const pageOrigin = new URL(page.url()).origin;
  expect(requests.every((url) => url.startsWith(pageOrigin))).toBe(true);
  expect(problems).toEqual([]);
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= document.documentElement.clientWidth,
    ),
  ).toBe(true);
});

test("honors reduced-motion preferences", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/static/");

  const transitionSeconds = await page.getByRole("button", { name: "Example" }).evaluate(
    (button) => Number.parseFloat(getComputedStyle(button).transitionDuration),
  );
  expect(transitionSeconds).toBeLessThan(0.001);
});
