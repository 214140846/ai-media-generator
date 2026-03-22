# ai-media-generator

`ai-media-generator` is the release repository for the `ai-media` CLI.

This repository is intended to be open-source friendly and is licensed under `MIT`.

It contains:

- Rust CLI source code
- npm thin wrapper
- PyPI thin wrapper
- GitHub Releases build and publish scripts

The default hosted platform is:

- <a href="https://ricebowl.ai">ricebowl.ai</a>

## Install As A Skill

This repository now uses `ricebowl.ai` as the default platform and exposes two layers of skills:

- a general CLI onboarding skill
- search-intent skills for image, video, workflows, and model-specific entry points

Recommended search-intent skills:

```bash
npx skills add https://github.com/214140846/ai-media-generator --skill ai-image-generation
npx skills add https://github.com/214140846/ai-media-generator --skill ai-video-generation
npx skills add https://github.com/214140846/ai-media-generator --skill text-to-video
npx skills add https://github.com/214140846/ai-media-generator --skill image-to-video
npx skills add https://github.com/214140846/ai-media-generator --skill text-to-image
npx skills add https://github.com/214140846/ai-media-generator --skill image-to-image
npx skills add https://github.com/214140846/ai-media-generator --skill flux-image-generator
npx skills add https://github.com/214140846/ai-media-generator --skill nano-banana-image-generator
npx skills add https://github.com/214140846/ai-media-generator --skill veo-video-generator
npx skills add https://github.com/214140846/ai-media-generator --skill seedance-video-generator
```

If you want the general CLI onboarding skill instead:

```bash
npx skills add https://github.com/214140846/ai-media-generator --skill ai-media-cli
```

You can also install each skill directly from its subdirectory:

```bash
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/ai-media-cli
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/ai-image-generation
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/ai-video-generation
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/text-to-video
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/image-to-video
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/text-to-image
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/image-to-image
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/flux-image-generator
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/nano-banana-image-generator
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/veo-video-generator
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/seedance-video-generator
```

`sora2-video-generator` is kept only as a legacy search-intent entry point and is no longer part of the default recommendation set.

`skills.sh` cards:

[![skills.sh card](https://skills.sh/214140846/ai-media-generator/ai-media-cli/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/ai-media-cli)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/ai-image-generation/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/ai-image-generation)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/ai-video-generation/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/ai-video-generation)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/text-to-video/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/text-to-video)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/image-to-video/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/image-to-video)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/text-to-image/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/text-to-image)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/image-to-image/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/image-to-image)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/flux-image-generator/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/flux-image-generator)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/nano-banana-image-generator/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/nano-banana-image-generator)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/veo-video-generator/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/veo-video-generator)
[![skills.sh card](https://skills.sh/214140846/ai-media-generator/seedance-video-generator/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/seedance-video-generator)

Published package identities:

- crate: `ai-media-generator`
- npm: `ai-media-generator`
- PyPI: `ai-media-generator`
- binary: `ai-media`

## What It Can Do

`ai-media` is designed for agents and automation scripts. It currently supports:

- storing an API key
- listing available image and video models
- creating image generation tasks
- creating video generation tasks
- polling async tasks until completion
- returning JSON that is easy to pipe into scripts

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
uv tool install ai-media-generator
```

## Quick Start

If this is your first time using `ricebowl.ai`, use this onboarding flow:

```text
sign in
  -> recharge credits
  -> create API key
  -> set key
  -> models list --json
```

```bash
# save your managed API key
ai-media config set-key gm_xxx

# inspect active config
ai-media config show

# list available models
ai-media models list
ai-media models list --json
ai-media models show --model veo3-1

# generate one image
ai-media image generate \
  --model nano-banana \
  --prompt "a cinematic ramen bowl on a wooden table" \
  --aspect-ratio 1:1 \
  --image https://example.com/reference.png \
  --param vendor_options='{"style":"cinematic"}' \
  --wait

# generate one video
ai-media video generate \
  --model veo3-1 \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 4 \
  --param audio=true \
  --wait
```

You can also override local config with environment variables:

```bash
export AI_MEDIA_API_KEY=gm_xxx
```

## Ricebowl Onboarding

1. Open <a href="https://ricebowl.ai/pricing">ricebowl.ai/pricing</a> and buy credits or a subscription.
2. Sign in and go to `Profile`.
3. Open `API Keys`.
4. Click `Create API Key`.
5. Copy the plaintext key immediately, then run `ai-media config set-key gm_xxx`.

## Docs

- Full CLI manual: [`cli/genmedia/README.md`](cli/genmedia/README.md)
- npm wrapper notes: [`packages/ai-media/README.md`](packages/ai-media/README.md)
- PyPI wrapper notes: [`packages/ai-media-py/README.md`](packages/ai-media-py/README.md)
- Codex skill: [`skills/ai-media-cli/SKILL.md`](skills/ai-media-cli/SKILL.md)
- Search-intent skill: [`skills/ai-image-generation/SKILL.md`](skills/ai-image-generation/SKILL.md)
- Search-intent skill: [`skills/ai-video-generation/SKILL.md`](skills/ai-video-generation/SKILL.md)
- Search-intent skill: [`skills/sora2-video-generator/SKILL.md`](skills/sora2-video-generator/SKILL.md)
- Search-intent skill: [`skills/text-to-video/SKILL.md`](skills/text-to-video/SKILL.md)
- Search-intent skill: [`skills/image-to-video/SKILL.md`](skills/image-to-video/SKILL.md)
- Search-intent skill: [`skills/text-to-image/SKILL.md`](skills/text-to-image/SKILL.md)
- Search-intent skill: [`skills/image-to-image/SKILL.md`](skills/image-to-image/SKILL.md)
- Search-intent skill: [`skills/flux-image-generator/SKILL.md`](skills/flux-image-generator/SKILL.md)
- Search-intent skill: [`skills/nano-banana-image-generator/SKILL.md`](skills/nano-banana-image-generator/SKILL.md)
- Search-intent skill: [`skills/veo-video-generator/SKILL.md`](skills/veo-video-generator/SKILL.md)
- Search-intent skill: [`skills/seedance-video-generator/SKILL.md`](skills/seedance-video-generator/SKILL.md)
- Legacy skill: [`skills/sora2-video-generator/SKILL.md`](skills/sora2-video-generator/SKILL.md)

## Repository Layout

```text
cli/genmedia           Rust CLI
packages/ai-media      npm wrapper
packages/ai-media-py   PyPI wrapper
scripts/cli            release helpers
skills/ai-media-cli    repo-local onboarding skill
skills/ai-image-generation
skills/ai-video-generation
skills/sora2-video-generator
skills/text-to-video
skills/image-to-video
skills/text-to-image
skills/image-to-image
skills/flux-image-generator
skills/nano-banana-image-generator
skills/veo-video-generator
skills/seedance-video-generator
skills/sora2-video-generator (legacy, hidden from default install path)
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
