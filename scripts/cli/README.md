# ai-media-generator release scripts

这些脚本默认在 `ai-media-generator` 仓库根目录执行。

本地构建时，脚本会优先使用 `AI_MEDIA_CARGO_BIN`，否则优先尝试 `~/.cargo/bin/cargo`，避免 PATH 中的 Homebrew Rust 看不到 rustup 安装的 targets。

## Local validation

```bash
cargo test --manifest-path cli/genmedia/Cargo.toml
cargo run --manifest-path cli/genmedia/Cargo.toml -- --help
node --test packages/ai-media/scripts/lib/platform.test.js
PYTHONPATH=packages/ai-media-py/src python3 -m unittest packages/ai-media-py/tests/test_platforms.py
```

## Build release artifacts

```bash
pnpm run cli:build:release
```

如果本地机器没有 Linux / Windows 交叉编译工具链，可以只构建当前需要的目标：

```bash
AI_MEDIA_RELEASE_TARGETS=aarch64-apple-darwin,x86_64-apple-darwin pnpm run cli:build:release
```

输出目录：

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

前提：

- GitHub Releases 已存在 `ai-media-generator-v<version>` tag 与对应二进制资产
- `NPM_TOKEN`、`PYPI_API_TOKEN`、`CARGO_REGISTRY_TOKEN` 已配置
