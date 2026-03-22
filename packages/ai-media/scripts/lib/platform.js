const SUPPORTED_TARGETS = {
  "darwin:arm64": "aarch64-apple-darwin",
  "darwin:x64": "x86_64-apple-darwin",
  "linux:x64": "x86_64-unknown-linux-gnu",
  "win32:x64": "x86_64-pc-windows-msvc"
};

function resolveTarget(platform, arch) {
  const key = `${platform}:${arch}`;
  const target = SUPPORTED_TARGETS[key];
  if (!target) {
    throw new Error(`Unsupported platform ${key} for ai-media`);
  }
  return target;
}

function resolveBinaryFilename(platform) {
  return platform === "win32" ? "ai-media.exe" : "ai-media";
}

function buildAssetName(target) {
  return target.includes("windows")
    ? `ai-media-generator-${target}.exe`
    : `ai-media-generator-${target}`;
}

function buildReleaseUrl(version, target) {
  const asset = buildAssetName(target);
  return `https://github.com/214140846/ai-media-generator/releases/download/ai-media-generator-v${version}/${asset}`;
}

module.exports = {
  buildAssetName,
  buildReleaseUrl,
  resolveBinaryFilename,
  resolveTarget
};
