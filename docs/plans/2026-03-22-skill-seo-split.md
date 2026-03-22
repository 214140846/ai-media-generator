# Skill SEO Split Plan

## Goal

把当前单一的 `ai-media-cli` skill 拆成更贴近用户搜索意图的 skill 入口，默认平台固定为 `ricebowl.ai`，并给每个模型专属 skill 补参数模板。

## Scope

- 保留原有 CLI 能力与共享 reference 文档
- 新增按搜索意图拆分的 skill 目录
- 更新仓库 README 的安装入口与 skill 展示
- 去掉用户手动设置 `base_url` 的路径
- 给模型专属 skill 加上可复制的参数模板

## Non-Goals

- 不修改 Rust CLI 行为
- 不新增空壳 skill 只做跳转
- 不一次性铺太多低价值长尾词

## Constraints

- skill 名称要短、像用户会搜的词
- skill 内容要复用已有 reference，避免维护两套命令文档
- 每个 skill 必须有独立触发词和默认响应模式

## Old vs New

```text
before
  ai-media-cli
    -> image
    -> video
    -> onboarding

after
  ai-image-generation
    -> image generation intent
    -> image model / prompt / history intent

  ai-video-generation
    -> video generation intent
    -> text-to-video / image-to-video / model intent

  sora2-video-generator
    -> legacy long-tail intent

shared references
  -> cli-commands.md
  -> platform-onboarding.md
```

## Files

- Create: `skills/ai-image-generation/SKILL.md`
- Create: `skills/ai-video-generation/SKILL.md`
- Create: `skills/sora2-video-generator/SKILL.md`
- Keep: `skills/ai-media-cli/references/cli-commands.md`
- Keep: `skills/ai-media-cli/references/platform-onboarding.md`
- Modify: `README.md`

## Acceptance

- `skills/` 下至少出现 3 个搜索意图明确的 skill
- 每个 skill 的 `description` 和触发词覆盖核心搜索短语
- README 能直接展示新的 skill 安装入口
- skill 文案明确指向共享 reference，避免重复维护
- 用户路径里不再需要手动设置 `base_url`
- 模型专属 skill 里都有可直接复制的参数模板

## Implemented

- 保留 `ai-media-cli` 作为通用 CLI onboarding skill
- 默认平台收口为 `ricebowl.ai`
- 新增 `ai-image-generation`
- 新增 `ai-video-generation`
- 新增 `sora2-video-generator`
- 新增 `text-to-video`
- 新增 `image-to-video`
- 新增 `text-to-image`
- 新增 `image-to-image`
- 新增 `flux-image-generator`
- 新增 `nano-banana-image-generator`
- 新增 `veo-video-generator`
- 新增 `seedance-video-generator`
- README 补了新的 `skills add` 命令和 `skills.sh` 卡片入口，并去掉了多平台 onboarding 主路径
- `sora2-video-generator` 保留为 legacy skill，但不再出现在默认推荐安装入口里

## Validation

运行并确认：

```bash
find ai-media-generator/skills -maxdepth 2 -name SKILL.md | sort
rg -n "^(name|description):|ai-image-generation|ai-video-generation|sora2-video-generator|text-to-video|image-to-video|text-to-image|image-to-image|flux-image-generator|nano-banana-image-generator|veo-video-generator|seedance-video-generator|npx skills add" ai-media-generator/skills ai-media-generator/README.md
git -C ai-media-generator diff -- README.md skills docs/plans
```

## Reference Cleanup

- `skills/ai-media-cli/references/cli-commands.md` 现在默认 `ricebowl.ai`，`set-base-url` 只保留历史兼容说明
- `skills/ai-media-cli/references/platform-onboarding.md` 现在只保留 `ricebowl.ai` 的 onboarding，`sora2.cloud` 路径已移除

## Next Recommended Step

如果要继续吃长尾流量，下一批优先补：

- `ai-photo-editor`
- `image-upscaler`
- `video-upscaler`
- `anime-image-generator`

## Residual Risk

当前默认路径已经统一到 `ricebowl.ai`。仅剩的 `set-base-url` / `AI_MEDIA_BASE_URL` / `sora2` 都属于历史兼容说明，不再出现在默认用户路径里。
