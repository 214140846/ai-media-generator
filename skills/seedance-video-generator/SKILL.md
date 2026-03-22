---
name: seedance-video-generator
description: Use when someone asks for seedance video generator, seedance text-to-video, seedance image-to-video, seedance model selection, or CLI-based video generation with seedance-style models on ricebowl.ai.
---

# Seedance Video Generator

用这个 skill 处理这些请求：

- `seedance video generator`
- `seedance text to video`
- `seedance image to video`
- `seedance model`
- `seedance pro fast`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> models list --json
  -> choose a seedance-capable video model
  -> video generate
  -> task get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "<scene>, <camera movement>, <mood>" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --param images='["https://example.com/reference.png"]' \
  --wait
```

这个模板先跑最短成片；如果要更长，再增加 `--duration`。
Seedance 的图生视频参数也走 `--param KEY=VALUE`。

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
