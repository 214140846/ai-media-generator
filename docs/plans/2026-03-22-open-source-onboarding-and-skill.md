# Open Source Onboarding And Skill Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make `ai-media-generator` feel open-source ready, add onboarding-friendly hosted-platform docs, and create a reusable repo-local skill for the CLI.

**Architecture:** Keep the repository root README as the open-source entry point, use the full CLI manual as the canonical command and onboarding guide, and place reusable Codex-facing guidance under `skills/ai-media-cli/` with slim core instructions plus reference files.

**Tech Stack:** Markdown, Rust CLI help output, repo-local skills, MIT licensing

---

### Task 1: Refresh hosted-platform messaging in READMEs

**Files:**
- Modify: `README.md`
- Modify: `cli/genmedia/README.md`
- Modify: `packages/ai-media/README.md`
- Modify: `packages/ai-media-py/README.md`

**Step 1: Add hosted-platform references**

- Mention <a href="https://sora2.cloud">sora2.cloud</a>
- Mention <a href="https://ricebowl.ai">ricebowl.ai</a>
- Use HTML `a` tags where requested

**Step 2: Add onboarding guidance**

- Explain recharge path
- Explain API key creation path
- Explain CLI configuration

### Task 2: Add repo-local skill

**Files:**
- Create: `skills/ai-media-cli/SKILL.md`
- Create: `skills/ai-media-cli/references/cli-commands.md`
- Create: `skills/ai-media-cli/references/platform-onboarding.md`

**Step 1: Keep the main skill compact**

- Trigger conditions
- Workflow order
- Reference loading guidance

**Step 2: Put detail into references**

- Full parameters and defaults
- Common command recipes
- Recharge and key-generation onboarding

### Task 3: Add repository-level open-source marker

**Files:**
- Create: `LICENSE`

**Step 1: Align with package metadata**

- Use MIT at repository root so repo-level licensing matches Cargo / npm / PyPI metadata

### Task 4: Verify docs against reality

**Files:**
- Verify only

**Step 1: Check CLI help**

Run:

```bash
cargo run --manifest-path cli/genmedia/Cargo.toml -- --help
cargo run --manifest-path cli/genmedia/Cargo.toml -- image generate --help
cargo run --manifest-path cli/genmedia/Cargo.toml -- video generate --help
cargo run --manifest-path cli/genmedia/Cargo.toml -- task get --help
```

**Step 2: Check final diff**

Run:

```bash
git diff -- README.md cli/genmedia/README.md packages/ai-media/README.md packages/ai-media-py/README.md LICENSE skills docs/plans
```
