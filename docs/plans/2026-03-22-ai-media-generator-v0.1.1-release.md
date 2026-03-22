# ai-media-generator v0.1.1 Release Draft

## Summary

This patch release is mainly documentation and onboarding polish.

## Highlights

- README now defaults to `ricebowl.ai`
- Skill discovery is split into image, video, and long-tail search intents
- Model-specific skills now include copyable parameter templates
- Existing CLI behavior is unchanged

## Notes

- No public publish has been run yet
- The release still needs the normal version bump and publish flow for Cargo, npm, and PyPI
- After tagging, GitHub Releases assets should match `ai-media-generator-v0.1.1`

## Validation

- `cargo test --manifest-path cli/genmedia/Cargo.toml`
- `node --test packages/ai-media/scripts/lib/platform.test.js`
- `PYTHONPATH=packages/ai-media-py/src python3 -m unittest packages/ai-media-py/tests/test_platforms.py`

## Residual

- `cli/genmedia/Cargo.lock` still contains an unrelated dependency version `0.1.0` for `pin-utils`
- There are no remaining release/version references for `ai-media-generator-v0.1.0`
