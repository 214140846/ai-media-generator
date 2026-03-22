# ai-media-generator

这个包是 `ai-media` Rust CLI 的 PyPI thin wrapper。

它不会重新实现另一套 Python CLI，而是安装并调用同一个 `ai-media` 二进制。

源码仓库：`https://github.com/214140846/ai-media-generator`

完整命令手册见：

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)

当前这套 CLI 已接入：

- <a href="https://ricebowl.ai">ricebowl.ai</a>
- <a href="https://sora2.cloud">sora2.cloud</a>

## Install

```bash
pipx install ai-media-generator
uv tool install ai-media-generator
```

## Run Without Persistent Install

```bash
uvx ai-media-generator --help
python -m ai_media_cli --help
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
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
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
