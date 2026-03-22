import { spawnSync } from "node:child_process";

import { ensurePackageVersion, getAiMediaVersion, rootPath } from "./release-lib";

const version = getAiMediaVersion();
const packageJsonPath = rootPath("packages", "ai-media", "package.json");
ensurePackageVersion(packageJsonPath, version);

const result = spawnSync("pnpm", ["publish", "--access", "public", "--no-git-checks"], {
  cwd: rootPath("packages", "ai-media"),
  stdio: "inherit"
});

process.exit(result.status ?? 1);
