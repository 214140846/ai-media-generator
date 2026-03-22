import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";

import {
  ensurePackageVersion,
  ensurePyProjectVersion,
  getAiMediaVersion,
  releaseAssetName,
  resolveCargoBin,
  resolveReleaseTargets,
  resolveRustcBin,
  rootPath
} from "./release-lib";

const version = getAiMediaVersion();
const cargoBin = resolveCargoBin();
const rustcBin = resolveRustcBin();
const cargoBinDir = path.dirname(cargoBin);
const releaseTargets = resolveReleaseTargets();
ensurePackageVersion(rootPath("packages", "ai-media", "package.json"), version);
ensurePyProjectVersion(rootPath("packages", "ai-media-py", "pyproject.toml"), version);

const outputDir = rootPath(".tmp", "ai-media-generator-release", version);
fs.mkdirSync(outputDir, { recursive: true });

console.log(`building targets: ${releaseTargets.map((target) => target.target).join(", ")}`);

for (const target of releaseTargets) {
  const build = spawnSync(
    cargoBin,
    [
      "build",
      "--release",
      "--manifest-path",
      rootPath("cli", "genmedia", "Cargo.toml"),
      "--target",
      target.target
    ],
    {
      env: {
        ...process.env,
        RUSTC: rustcBin,
        PATH: `${cargoBinDir}:${process.env.PATH ?? ""}`
      },
      stdio: "inherit"
    }
  );

  if (build.status !== 0) {
    process.exit(build.status ?? 1);
  }

  const source = rootPath(
    "cli",
    "genmedia",
    "target",
    target.target,
    "release",
    target.binaryName
  );
  const destination = path.join(outputDir, releaseAssetName(target));
  fs.copyFileSync(source, destination);
  if (!target.binaryName.endsWith(".exe")) {
    fs.chmodSync(destination, 0o755);
  }
  console.log(`using cargo binary: ${cargoBin}`);
  console.log(`using rustc binary: ${rustcBin}`);
  console.log(`built ${destination}`);
}
