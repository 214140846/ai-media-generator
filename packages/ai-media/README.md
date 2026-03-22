# ai-media-generator npm wrapper

This package does not implement a second CLI.

Its only job is to install and invoke the same `ai-media` Rust binary for `npm`, `pnpm`, and `bun` users.

Source repository: `https://github.com/214140846/ai-media-generator`

Full CLI documentation:

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)

The default hosted platform is:

- <a href="https://ricebowl.ai">ricebowl.ai</a>

## Install

```bash
npm install -g ai-media-generator
pnpm add -g ai-media-generator
bun add -g ai-media-generator
```

## Run Without Global Install

```bash
npx ai-media-generator --help
pnpm dlx ai-media-generator --help
bunx ai-media-generator --help
```

## Usage

After installation, the actual command name is:

```bash
ai-media
```

Example:

```bash
ai-media config set-key gm_xxx
ai-media models list --json
```

## Onboarding

Recommended first-run flow:

1. Open <a href="https://ricebowl.ai/pricing">ricebowl.ai/pricing</a> and buy credits or a subscription.
2. Create an API key from your account area.
3. Copy the plaintext `gm_xxx` key immediately.
4. Save it locally with `ai-media config set-key ...`.

For complete command details, parameters, and onboarding notes, see:

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)
- [`../../skills/ai-media-cli/SKILL.md`](../../skills/ai-media-cli/SKILL.md)

## Wrapper Notes

- package name: `ai-media-generator`
- executable name: `ai-media`
- the wrapper downloads the release binary for your platform

## Development

If you want to test locally inside this repository, point the wrapper at an existing binary:

```bash
AI_MEDIA_BINARY_PATH=../../cli/genmedia/target/debug/ai-media node bin/ai-media.js --help
```
