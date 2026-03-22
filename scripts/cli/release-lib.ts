import fs from "node:fs";
import path from "node:path";
import os from "node:os";

const ROOT = path.resolve(__dirname, "..", "..");
const CARGO_TOML = path.join(ROOT, "cli", "genmedia", "Cargo.toml");

export type ReleaseTarget = {
  target: string;
  binaryName: string;
};

export const RELEASE_TARGETS: ReleaseTarget[] = [
  { target: "aarch64-apple-darwin", binaryName: "ai-media" },
  { target: "x86_64-apple-darwin", binaryName: "ai-media" },
  { target: "x86_64-unknown-linux-gnu", binaryName: "ai-media" },
  { target: "x86_64-pc-windows-msvc", binaryName: "ai-media.exe" }
];

export function resolveReleaseTargets(): ReleaseTarget[] {
  const raw = process.env.AI_MEDIA_RELEASE_TARGETS?.trim();
  if (!raw) {
    return RELEASE_TARGETS;
  }

  const allowed = new Set(
    raw
      .split(",")
      .map((item) => item.trim())
      .filter(Boolean)
  );

  const selected = RELEASE_TARGETS.filter((target) => allowed.has(target.target));
  if (selected.length === 0) {
    throw new Error(
      `AI_MEDIA_RELEASE_TARGETS did not match any known target. Got: ${raw}`
    );
  }

  return selected;
}

export function getAiMediaVersion(): string {
  const cargoToml = fs.readFileSync(CARGO_TOML, "utf8");
  const match = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);
  if (!match) {
    throw new Error(`Failed to read ai-media-generator version from ${CARGO_TOML}`);
  }
  return match[1];
}

export function ensurePackageVersion(filePath: string, expectedVersion: string) {
  const raw = fs.readFileSync(filePath, "utf8");
  const versionMatch = raw.match(/"version"\s*:\s*"([^"]+)"/);
  if (!versionMatch) {
    throw new Error(`Missing version field in ${filePath}`);
  }
  if (versionMatch[1] !== expectedVersion) {
    throw new Error(
      `Version mismatch in ${filePath}: expected ${expectedVersion}, got ${versionMatch[1]}`
    );
  }
}

export function ensurePyProjectVersion(filePath: string, expectedVersion: string) {
  const raw = fs.readFileSync(filePath, "utf8");
  const versionMatch = raw.match(/^version\s*=\s*"([^"]+)"/m);
  if (!versionMatch) {
    throw new Error(`Missing version field in ${filePath}`);
  }
  if (versionMatch[1] !== expectedVersion) {
    throw new Error(
      `Version mismatch in ${filePath}: expected ${expectedVersion}, got ${versionMatch[1]}`
    );
  }
}

export function releaseAssetName(target: ReleaseTarget): string {
  return `ai-media-generator-${target.target}${target.binaryName.endsWith(".exe") ? ".exe" : ""}`;
}

export function rootPath(...parts: string[]) {
  return path.join(ROOT, ...parts);
}

export function resolveCargoBin(): string {
  if (process.env.AI_MEDIA_CARGO_BIN) {
    return process.env.AI_MEDIA_CARGO_BIN;
  }

  const rustupCargo = path.join(os.homedir(), ".cargo", "bin", "cargo");
  if (fs.existsSync(rustupCargo)) {
    return rustupCargo;
  }

  return "cargo";
}

export function resolveRustcBin(): string {
  if (process.env.AI_MEDIA_RUSTC_BIN) {
    return process.env.AI_MEDIA_RUSTC_BIN;
  }

  const rustupRustc = path.join(os.homedir(), ".cargo", "bin", "rustc");
  if (fs.existsSync(rustupRustc)) {
    return rustupRustc;
  }

  return "rustc";
}
