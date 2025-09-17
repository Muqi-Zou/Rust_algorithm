#!/usr/bin/env python3
"""Generate Cargo binary wrappers for every source file in `src`."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC_DIR = ROOT / "src"
BIN_DIR = SRC_DIR / "bin"
PREFIX = "algo__"
CRATE_NAME = "the_algorithms_rust"
MOD_DECL_RE = re.compile(r"(?:pub\s+)?mod\s+([A-Za-z0-9_]+)\s*;")


def module_path(rel: Path) -> list[str]:
    parts = list(rel.parts)
    if parts[-1] == "lib.rs":
        return []
    if parts[-1] == "mod.rs":
        return parts[:-1]
    stem = parts[-1][:-3]
    return parts[:-1] + [stem]


def is_cfg_test_module(rel: Path) -> bool:
    """Return True when `rel` is declared under `#[cfg(test)] mod ...]`."""
    if rel.name in {"lib.rs", "mod.rs"}:
        return False
    if len(rel.parts) < 2:
        return False
    parent_mod = SRC_DIR.joinpath(*rel.parts[:-1], "mod.rs")
    if not parent_mod.exists():
        return False
    target = rel.stem
    previous_cfg_test = False
    for line in parent_mod.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped.startswith("#[cfg(test)]"):
            previous_cfg_test = True
            continue
        if stripped.startswith("#[") and stripped.endswith("]"):
            continue
        match = MOD_DECL_RE.match(stripped)
        if match:
            name = match.group(1)
            if name == target:
                return previous_cfg_test
            previous_cfg_test = False
        else:
            previous_cfg_test = False
    return False


def main() -> None:
    BIN_DIR.mkdir(exist_ok=True)

    for path in BIN_DIR.glob(f"{PREFIX}*.rs"):
        path.unlink()

    for source in SRC_DIR.rglob("*.rs"):
        rel = source.relative_to(SRC_DIR)
        if rel.parts and rel.parts[0] == "bin":
            continue
        if is_cfg_test_module(rel):
            continue

        module_parts = module_path(rel)

        if rel.name == "lib.rs":
            bin_name = PREFIX + "lib"
        else:
            parts = list(rel.parts)
            parts[-1] = parts[-1].removesuffix(".rs")
            parts = [p.replace("-", "_") for p in parts]
            bin_name = PREFIX + "__".join(parts)
        dest = BIN_DIR / f"{bin_name}.rs"

        lines: list[str] = [
            "#![allow(warnings)]",
            "#![allow(clippy::all)]",
            "",
        ]

        if not module_parts:
            lines.append(f"pub use {CRATE_NAME} as source;")
        else:
            module_path_str = "::".join([CRATE_NAME, *module_parts])
            lines.append(f"pub use {module_path_str} as source;")

        lines.append("pub use source::*;")

        lines.extend([
            "",
            "fn main() {",
            "    let _ = core::hint::black_box(());",
            "}",
        ])

        dest.write_text("\n".join(lines) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
