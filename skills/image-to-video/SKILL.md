---
name: image-to-video
description: Use when someone asks for image-to-video, animate image, convert image to video, reference-image video generation, or CLI-based image-to-video workflows on ricebowl.ai.
---

# Image To Video

用这个 skill 处理这些请求：

- `image to video`
- `image-to-video`
- `animate image`
- `convert image to video`
- `reference image video`
- `ai image to video generator`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> upload or prepare image URL
  -> models list --json
  -> choose an image-to-video model
  -> video generate
  -> task get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media video generate \
  --model <MODEL> \
  --prompt "<subject moving from the reference image>, <camera movement>, <mood>" \
  --aspect-ratio 16:9 \
  --duration 4 \
  --wait
```

先把参考图准备好，再把同一条 prompt 套进你选中的 image-to-video 模型。

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
