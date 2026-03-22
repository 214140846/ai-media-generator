# ai-media CLI Manual

`ai-media` is an AI generation CLI for agent workflows.

It talks directly to the backend HTTP API and helps with:

- storing an API key and service address
- listing available image and video models
- creating image generation tasks
- creating video generation tasks
- polling async tasks until completion
- returning JSON that is easy to pipe into scripts

Source repository: `https://github.com/214140846/ai-media-generator`

Supported hosted platforms:

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

The installable package name is `ai-media-generator`, but the actual command name is:

```bash
ai-media
```

## Configuration

The CLI uses two core configuration values:

- `base_url`: your API service address
- `api_key`: a managed API key such as `gm_xxx`

Recommended first-run order for a hosted platform:

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

`sora2.cloud` works the same way, but with a different `base_url`:

```bash
ai-media config set-base-url https://sora2.cloud
```

The CLI writes config to `ai-media/config.json` inside the system config directory.

### Override With Environment Variables

Environment variables take precedence over the local config file:

```bash
export AI_MEDIA_BASE_URL=https://ricebowl.ai
export AI_MEDIA_API_KEY=gm_xxx
```

If `base_url` is not set explicitly, the default is:

```bash
http://127.0.0.1:3000
```

## Hosted Platform Onboarding

### ricebowl.ai

Recharge:

- Open <a href="https://ricebowl.ai/pricing">ricebowl.ai/pricing</a>
- Choose a plan or credit package and complete payment

Create an API key:

- Sign in and go to `Profile`
- Open `API Keys`
- Click `Create API Key`
- Copy the plaintext `gm_xxx` key immediately

Configure the CLI:

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

### sora2.cloud

Recharge:

- Open <a href="https://sora2.cloud/pricing">sora2.cloud/pricing</a>
- Choose monthly, yearly, or one-time credits/top-up

Create an API key:

- Sign in and open the account area
- Look for `API Keys`, `Developer`, or `Integrations`
- Create a new key
- Copy the plaintext `gm_xxx` key immediately

Configure the CLI:

```bash
ai-media config set-base-url https://sora2.cloud
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

Notes:

- The `ricebowl.ai` source code maps API key creation to `Profile -> API Keys`
- I did not find a public `sora2.cloud` API key documentation page, so this section uses a conservative onboarding path

## Command Tree

```text
ai-media
  config
    set-key <KEY>
    set-base-url <BASE_URL>
    show
  models
    list [--json]
    show --model <MODEL> [--json]
  image
    generate --model --prompt [--aspect-ratio] [--response-format] [--image]... [--metadata-json] [--param KEY=VALUE] [--wait] [--poll-interval]
    get --task-id [--wait] [--poll-interval]
  video
    generate --model --prompt [--aspect-ratio] [--duration] [--image]... [--param KEY=VALUE] [--wait] [--poll-interval]
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

#### `models show`

Inspect a single model and its parameter metadata.

```bash
ai-media models show --model veo3.1
ai-media models show --model veo3.1 --json
```

The text output includes the model kind plus a parameter summary, so agents can discover which fields to pass with `--param`.

### `image`

Create or inspect image generation tasks.

#### `image generate`

Required parameters:

- `--model <MODEL>`
- `--prompt <PROMPT>`

Optional parameters:

- `--aspect-ratio <ASPECT_RATIO>`
- `--response-format <url|b64_json>`
- `--image <IMAGE_URL>` repeatable
- `--metadata-json <JSON>`
- `--param <KEY=VALUE>`
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
  --image https://example.com/reference.png \
  --param vendor_options='{"style":"cinematic"}' \
  --wait \
  --poll-interval 3
```

Behavior:

- without `--wait`, prints the initial create-task response
- with `--wait`, polls until the task becomes `completed` or `failed`
- `--param` values are merged into the JSON body and JSON-looking values are parsed as JSON first

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
- `--image <IMAGE_URL>` repeatable
- `--param <KEY=VALUE>`
- `--wait`
- `--poll-interval <SECONDS>` default `8`

Example:

```bash
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 5 \
  --param resolution=720p \
  --param seed=42
```

Wait for completion:

```bash
ai-media video generate \
  --model veo3-1 \
  --prompt "slow dolly shot over a steaming rice bowl" \
  --aspect-ratio 16:9 \
  --duration 4 \
  --image https://example.com/start-frame.png \
  --param audio=true \
  --wait
```

Behavior:

- without `--wait`, prints the initial create-task response
- with `--wait`, polls until the task becomes `SUCCESS` or `FAILURE`
- `--param` values are merged into the JSON body and JSON-looking values are parsed as JSON first

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

## Model-Specific Parameters

Use the built-in flags for common fields:

- `--aspect-ratio`
- `--duration`

Start with `ai-media models show --model <MODEL>` to inspect the parameter metadata for the exact model you want to use.

Use `--param KEY=VALUE` for model-specific fields from the upstream API docs. Booleans, numbers, arrays, and objects are parsed as JSON when possible.

Examples:

```bash
# Veo text-to-video
ai-media video generate \
  --model veo3.1 \
  --prompt "cinematic bowl of ramen under soft light" \
  --aspect-ratio 16:9 \
  --param enhance_prompt=true \
  --param enable_upsample=true
```

```bash
# Veo image-to-video
ai-media video generate \
  --model veo3.1 \
  --prompt "steam drifting upward, gentle camera push-in" \
  --param images='["https://example.com/reference.png"]' \
  --aspect-ratio 16:9
```

```bash
# Flux / Recraft-style image models
ai-media image generate \
  --model bfl/flux-2-max \
  --prompt "minimal product photo of a ceramic bowl" \
  --param size=1024x1024
```

```bash
# Seedance / other reference-image video models
ai-media video generate \
  --model doubao-seedance-1-5-pro-251215 \
  --prompt "slow push-in over the food" \
  --param images='["https://example.com/reference.png"]' \
  --duration 4
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

Or:

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
