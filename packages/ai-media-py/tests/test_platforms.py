import unittest

from ai_media_cli.platforms import (
    build_asset_name,
    build_release_url,
    resolve_binary_name,
    resolve_target,
)


class PlatformTests(unittest.TestCase):
    def test_resolve_target(self) -> None:
        self.assertEqual(resolve_target("Darwin", "arm64"), "aarch64-apple-darwin")
        self.assertEqual(resolve_target("Linux", "x86_64"), "x86_64-unknown-linux-gnu")
        self.assertEqual(resolve_target("Windows", "AMD64"), "x86_64-pc-windows-msvc")

    def test_resolve_binary_name(self) -> None:
        self.assertEqual(resolve_binary_name("Linux"), "ai-media")
        self.assertEqual(resolve_binary_name("Windows"), "ai-media.exe")

    def test_release_url(self) -> None:
        target = "x86_64-unknown-linux-gnu"
        self.assertEqual(build_asset_name(target), "ai-media-generator-x86_64-unknown-linux-gnu")
        self.assertEqual(
            build_release_url("0.1.1", target),
            "https://github.com/214140846/ai-media-generator/releases/download/ai-media-generator-v0.1.1/ai-media-generator-x86_64-unknown-linux-gnu",
        )


if __name__ == "__main__":
    unittest.main()
