# ai-media-generator release scripts

These scripts are intended to run from the `ai-media-generator` repository root.

During local builds, the scripts prefer `AI_MEDIA_CARGO_BIN`. Otherwise they try `~/.cargo/bin/cargo` first so Homebrew Rust on `PATH` does not hide rustup-installed targets.

## Local Validation

```bash
cargo test --manifest-path cli/genmedia/Cargo.toml
cargo run --manifest-path cli/genmedia/Cargo.toml -- --help
node --test packages/ai-media/scripts/lib/platform.test.js
PYTHONPATH=packages/ai-media-py/src python3 -m unittest packages/ai-media-py/tests/test_platforms.py
```

## Build Release Artifacts

```bash
pnpm run cli:build:release
```

If the local machine does not have Linux or Windows cross-compilation toolchains, build only the targets you need:

```bash
AI_MEDIA_RELEASE_TARGETS=aarch64-apple-darwin,x86_64-apple-darwin pnpm run cli:build:release
```

Output directory:

```text
.tmp/ai-media-generator-release/<version>/
  ai-media-generator-aarch64-apple-darwin
  ai-media-generator-x86_64-apple-darwin
  ai-media-generator-x86_64-unknown-linux-gnu
  ai-media-generator-x86_64-pc-windows-msvc.exe
```

## Publish

```bash
pnpm run cli:publish:npm
pnpm run cli:publish:pypi
cargo publish --manifest-path cli/genmedia/Cargo.toml
```

Prerequisites:

- GitHub Releases already contains the `ai-media-generator-v<version>` tag and matching binary assets
- `NPM_TOKEN`, `PYPI_API_TOKEN`, and `CARGO_REGISTRY_TOKEN` are configured
