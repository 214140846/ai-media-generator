# ai-media

给 ricebowl.ai / openclaw 等 agent 使用的 AI generation CLI。

独立仓库：`https://github.com/214140846/ai-media-generator`

## Install

```bash
# Rust
cargo install ai-media-generator

# npm / pnpm / bun
npm install -g ai-media-generator
pnpm add -g ai-media-generator
bun add -g ai-media-generator

# Python
pipx install ai-media-generator
```

## Quick Start

```bash
source "$HOME/.cargo/env"
cargo run --manifest-path cli/genmedia/Cargo.toml -- config set-base-url http://127.0.0.1:3001
cargo run --manifest-path cli/genmedia/Cargo.toml -- config set-key gm_xxx
cargo run --manifest-path cli/genmedia/Cargo.toml -- models list --json

# Installed binary
ai-media config set-base-url http://127.0.0.1:3001
ai-media config set-key gm_xxx
ai-media models list --json
```

环境变量：

```bash
export AI_MEDIA_BASE_URL=http://127.0.0.1:3000
export AI_MEDIA_API_KEY=gm_xxx
```

## Image

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "a cinematic ramen bowl on a wooden table" \
  --aspect-ratio 1:1 \
  --wait
```

## Video

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
```

## Distribution

- `crates.io`: `cargo install ai-media-generator`
- `npm`: `npx ai-media-generator` / `pnpm dlx ai-media-generator`
- `bun`: `bunx ai-media-generator`
- `PyPI`: `pipx install ai-media-generator`
