from __future__ import annotations

import platform

SUPPORTED_TARGETS = {
    ("Darwin", "arm64"): "aarch64-apple-darwin",
    ("Darwin", "x86_64"): "x86_64-apple-darwin",
    ("Linux", "x86_64"): "x86_64-unknown-linux-gnu",
    ("Windows", "AMD64"): "x86_64-pc-windows-msvc",
}


def resolve_target(system: str | None = None, machine: str | None = None) -> str:
    system_name = system or platform.system()
    machine_name = machine or platform.machine()
    key = (system_name, machine_name)
    if key not in SUPPORTED_TARGETS:
        raise RuntimeError(f"Unsupported platform {system_name}:{machine_name} for ai-media")
    return SUPPORTED_TARGETS[key]


def resolve_binary_name(system: str | None = None) -> str:
    system_name = system or platform.system()
    return "ai-media.exe" if system_name == "Windows" else "ai-media"


def build_asset_name(target: str) -> str:
    if "windows" in target:
        return f"ai-media-generator-{target}.exe"
    return f"ai-media-generator-{target}"


def build_release_url(version: str, target: str) -> str:
    asset = build_asset_name(target)
    return (
        f"https://github.com/214140846/ai-media-generator/releases/download/"
        f"ai-media-generator-v{version}/{asset}"
    )
