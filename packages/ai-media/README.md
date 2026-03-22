# ai-media-generator npm wrapper

这个包不重新实现 CLI 逻辑，只负责为 `npm` / `pnpm` / `bun` 用户安装 `ai-media` Rust 二进制。

源码仓库：`https://github.com/214140846/ai-media-generator`

## Install

```bash
npm install -g ai-media-generator
pnpm add -g ai-media-generator
bun add -g ai-media-generator
```

## Usage

```bash
ai-media --help
pnpm dlx ai-media-generator --help
bunx ai-media-generator --help
```

## Development

如果你在本仓库里本地联调，可以直接指定已有二进制：

```bash
AI_MEDIA_BINARY_PATH=../../cli/genmedia/target/debug/ai-media node bin/ai-media.js --help
```
