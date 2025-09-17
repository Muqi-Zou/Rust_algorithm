#!/usr/bin/env python3
"""Generate Cargo binary wrappers for every source file in `src`."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC_DIR = ROOT / "src"
BIN_DIR = SRC_DIR / "bin"
PREFIX = "algo__"
MOD_DECL_RE = re.compile(r"mod\s+([A-Za-z0-9_]+)")

def module_path(rel: Path) -> list[str]:
    parts = list(rel.parts)
    if parts[-1] == "lib.rs":
        return []
    if parts[-1] == "mod.rs":
        return parts[:-1]
    stem = parts[-1][:-3]
    return parts[:-1] + [stem]

def is_cfg_test_module(rel: Path) -> bool:
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
        match = MOD_DECL_RE.search(stripped)
        if match:
            name = match.group(1).rstrip(';')
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

        module_parts = module_path(rel)
        module_use = "::".join(["crate", *module_parts]) if module_parts else None
        cfg_test = is_cfg_test_module(rel)

        if rel.name == "lib.rs":
            bin_name = PREFIX + "lib"
        else:
            parts = list(rel.parts)
            parts[-1] = parts[-1].replace(".rs", "")
            parts = [p.replace("-", "_") for p in parts]
            bin_name = PREFIX + "__".join(parts)
        dest = BIN_DIR / f"{bin_name}.rs"

        lines = [
            "#![allow(warnings)]",
            "#![allow(clippy::all)]",
            "",
            "include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/lib.rs\"));",
            "",
        ]

        if cfg_test and rel.name != "lib.rs":
            include_path = "../" + rel.as_posix()
            lines.append(f"#[path = \"{include_path}\"]")
            lines.append("mod source;")
            lines.append("")
            module_use = None
        elif module_use is not None:
            lines.append("#[allow(unused_imports)]")
            lines.append(f"use {module_use} as source;")
            lines.append("")

        module_filter_parts = module_parts + ["tests"]
        filter_arg = "::".join(module_filter_parts) + "::"

        lines.extend(
            [
                "fn main() {",
                "    let manifest_dir = env!(\"CARGO_MANIFEST_DIR\");",
                "    let cargo = std::env::var(\"CARGO\").unwrap_or_else(|_| \"cargo\".to_string());",
                "    let mut cargo_args = Vec::new();",
                "    let mut test_args = Vec::new();",
                "    let mut after_delimiter = false;",
                "    for arg in std::env::args().skip(1) {",
                "        if !after_delimiter && arg == \"--\" {",
                "            after_delimiter = true;",
                "            continue;",
                "        }",
                "        if after_delimiter {",
                "            test_args.push(arg);",
                "        } else {",
                "            cargo_args.push(arg);",
                "        }",
                "    }",
                "    let mut command = std::process::Command::new(cargo);",
                "    command.current_dir(manifest_dir);",
                "    command.arg(\"test\");",
                f"    command.arg(\"{filter_arg}\");",
                "    if !cargo_args.is_empty() {",
                "        command.args(cargo_args);",
                "    }",
                "    if !test_args.is_empty() {",
                "        command.arg(\"--\");",
                "        command.args(test_args);",
                "    }",
                "    match command.status() {",
                "        Ok(status) if status.success() => {}",
                "        Ok(status) => std::process::exit(status.code().unwrap_or(1)),",
                "        Err(err) => {",
                "            eprintln!(\"failed to run cargo test: {err}\");",
                "            std::process::exit(1);",
                "        }",
                "    }",
                "}",
            ]
        )

        dest.write_text("\n".join(lines) + "\n", encoding="utf-8")

if __name__ == "__main__":
    main()
