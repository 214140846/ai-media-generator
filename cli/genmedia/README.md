# ai-media CLI Manual

`ai-media` 是一个面向 agent 工作流的 AI generation CLI。

它直接调用后端 HTTP API，帮你完成：

- 配置 API key 和服务地址
- 查询可用图片 / 视频模型
- 发起图片生成任务
- 发起视频生成任务
- 轮询异步任务直到完成
- 输出适合脚本二次处理的 JSON

源码仓库：`https://github.com/214140846/ai-media-generator`

已接入的平台：

- <a href="https://ricebowl.ai">ricebowl.ai</a>
- <a href="https://sora2.cloud">sora2.cloud</a>

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

安装包名是 `ai-media-generator`，真正执行的命令名是：

```bash
ai-media
```

## Configuration

CLI 需要两个核心配置：

- `base_url`: 你的 API 服务地址
- `api_key`: 受管 API key，例如 `gm_xxx`

如果你是第一次接 hosted platform，推荐顺序：

```text
recharge credits
  -> create API key
  -> set base_url
  -> set key
  -> config show
  -> models list --json
```

### Save Config Locally

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
```

`sora2.cloud` 用法相同，只是把 `base_url` 换成：

```bash
ai-media config set-base-url https://sora2.cloud
```

CLI 会把配置写到系统配置目录下的 `ai-media/config.json`。

### Override With Environment Variables

环境变量优先级高于本地配置文件：

```bash
export AI_MEDIA_BASE_URL=https://ricebowl.ai
export AI_MEDIA_API_KEY=gm_xxx
```

如果没有显式配置 `base_url`，默认值是：

```bash
http://127.0.0.1:3000
```

## Hosted Platform Onboarding

### ricebowl.ai

充值：

- 打开 <a href="https://ricebowl.ai/pricing">ricebowl.ai/pricing</a>
- 选择计划或 credits 档位并完成支付

生成 key：

- 登录后进入 `Profile`
- 切到 `API Keys`
- 点击 `Create API Key`
- 立即复制明文 `gm_xxx`

配置 CLI：

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

### sora2.cloud

充值：

- 打开 <a href="https://sora2.cloud/pricing">sora2.cloud/pricing</a>
- 选择月付、年付或 one-time credits/top-up

生成 key：

- 登录后进入账号区
- 找 `API Keys`、`Developer` 或 `Integrations`
- 创建新的 key
- 立即复制明文 `gm_xxx`

配置 CLI：

```bash
ai-media config set-base-url https://sora2.cloud
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

提示：

- `ricebowl.ai` 的 `API Keys` 路由在源码里对应 `Profile -> API Keys`
- `sora2.cloud` 的 key 入口我没有拿到公开文档页，所以这里写的是保守 onboarding 路径

## Command Tree

```text
ai-media
  config
    set-key <KEY>
    set-base-url <BASE_URL>
    show
  models
    list [--json]
  image
    generate --model --prompt [--aspect-ratio] [--wait] [--poll-interval]
    get --task-id [--wait] [--poll-interval]
  video
    generate --model --prompt [--aspect-ratio] [--duration] [--wait] [--poll-interval]
    get --task-id [--wait] [--poll-interval]
  task
    get --kind <image|video> --task-id [--wait] [--poll-interval]
```

## Commands

### `config`

Manage local CLI configuration.

#### `config set-key <KEY>`

Save your managed API key locally.

```bash
ai-media config set-key gm_xxx
```

#### `config set-base-url <BASE_URL>`

Save the API server base URL.

```bash
ai-media config set-base-url https://ricebowl.ai
```

The CLI trims the trailing slash automatically.

#### `config show`

Print the current config as pretty JSON.

```bash
ai-media config show
```

### `models`

Inspect models currently exposed by the API.

#### `models list`

Human-readable output grouped by `Videos` and `Images`.

```bash
ai-media models list
```

#### `models list --json`

Machine-readable JSON output for scripts, agents, and pipelines.

```bash
ai-media models list --json
```

### `image`

Create or inspect image generation tasks.

#### `image generate`

Required parameters:

- `--model <MODEL>`
- `--prompt <PROMPT>`

Optional parameters:

- `--aspect-ratio <ASPECT_RATIO>`
- `--wait`
- `--poll-interval <SECONDS>` default `5`

Example:

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "a cinematic ramen bowl on a wooden table" \
  --aspect-ratio 1:1
```

Wait for completion:

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "studio portrait lighting, close-up ramen bowl" \
  --aspect-ratio 1:1 \
  --wait \
  --poll-interval 3
```

Behavior:

- without `--wait`, prints the initial create-task response
- with `--wait`, polls until the task becomes `completed` or `failed`

#### `image get`

Inspect an existing image task.

Parameters:

- `--task-id <TASK_ID>` required
- `--wait`
- `--poll-interval <SECONDS>` default `5`

```bash
ai-media image get --task-id img_task_xxx
ai-media image get --task-id img_task_xxx --wait --poll-interval 3
```

### `video`

Create or inspect video generation tasks.

#### `video generate`

Required parameters:

- `--model <MODEL>`
- `--prompt <PROMPT>`

Optional parameters:

- `--aspect-ratio <ASPECT_RATIO>`
- `--duration <SECONDS>`
- `--wait`
- `--poll-interval <SECONDS>` default `8`

Example:

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2
```

Wait for completion:

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "slow dolly shot over a steaming rice bowl" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
```

Behavior:

- without `--wait`, prints the initial create-task response
- with `--wait`, polls until the task becomes `SUCCESS` or `FAILURE`

#### `video get`

Inspect an existing video task.

Parameters:

- `--task-id <TASK_ID>` required
- `--wait`
- `--poll-interval <SECONDS>` default `5`

```bash
ai-media video get --task-id vid_task_xxx
ai-media video get --task-id vid_task_xxx --wait
```

### `task`

Use a single generic polling entry point when you already know the task kind.

#### `task get`

Parameters:

- `--kind <image|video>` required
- `--task-id <TASK_ID>` required
- `--wait`
- `--poll-interval <SECONDS>` default `5`

```bash
ai-media task get --kind image --task-id img_task_xxx
ai-media task get --kind video --task-id vid_task_xxx --wait
```

## Output Shape

The CLI prints JSON for task-oriented commands:

- `config show`
- `models list --json`
- `image generate`
- `image get`
- `video generate`
- `video get`
- `task get`

That makes it easy to pipe into other tools:

```bash
ai-media models list --json
ai-media image get --task-id img_task_xxx | jq .
```

## Common Workflows

### Local Development Against a Local Server

```bash
ai-media config set-base-url http://127.0.0.1:3000
ai-media config set-key gm_local_dev_key
ai-media models list --json
```

### Hosted Platform First Run

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

或者：

```bash
ai-media config set-base-url https://sora2.cloud
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

### Generate and Wait for an Image

```bash
ai-media image generate \
  --model nano-banana \
  --prompt "minimal studio packshot of a rice bowl" \
  --aspect-ratio 1:1 \
  --wait
```

### Generate and Wait for a Video

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "macro close-up of steam rising from a rice bowl, cinematic lighting" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
```

### Poll an Existing Task Later

```bash
ai-media task get --kind image --task-id img_task_xxx --wait
ai-media task get --kind video --task-id vid_task_xxx --wait
```

## Distribution Shortcuts

- `crates.io`: `cargo install ai-media-generator`
- `npm`: `npx ai-media-generator`
- `pnpm`: `pnpm dlx ai-media-generator`
- `bun`: `bunx ai-media-generator`
- `PyPI`: `pipx install ai-media-generator`
