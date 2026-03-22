const test = require("node:test");
const assert = require("node:assert/strict");
const {
  buildAssetName,
  buildReleaseUrl,
  resolveBinaryFilename,
  resolveTarget
} = require("./platform");

test("resolveTarget maps supported platforms", () => {
  assert.equal(resolveTarget("darwin", "arm64"), "aarch64-apple-darwin");
  assert.equal(resolveTarget("linux", "x64"), "x86_64-unknown-linux-gnu");
  assert.equal(resolveTarget("win32", "x64"), "x86_64-pc-windows-msvc");
});

test("resolveBinaryFilename uses .exe on windows only", () => {
  assert.equal(resolveBinaryFilename("linux"), "ai-media");
  assert.equal(resolveBinaryFilename("win32"), "ai-media.exe");
});

test("buildReleaseUrl uses ai-media release tag and asset naming", () => {
  assert.equal(
    buildAssetName("x86_64-unknown-linux-gnu"),
    "ai-media-generator-x86_64-unknown-linux-gnu"
  );
  assert.equal(
    buildReleaseUrl("0.1.2", "x86_64-unknown-linux-gnu"),
    "https://github.com/214140846/ai-media-generator/releases/download/ai-media-generator-v0.1.2/ai-media-generator-x86_64-unknown-linux-gnu"
  );
});
