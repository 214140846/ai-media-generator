---
name: veo-video-generator
description: Use when someone asks for veo video generator, veo text-to-video, veo prompt examples, veo model selection, or CLI-based video generation with veo-style models on ricebowl.ai.
---

# Veo Video Generator

用这个 skill 处理这些请求：

- `veo video generator`
- `veo text to video`
- `veo prompt examples`
- `veo model`
- `google veo generator`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> models show --model <VEO_MODEL>
  -> choose a veo-capable video model
  -> video generate
  -> task get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media models show --model <VEO_MODEL>
ai-media video generate \
  --model <VEO_MODEL> \
  --prompt "<scene>, <camera movement>, <mood>" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --param enhance_prompt=true \
  --wait
```

如果模型支持更长成片，再把 `--duration` 往上调。
如果是图生视频，补 `--image https://...`。

## Core Commands

```bash
ai-media config set-key <KEY>
ai-media config show
ai-media models list --json
ai-media models show --model <MODEL>
ai-media video generate --model <MODEL> --prompt <PROMPT>
ai-media task get --kind video --task-id <TASK_ID>
```

## When To Load References

- 参数全表、默认值、输出行为：读 `../ai-media-cli/references/cli-commands.md`
- 平台充值、生成 key、平台 onboarding：读 `../ai-media-cli/references/platform-onboarding.md`
