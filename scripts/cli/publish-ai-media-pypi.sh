#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
PKG_DIR="$ROOT_DIR/packages/ai-media-py"

EXPECTED_VERSION="$(node -e "const fs=require('node:fs');const raw=fs.readFileSync('$ROOT_DIR/cli/genmedia/Cargo.toml','utf8');const match=raw.match(/^version\\s*=\\s*\"([^\"]+)\"/m);if(!match) process.exit(1);process.stdout.write(match[1]);")"
ACTUAL_VERSION="$(python3 - <<'PY'
from pathlib import Path
import re

raw = Path("packages/ai-media-py/pyproject.toml").read_text()
match = re.search(r'^version\s*=\s*"([^"]+)"', raw, re.M)
if not match:
    raise SystemExit(1)
print(match.group(1), end="")
PY
)"

if [[ "$EXPECTED_VERSION" != "$ACTUAL_VERSION" ]]; then
  echo "ai-media-py version mismatch: expected $EXPECTED_VERSION, got $ACTUAL_VERSION" >&2
  exit 1
fi

python3 -m pip install --upgrade build twine
cd "$PKG_DIR"
python3 -m build
python3 -m twine upload dist/*
