# ai-media-generator

`ai-media-generator` 是 `ai-media` CLI 的独立发布仓库。

这是一个可直接对外开源的 CLI 项目，仓库级许可证使用 `MIT`。

它统一承载：

- Rust CLI 源码
- npm thin wrapper
- PyPI thin wrapper
- GitHub Releases 构建与发布脚本

默认接入的平台是：

- <a href="https://ricebowl.ai">ricebowl.ai</a>

## Install As A Skill

这个仓库里的 skill 现在以 `ricebowl.ai` 为默认平台，分成“泛 CLI 入口”和“搜索意图入口”两层。

推荐优先安装这些搜索型 skill：

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

如果你要的是通用 CLI onboarding，再装：

```bash
npx skills add https://github.com/214140846/ai-media-generator --skill ai-media-cli
```

也可以直接装各自的 skill 子目录：

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

`sora2-video-generator` 作为旧搜索词入口保留，但不是默认推荐入口。

GitHub README 里的 `skills.sh` Markdown 卡片：

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

对外安装身份：

- crate: `ai-media-generator`
- npm: `ai-media-generator`
- PyPI: `ai-media-generator`
- binary: `ai-media`

## What It Can Do

`ai-media` 面向 agent 和自动化脚本，当前提供这些能力：

- 配置 API key
- 查询当前服务支持的视频 / 图片模型
- 发起图片生成任务
- 发起视频生成任务
- 轮询图片 / 视频任务直到完成
- 输出适合脚本消费的 JSON 结果

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

如果你第一次接入 `ricebowl.ai`，建议先走这条 onboarding：

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

# generate one image
ai-media image generate \
  --model nano-banana \
  --prompt "a cinematic ramen bowl on a wooden table" \
  --aspect-ratio 1:1 \
  --wait

# generate one video
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
```

也可以用环境变量覆盖本地配置：

```bash
export AI_MEDIA_API_KEY=gm_xxx
```

## Ricebowl Onboarding

1. 去 <a href="https://ricebowl.ai/pricing">pricing</a> 充值或订阅
2. 登录后进入 `Profile`
3. 打开 `API Keys`
4. 点击 `Create API Key`
5. 立刻复制明文 key，然后执行 `ai-media config set-key gm_xxx`

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
