# ai-media-generator

`ai-media-generator` 是 `ai-media` CLI 的独立发布仓库。

这是一个可直接对外开源的 CLI 项目，仓库级许可证使用 `MIT`。

它统一承载：

- Rust CLI 源码
- npm thin wrapper
- PyPI thin wrapper
- GitHub Releases 构建与发布脚本

目前已经接入这些 hosted platforms：

- <a href="https://ricebowl.ai">ricebowl.ai</a>
- <a href="https://sora2.cloud">sora2.cloud</a>

## Install As A Skill

这个仓库里的 `ai-media-cli` 也可以直接被 `skills` CLI 识别，已经验证过下面两种装法都能成功列出 skill：

```bash
npx skills add https://github.com/214140846/ai-media-generator --skill ai-media-cli
```

或者直接装 skill 子目录：

```bash
npx skills add https://github.com/214140846/ai-media-generator/tree/main/skills/ai-media-cli
```

GitHub README 里的 `skills.sh` Markdown 卡片：

[![skills.sh card](https://skills.sh/214140846/ai-media-generator/ai-media-cli/opengraph-image)](https://github.com/214140846/ai-media-generator/tree/main/skills/ai-media-cli)

对外安装身份：

- crate: `ai-media-generator`
- npm: `ai-media-generator`
- PyPI: `ai-media-generator`
- binary: `ai-media`

## What It Can Do

`ai-media` 面向 agent 和自动化脚本，当前提供这些能力：

- 配置 API key 和 base URL
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

如果你第一次接入 hosted platform，建议先走这条 onboarding：

```text
sign in
  -> recharge credits
  -> create API key
  -> set base_url
  -> set key
  -> models list --json
```

```bash
# point the CLI at your API server
ai-media config set-base-url https://ricebowl.ai

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
export AI_MEDIA_BASE_URL=https://ricebowl.ai
export AI_MEDIA_API_KEY=gm_xxx
```

如果你对接的是 `sora2.cloud`，只需要把 `base_url` 换成：

```bash
ai-media config set-base-url https://sora2.cloud
```

## Hosted Platform Onboarding

### ricebowl.ai

1. 去 <a href="https://ricebowl.ai/pricing">pricing</a> 充值或订阅
2. 登录后进入 `Profile`
3. 打开 `API Keys`
4. 点击 `Create API Key`
5. 立刻复制明文 key，并执行：

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
```

### sora2.cloud

1. 去 <a href="https://sora2.cloud/pricing">pricing</a> 购买 credits 或订阅
2. 登录后进入账号区
3. 找 `API Keys` / `Developer` / `Integrations`
4. 创建 key 并立即复制
5. 然后执行：

```bash
ai-media config set-base-url https://sora2.cloud
ai-media config set-key gm_xxx
ai-media config show
```

## Docs

- Full CLI manual: [`cli/genmedia/README.md`](cli/genmedia/README.md)
- npm wrapper notes: [`packages/ai-media/README.md`](packages/ai-media/README.md)
- PyPI wrapper notes: [`packages/ai-media-py/README.md`](packages/ai-media-py/README.md)
- Codex skill: [`skills/ai-media-cli/SKILL.md`](skills/ai-media-cli/SKILL.md)

## Repository Layout

```text
cli/genmedia           Rust CLI
packages/ai-media      npm wrapper
packages/ai-media-py   PyPI wrapper
scripts/cli            release helpers
skills/ai-media-cli    repo-local onboarding skill
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
