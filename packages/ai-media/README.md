# ai-media-generator npm wrapper

这个包不重新实现第二套 CLI。

它的职责只有一个：为 `npm` / `pnpm` / `bun` 用户安装并调用同一个 `ai-media` Rust 二进制。

源码仓库：`https://github.com/214140846/ai-media-generator`

完整命令手册见：

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)

当前这套 CLI 已接入：

- <a href="https://ricebowl.ai">ricebowl.ai</a>
- <a href="https://sora2.cloud">sora2.cloud</a>

## Install

```bash
npm install -g ai-media-generator
pnpm add -g ai-media-generator
bun add -g ai-media-generator
```

## Run Without Global Install

```bash
npx ai-media-generator --help
pnpm dlx ai-media-generator --help
bunx ai-media-generator --help
```

## Usage

安装完成后，实际命令名是：

```bash
ai-media
```

例如：

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media models list --json
```

如果你对接 `sora2.cloud`：

```bash
ai-media config set-base-url https://sora2.cloud
ai-media config set-key gm_xxx
ai-media models list --json
```

## Onboarding

第一次接平台时，推荐这样做：

1. 去 <a href="https://ricebowl.ai/pricing">ricebowl.ai/pricing</a> 或 <a href="https://sora2.cloud/pricing">sora2.cloud/pricing</a> 充值
2. 登录后在账号区创建 API key
3. 立刻复制明文 `gm_xxx`
4. 用 `ai-media config set-base-url ...` 和 `ai-media config set-key ...` 写入本地配置

更完整的参数、充值和 key onboarding 见：

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)
- [`../../skills/ai-media-cli/SKILL.md`](../../skills/ai-media-cli/SKILL.md)

## Wrapper Notes

- package name: `ai-media-generator`
- executable name: `ai-media`
- the wrapper downloads the release binary for your platform

## Development

如果你在本仓库里本地联调，可以直接指定已有二进制：

```bash
AI_MEDIA_BINARY_PATH=../../cli/genmedia/target/debug/ai-media node bin/ai-media.js --help
```
