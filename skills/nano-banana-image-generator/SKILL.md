---
name: nano-banana-image-generator
description: Use when someone asks for nano banana image generator, nano-banana text-to-image, nano-banana prompt examples, nano-banana model selection, or CLI-based image generation with nano-banana-style models on ricebowl.ai.
---

# Nano Banana Image Generator

用这个 skill 处理这些请求：

- `nano banana image generator`
- `nano-banana image generator`
- `nano banana text to image`
- `nano banana prompt examples`
- `nano banana model`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> models show --model nano-banana
  -> choose a nano-banana-capable image model
  -> image generate
  -> image get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media models show --model nano-banana
ai-media image generate \
  --model nano-banana \
  --prompt "<subject>, <style>, <lighting>" \
  --aspect-ratio 1:1 \
  --image https://example.com/reference.png \
  --param vendor_options='{"style":"cinematic"}' \
  --wait
```

如果是社媒封面，就把 `--aspect-ratio` 改成 `16:9`。
如果这个模型版本暴露了额外参数，就先跑 `models show --model nano-banana`，再用 `--param KEY=VALUE`。

## Core Commands

```bash
ai-media config set-key <KEY>
ai-media config show
ai-media models list --json
ai-media models show --model <MODEL>
ai-media image generate --model <MODEL> --prompt <PROMPT>
ai-media image get --task-id <TASK_ID>
```

## When To Load References

- 参数全表、默认值、输出行为：读 `../ai-media-cli/references/cli-commands.md`
- 平台充值、生成 key、平台 onboarding：读 `../ai-media-cli/references/platform-onboarding.md`
