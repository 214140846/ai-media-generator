#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const { resolveBinaryFilename } = require("../scripts/lib/platform");

const envBinaryPath = process.env.AI_MEDIA_BINARY_PATH;
const packagedBinaryPath = path.join(
  __dirname,
  "..",
  "vendor",
  resolveBinaryFilename(process.platform)
);

const binaryPath = envBinaryPath || packagedBinaryPath;

if (!fs.existsSync(binaryPath)) {
  console.error(
    [
      "ai-media binary is not installed yet.",
      `Expected binary at: ${binaryPath}`,
      "Reinstall the package with network access, or set AI_MEDIA_BINARY_PATH to a local binary."
    ].join("\n")
  );
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  env: process.env
});

if (result.error) {
  console.error(`Failed to launch ai-media: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status ?? 0);
