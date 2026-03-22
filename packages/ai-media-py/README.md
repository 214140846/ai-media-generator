# ai-media-generator

This package is the PyPI thin wrapper for the `ai-media` Rust CLI.

It does not implement a separate Python CLI. It installs and invokes the same `ai-media` binary.

Source repository: `https://github.com/214140846/ai-media-generator`

Full CLI documentation:

- [`../../cli/genmedia/README.md`](../../cli/genmedia/README.md)

The default hosted platform is:

- <a href="https://ricebowl.ai">ricebowl.ai</a>

## Install

```bash
pipx install ai-media-generator
uv tool install ai-media-generator
```

## Run Without Persistent Install

```bash
uvx ai-media-generator --help
python -m ai_media_cli --help
```

## Usage

After installation, the actual command name is:

```bash
ai-media
```

Example:

```bash
ai-media config set-key gm_xxx
ai-media video generate \
  --model seedance-pro-fast \
  --prompt "steam rising from a rice bowl, cinematic close-up" \
  --aspect-ratio 16:9 \
  --duration 2 \
  --wait
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
