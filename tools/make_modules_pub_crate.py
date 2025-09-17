#!/usr/bin/env python3
"""Update internal module declarations in mod.rs files to `pub(crate)` visibility."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC_DIR = ROOT / "src"

MOD_PATTERN = re.compile(r"^(\s*)mod(\s+)")


def process_file(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    changed = False

    for idx, line in enumerate(lines):
        stripped = line.lstrip()
        if not stripped:
            continue
        if stripped.startswith("//") or stripped.startswith("/*"):
            continue
        match = MOD_PATTERN.match(line)
        if match is None:
            continue
        prefix = line[: match.end()]
        if prefix.lstrip().startswith("pub"):
            continue
        lines[idx] = f"{match.group(1)}pub(crate) mod{match.group(2)}" + line[match.end():]
        changed = True

    if changed:
        path.write_text("\n".join(lines) + ("\n" if text.endswith("\n") else ""), encoding="utf-8")


def main() -> None:
    for path in SRC_DIR.rglob("mod.rs"):
        if "bin" in path.parts:
            continue
        process_file(path)

if __name__ == "__main__":
    main()
