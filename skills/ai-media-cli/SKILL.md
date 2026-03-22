---
name: ai-media-cli
description: Use when someone wants to install, configure, onboard, or automate the ai-media CLI against ricebowl.ai, sora2.cloud, or a compatible local API. Covers install paths, credits recharge flow, API key setup, command parameters, common workflows, and script-friendly usage.
---

# ai-media CLI

用这个 skill 处理这些请求：

- 安装 `ai-media-generator`
- 配置 `ai-media` CLI
- 对接 <a href="https://ricebowl.ai">ricebowl.ai</a> 或 <a href="https://sora2.cloud">sora2.cloud</a>
- 讲命令参数、常用法、脚本化调用
- onboarding 新用户，包括充值、生成 key、写入配置

## Quick Start

先判断用户在哪个平台：

- `ricebowl.ai`
- `sora2.cloud`
- 本地兼容 API

然后按这个顺序带用户走：

```text
install
  -> recharge credits
  -> create API key
  -> set base_url
  -> set key
  -> config show
  -> models list --json
  -> run first image/video task
```

## Platform Mapping

- `ricebowl.ai` -> `https://ricebowl.ai`
- `sora2.cloud` -> `https://sora2.cloud`
- local dev -> `http://127.0.0.1:3000`

## What To Explain By Default

默认要讲清楚这 4 件事：

1. 安装包名是 `ai-media-generator`
2. 实际命令名是 `ai-media`
3. 核心配置只有 `base_url` 和 `api_key`
4. 环境变量 `AI_MEDIA_BASE_URL` / `AI_MEDIA_API_KEY` 会覆盖本地配置

## Core Commands

常用命令：

```bash
ai-media config set-base-url <BASE_URL>
ai-media config set-key <KEY>
ai-media config show
ai-media models list
ai-media models list --json
ai-media image generate --model <MODEL> --prompt <PROMPT>
ai-media image get --task-id <TASK_ID>
ai-media video generate --model <MODEL> --prompt <PROMPT>
ai-media video get --task-id <TASK_ID>
ai-media task get --kind <image|video> --task-id <TASK_ID>
```

## When To Load References

- 参数全表、默认值、输出行为：读 `references/cli-commands.md`
- 充值、生成 key、平台 onboarding：读 `references/platform-onboarding.md`

## Response Pattern

如果用户是第一次接触这个 CLI，优先给：

1. 最短安装命令
2. 平台充值路径
3. 生成 key 的位置
4. 一组可直接复制的配置命令
5. 一条图片命令或视频命令

如果用户要自动化或脚本集成，补充：

- `models list --json`
- `image/video/task get`
- `--wait`
- `--poll-interval`
- 环境变量覆盖方式

## Friendly Onboarding Snippet

```bash
ai-media config set-base-url https://ricebowl.ai
ai-media config set-key gm_xxx
ai-media config show
ai-media models list --json
```

如果是 `sora2.cloud`，只把 `set-base-url` 改成：

```bash
ai-media config set-base-url https://sora2.cloud
```
