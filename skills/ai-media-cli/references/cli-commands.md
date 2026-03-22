# ai-media CLI Commands

## Install Matrix

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

实际可执行命令：

```bash
ai-media
```

## Config

### `ai-media config set-key <KEY>`

- 用途：保存受管 API key
- 例子：`ai-media config set-key gm_xxx`

### `ai-media config show`

- 用途：打印当前配置
- 输出：pretty JSON

## Default Platform

当前默认平台固定为 `ricebowl.ai`，用户不需要手动设置 `base_url`。

历史兼容说明：

- 旧版文档可能会出现 `ai-media config set-base-url ...`
- 旧版环境变量可能会出现 `AI_MEDIA_BASE_URL`
- 这两种写法现在都不作为默认用户路径推荐

## Models

### `ai-media models list`

- 用途：查看当前 API 暴露的模型
- 输出：按 `Videos` 和 `Images` 分组的人类可读文本

### `ai-media models list --json`

- 用途：给 agent、脚本、流水线消费
- 输出：JSON

## Image

### `ai-media image generate`

用法：

```bash
ai-media image generate [OPTIONS] --model <MODEL> --prompt <PROMPT>
```

参数：

- `--model <MODEL>` 必填
- `--prompt <PROMPT>` 必填
- `--aspect-ratio <ASPECT_RATIO>` 选填
- `--wait` 选填
- `--poll-interval <POLL_INTERVAL>` 选填，默认 `5`

常用例子：

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "a cinematic ramen bowl on a wooden table" \
  --aspect-ratio 1:1
```

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "studio portrait lighting, close-up ramen bowl" \
  --aspect-ratio 1:1 \
  --wait \
  --poll-interval 3
```

行为：

- 不带 `--wait`：打印初始创建任务响应
- 带 `--wait`：轮询直到任务变成 `completed` 或 `failed`

### `ai-media image get`

用法：

```bash
ai-media image get [OPTIONS] --task-id <TASK_ID>
```

参数：

- `--task-id <TASK_ID>` 必填
- `--wait` 选填
- `--poll-interval <POLL_INTERVAL>` 选填，默认 `5`

## Video

### `ai-media video generate`

用法：

```bash
ai-media video generate [OPTIONS] --model <MODEL> --prompt <PROMPT>
```

参数：

- `--model <MODEL>` 必填
- `--prompt <PROMPT>` 必填
- `--aspect-ratio <ASPECT_RATIO>` 选填
- `--duration <DURATION>` 选填
- `--wait` 选填
- `--poll-interval <POLL_INTERVAL>` 选填，默认 `8`

常用例子：

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2
```

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "slow dolly shot over a steaming rice bowl" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
```

行为：

- 不带 `--wait`：打印初始创建任务响应
- 带 `--wait`：轮询直到任务变成 `SUCCESS` 或 `FAILURE`

### `ai-media video get`

用法：

```bash
ai-media video get [OPTIONS] --task-id <TASK_ID>
```

参数：

- `--task-id <TASK_ID>` 必填
- `--wait` 选填
- `--poll-interval <POLL_INTERVAL>` 选填，默认 `5`

## Task

### `ai-media task get`

用法：

```bash
ai-media task get [OPTIONS] --kind <KIND> --task-id <TASK_ID>
```

参数：

- `--kind <KIND>` 必填，可选值：`image`、`video`
- `--task-id <TASK_ID>` 必填
- `--wait` 选填
- `--poll-interval <POLL_INTERVAL>` 选填，默认 `5`

## Config Precedence

```text
AI_MEDIA_API_KEY
  -> local config file
  -> built-in ricebowl.ai default
```

本地配置文件路径：

```text
<system config dir>/ai-media/config.json
```

## Script-Friendly Usage

适合自动化的命令：

- `ai-media config show`
- `ai-media models list --json`
- `ai-media image generate`
- `ai-media image get`
- `ai-media video generate`
- `ai-media video get`
- `ai-media task get`

例子：

```bash
ai-media models list --json
ai-media image get --task-id img_task_xxx | jq .
```

## Historical Notes

如果你在旧脚本里看到 `set-base-url` 或 `AI_MEDIA_BASE_URL`，那是历史兼容写法。当前默认路径里，只有 `AI_MEDIA_API_KEY` 还保留为推荐的可覆盖配置。
