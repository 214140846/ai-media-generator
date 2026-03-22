# ai-media-generator

`ai-media-generator` 是 `ai-media` CLI 的独立发布仓库。

它统一承载：

- Rust CLI 源码
- npm thin wrapper
- PyPI thin wrapper
- GitHub Releases 构建与发布脚本

## Layout

```text
cli/genmedia           Rust CLI
packages/ai-media      npm wrapper
packages/ai-media-py   PyPI wrapper
scripts/cli            release helpers
```

## Local Development

```bash
pnpm install
cargo test --manifest-path cli/genmedia/Cargo.toml
cargo run --manifest-path cli/genmedia/Cargo.toml -- --help
pnpm run cli:test:npm
pnpm run cli:test:pypi
```

## Release

```bash
pnpm run cli:build:release
pnpm run cli:publish:npm
pnpm run cli:publish:pypi
cargo publish --manifest-path cli/genmedia/Cargo.toml
```

对外安装身份保持不变：

- crate: `ai-media-generator`
- npm: `ai-media-generator`
- PyPI: `ai-media-generator`
- binary: `ai-media`
