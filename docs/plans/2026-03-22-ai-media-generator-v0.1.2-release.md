# ai-media-generator v0.1.2 Release Draft

## Summary

This patch release adds model-parameter passthrough for the CLI and aligns web manifest guidance with the local skills.

## Highlights

- `ai-media models show --model <MODEL>` now prints per-model parameter guidance
- `ai-media image/video generate` now accepts generic `--param key=value`
- Image generation supports `--image`, `--response-format`, and `--metadata-json`
- Skills and README examples now teach the new parameter flow

## Notes

- Web manifest is already deployed on `ricebowl.ai`
- GitHub Releases assets for this tag must match `ai-media-generator-v0.1.2`
- npm, PyPI, and crates.io should all publish as `0.1.2`

## Validation

- `cargo test --manifest-path cli/genmedia/Cargo.toml`
- `node --test packages/ai-media/scripts/lib/platform.test.js`
- `PYTHONPATH=packages/ai-media-py/src python3 -m unittest packages/ai-media-py/tests/test_platforms.py`

## Residual

- `ricebowl.ai` is deployed separately from this package release
- If release assets are missing on GitHub, npm/PyPI installs will fail at binary download time
