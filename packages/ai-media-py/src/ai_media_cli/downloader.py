from __future__ import annotations

import os
import pathlib
import stat
import urllib.request

from .platforms import build_release_url, resolve_binary_name, resolve_target

VERSION = "0.1.1"


def resolve_cache_dir() -> pathlib.Path:
    xdg = os.environ.get("XDG_CACHE_HOME")
    if xdg:
        return pathlib.Path(xdg) / "ai-media"
    return pathlib.Path.home() / ".cache" / "ai-media"


def ensure_binary() -> pathlib.Path:
    explicit = os.environ.get("AI_MEDIA_BINARY_PATH")
    if explicit:
        return pathlib.Path(explicit).expanduser().resolve()

    target = resolve_target()
    cache_dir = resolve_cache_dir() / VERSION / target
    binary_path = cache_dir / resolve_binary_name()
    if binary_path.exists():
        return binary_path

    cache_dir.mkdir(parents=True, exist_ok=True)
    url = build_release_url(VERSION, target)
    urllib.request.urlretrieve(url, binary_path)
    current_mode = binary_path.stat().st_mode
    binary_path.chmod(current_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return binary_path
