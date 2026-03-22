---
name: text-to-image
description: Use when someone asks for text-to-image, prompt-to-image, generate image from text, ai image prompt generation, or CLI-based text-to-image workflows on ricebowl.ai.
---

# Text To Image

用这个 skill 处理这些请求：

- `text to image`
- `text-to-image`
- `prompt to image`
- `generate image from text`
- `ai image prompt`
- `ai text to image generator`

## Default Route

```text
ricebowl.ai
  -> recharge credits
  -> create API key
  -> set key
  -> models list --json
  -> choose a text-to-image model
  -> image generate
  -> image get
```

## Recommended Template

```bash
ai-media config set-key <KEY>
ai-media models list --json
ai-media image generate \
  --model <MODEL> \
  --prompt "<subject>, <style>, <lighting>" \
  --aspect-ratio 1:1 \
  --param size=1024x1024 \
  --wait
```

如果要做横幅图，就把 `--aspect-ratio` 改成 `16:9`。
如果当前模型要求别的图片参数，用 `--param KEY=VALUE` 补上。

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
