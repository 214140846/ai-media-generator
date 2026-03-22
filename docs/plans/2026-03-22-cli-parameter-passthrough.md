# CLI Parameter Passthrough Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Let `ai-media` pass model-specific generation parameters through the existing image/video commands, and teach the repo-local skills how to use those parameters.

**Architecture:** Keep the current `image generate` and `video generate` commands, add a repeatable `--param KEY=VALUE` passthrough layer that merges into the request body, and document model-family parameter templates in the CLI manual and skills. This avoids a command-tree explosion while still supporting model-specific fields from the upstream API docs.

**Tech Stack:** Rust, clap, reqwest, serde_json, Markdown

---

### Task 1: Add generic request parameter passthrough

**Files:**
- Modify: `cli/genmedia/src/main.rs`
- Modify: `cli/genmedia/src/commands/image.rs`
- Modify: `cli/genmedia/src/commands/video.rs`
- Add: `cli/genmedia/src/request_params.rs`
- Add: `cli/genmedia/src/request_params.test.rs` or inline module tests

**Step 1: Write the failing test**

- Add tests for parsing `KEY=VALUE`
- Add tests for merging extra params into a base request body

**Step 2: Run the tests to see them fail**

Run: `cargo test --manifest-path cli/genmedia/Cargo.toml`

**Step 3: Implement minimal passthrough**

- Parse JSON-looking values first, fall back to strings
- Reject duplicate reserved keys
- Merge extra params into image/video request bodies

**Step 4: Re-run tests**

Run: `cargo test --manifest-path cli/genmedia/Cargo.toml`

### Task 2: Document model-family parameters

**Files:**
- Modify: `cli/genmedia/README.md`
- Modify: `skills/ai-media-cli/SKILL.md`
- Modify: `skills/ai-media-cli/references/cli-commands.md`
- Modify: `skills/ai-image-generation/SKILL.md`
- Modify: `skills/ai-video-generation/SKILL.md`
- Modify: `skills/text-to-image/SKILL.md`
- Modify: `skills/text-to-video/SKILL.md`
- Modify: `skills/image-to-image/SKILL.md`
- Modify: `skills/image-to-video/SKILL.md`
- Modify: `skills/nano-banana-image-generator/SKILL.md`
- Modify: `skills/flux-image-generator/SKILL.md`
- Modify: `skills/veo-video-generator/SKILL.md`
- Modify: `skills/seedance-video-generator/SKILL.md`

**Step 1: Add passthrough guidance**

- Explain `--param KEY=VALUE`
- Show JSON array/object examples when needed

**Step 2: Add family templates**

- Veo: `aspect_ratio`, `enhance_prompt`, `enable_upsample`
- Image-to-video: `images`
- Flux/Recraft: `size`
- Generic fallback: consult `models list --json` and upstream docs

### Task 3: Verify

**Files:**
- Verify only

**Step 1: Run tests**

Run:

```bash
cargo test --manifest-path cli/genmedia/Cargo.toml
cargo run --manifest-path cli/genmedia/Cargo.toml -- --help
cargo run --manifest-path cli/genmedia/Cargo.toml -- image generate --help
cargo run --manifest-path cli/genmedia/Cargo.toml -- video generate --help
```

---

## Status

**Current state:** Task 1 and Task 2 are complete for the Rust CLI and repo-local skills/docs, and the model metadata browser flow is now documented as well.

**Completed:** Added `--param KEY=VALUE` passthrough to `image generate` and `video generate`, added unit coverage for parsing/merging behavior, documented `models show --model <MODEL>` as the parameter metadata lookup step, and synced the CLI manual plus skill guidance with family-level parameter templates.

**Validation:** `cargo test --manifest-path cli/genmedia/Cargo.toml`; `cargo run --manifest-path cli/genmedia/Cargo.toml -- --help`; `cargo run --manifest-path cli/genmedia/Cargo.toml -- image generate --help`; `cargo run --manifest-path cli/genmedia/Cargo.toml -- video generate --help`; `cargo run --manifest-path cli/genmedia/Cargo.toml -- models --help`; `cargo run --manifest-path cli/genmedia/Cargo.toml -- models show --help`

**Next exact step:** If we want more coverage, add a lightweight `models list --json` fixture or a CLI integration test for one image and one video body shape using a real API key.
