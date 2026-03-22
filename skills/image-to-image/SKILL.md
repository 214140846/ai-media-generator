---
name: image-to-image
description: Use when someone asks for image-to-image, reference-image generation, transform image with prompt, edit image with AI, or CLI-based image-to-image workflows on ricebowl.ai.
---

# Image To Image

用这个 skill 处理这些请求：

- `image to image`
- `image-to-image`
- `reference image generation`
- `transform image with prompt`
- `edit image with ai`
- `ai image to image generator`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> upload or prepare reference image URL
  -> models list --json
  -> choose an image-to-image model
  -> image generate
  -> image get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media image generate \
  --model <MODEL> \
  --prompt "<edit instruction>, <style>, <lighting>" \
  --aspect-ratio 1:1 \
  --wait
```

先把参考图准备好，再让 prompt 只描述你要怎么改它。

## Core Commands

```bash
ai-media config set-key <KEY>
ai-media config show
ai-media models list --json
ai-media image generate --model <MODEL> --prompt <PROMPT>
ai-media image get --task-id <TASK_ID>
```

## When To Load References

- 参数全表、默认值、输出行为：读 `../ai-media-cli/references/cli-commands.md`
- 平台充值、生成 key、平台 onboarding：读 `../ai-media-cli/references/platform-onboarding.md`
