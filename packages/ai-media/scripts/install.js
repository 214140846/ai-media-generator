const fs = require("node:fs");
const path = require("node:path");
const { downloadReleaseBinary } = require("./lib/download");
const { resolveTarget, resolveBinaryFilename } = require("./lib/platform");

async function main() {
  if (process.env.AI_MEDIA_SKIP_DOWNLOAD === "1") {
    console.log("[ai-media] skipping binary download because AI_MEDIA_SKIP_DOWNLOAD=1");
    return;
  }

  if (process.env.AI_MEDIA_BINARY_PATH) {
    console.log("[ai-media] skipping binary download because AI_MEDIA_BINARY_PATH is set");
    return;
  }

  const pkg = require("../package.json");
  const target = resolveTarget(process.platform, process.arch);
  const destinationDir = path.join(__dirname, "..", "vendor");
  const destination = path.join(destinationDir, resolveBinaryFilename(process.platform));

  fs.mkdirSync(destinationDir, { recursive: true });

  await downloadReleaseBinary({
    version: pkg.version,
    target,
    destination
  });

  if (process.platform !== "win32") {
    fs.chmodSync(destination, 0o755);
  }

  console.log(`[ai-media] installed ${target} binary to ${destination}`);
}

main().catch((error) => {
  console.error(`[ai-media] install failed: ${error.message}`);
  process.exit(1);
});
