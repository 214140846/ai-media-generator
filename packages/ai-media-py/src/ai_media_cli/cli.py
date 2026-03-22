from __future__ import annotations

import subprocess
import sys

from .downloader import ensure_binary


def main(argv: list[str] | None = None) -> int:
    args = argv if argv is not None else sys.argv[1:]
    binary = ensure_binary()
    completed = subprocess.run([str(binary), *args], check=False)
    return completed.returncode
