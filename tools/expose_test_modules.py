#!/usr/bin/env python3
"""Ensure all `#[cfg(test)] mod tests` modules are `pub(crate)`."""

from __future__ import annotations

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SRC_DIR = ROOT / "src"


def rewrite_tests_module(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    updated = text
    replacements = [
        ("#[cfg(test)]\nmod tests {", "#[cfg(any(test, feature = \"bin-tests\"))]\npub(crate) mod tests {"),
        ("#[cfg(test)] mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))] pub(crate) mod tests {"),
        ("#[cfg(test)]\npub(crate) mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))]\npub(crate) mod tests {"),
        ("#[cfg(test)] pub(crate) mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))] pub(crate) mod tests {"),
        ("#[cfg(test)]\npub mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))]\npub mod tests {"),
        ("#[cfg(test)] pub mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))] pub mod tests {"),
        ("#[cfg(any(test, feature = \"bin-tests\"))]\nmod tests {", "#[cfg(any(test, feature = \"bin-tests\"))]\npub(crate) mod tests {"),
        ("#[cfg(any(test, feature = \"bin-tests\"))] mod tests {", "#[cfg(any(test, feature = \"bin-tests\"))] pub(crate) mod tests {"),
    ]
    for old, new in replacements:
        updated = updated.replace(old, new)
    updated = updated.replace(
        "#[cfg(test)]", "#[cfg(any(test, feature = \"bin-tests\"))]"
    )
    if updated != text:
        path.write_text(updated, encoding="utf-8")
        return True
    return False


def main() -> None:
    for source in SRC_DIR.rglob("*.rs"):
        if "bin" in source.relative_to(SRC_DIR).parts:
            continue
        rewrite_tests_module(source)


if __name__ == "__main__":
    main()
