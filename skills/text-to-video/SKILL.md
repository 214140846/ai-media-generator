---
name: text-to-video
description: Use when someone asks for text-to-video, prompt-to-video, generate video from text, cinematic video prompt generation, or CLI-based text-to-video workflows on ricebowl.ai.
---

# Text To Video

用这个 skill 处理这些请求：

- `text to video`
- `text-to-video`
- `prompt to video`
- `generate video from text`
- `cinematic video prompt`
- `ai text to video generator`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> models list --json
  -> choose a text-to-video model
  -> video generate
  -> task get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media video generate \
  --model <MODEL> \
  --prompt "<scene>, <camera movement>, <mood>" \
  --aspect-ratio 16:9 \
  --duration 4 \
  --param enhance_prompt=true \
  --wait
```

如果模型是图生视频，就把参考图放进 `--param images='["https://..."]'`。

## Core Commands

```bash
ai-media config set-key <KEY>
ai-media config show
ai-media models list --json
ai-media video generate --model <MODEL> --prompt <PROMPT>
ai-media task get --kind video --task-id <TASK_ID>
```

## When To Load References

- 参数全表、默认值、输出行为：读 `../ai-media-cli/references/cli-commands.md`
- 平台充值、生成 key、平台 onboarding：读 `../ai-media-cli/references/platform-onboarding.md`
