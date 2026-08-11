import { cp, lstat, mkdir, readdir, realpath, rm } from "node:fs/promises";
import { join, relative, resolve } from "node:path";

const repositoryRoot = resolve("../..");
const destination = resolve("static");
const sourcePackage = resolve(process.env.WASM_SMOKE_PACKAGE ?? "../../pkg");
const requiredFiles = ["package.json", "curl_builder_web.js", "curl_builder_web_bg.wasm"];

async function metadata(path) {
  try {
    return await lstat(path);
  } catch (error) {
    if (error.code === "ENOENT") return null;
    throw error;
  }
}

async function validateTree(root, current = root) {
  for (const entry of await readdir(current)) {
    const path = join(current, entry);
    const entryMetadata = await lstat(path);
    if (entryMetadata.isSymbolicLink()) {
      throw new Error(`WASM package contains a symbolic link: ${relative(root, path)}`);
    }
    if (entryMetadata.isDirectory()) await validateTree(root, path);
    else if (!entryMetadata.isFile()) {
      throw new Error(`WASM package contains a special file: ${relative(root, path)}`);
    }
  }
}

async function validatePackage(root) {
  const rootMetadata = await metadata(root);
  if (!rootMetadata?.isDirectory() || rootMetadata.isSymbolicLink()) {
    throw new Error("WASM package must be a real directory");
  }
  await validateTree(root);
  for (const requiredFile of requiredFiles) {
    const requiredMetadata = await metadata(join(root, requiredFile));
    if (!requiredMetadata?.isFile() || requiredMetadata.isSymbolicLink()) {
      throw new Error(`WASM package is missing ${requiredFile}`);
    }
  }
}

const packageRoot = await realpath(sourcePackage);
await validatePackage(packageRoot);
await rm(destination, { recursive: true, force: true });
await mkdir(join(destination, "pkg"), { recursive: true });
await cp(join(repositoryRoot, "index.html"), join(destination, "index.html"));
await cp(join(repositoryRoot, "assets"), join(destination, "assets"), { recursive: true });
for (const entry of await readdir(packageRoot)) {
  await cp(join(packageRoot, entry), join(destination, "pkg", entry), {
    recursive: true,
    force: false,
    errorOnExist: true,
  });
}
