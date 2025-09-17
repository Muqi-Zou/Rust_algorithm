#!/usr/bin/env python3
"""Utility to mirror all source modules into Cargo examples and build them.

The workflow is tailored for reverse-engineering use-cases where each source
file under ``src/`` must be compiled as an individual binary without relying on
the ``cargo test`` harness that generates temporary wrappers.

Steps performed:
1. Recreate an ``examples/src`` directory that contains a verbatim copy of the
   crate's ``src`` tree.
2. Emit a thin example wrapper for every copied file so that ``cargo test
   --examples`` produces a binary containing that module's implementation.
3. Ensure the Cargo manifest disables optimisation in the dev profile which
   keeps inlining off when the examples are compiled.
4. Execute ``cargo test --examples`` and rename the produced binaries under
   ``target/debug/deps`` so their filenames mirror the original ``src`` paths.
"""

from __future__ import annotations

import re
import shutil
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SRC_DIR = ROOT / "src"
EXAMPLES_DIR = ROOT / "examples"
EXAMPLES_SRC_DIR = EXAMPLES_DIR / "src"
CARGO_TOML = ROOT / "Cargo.toml"
TARGET_DEPS = ROOT / "target" / "debug" / "deps"


def copy_source_tree() -> dict[str, Path]:
    """Copy every ``.rs`` file from ``src`` into ``examples/src``.

    Returns a mapping from the sanitised example name to its relative source
    path which is reused when renaming build artefacts.
    """

    if EXAMPLES_DIR.exists():
        shutil.rmtree(EXAMPLES_DIR)

    mapping: dict[str, Path] = {}

    for path in SRC_DIR.rglob("*.rs"):
        relative = path.relative_to(SRC_DIR)
        destination = EXAMPLES_SRC_DIR / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(path, destination)

        example_name = sanitise_example_name(relative)
        mapping[example_name] = relative

    return mapping


def sanitise_example_name(relative: Path) -> str:
    name = relative.as_posix().replace("/", "__").replace("-", "_")
    if name.endswith(".rs"):
        name = name[:-3]
    return name


def emit_example_wrappers(mapping: dict[str, Path]) -> None:
    """Generate ``examples/<name>.rs`` wrappers that pull in the copied files."""

    EXAMPLES_DIR.mkdir(parents=True, exist_ok=True)

    for example_name, relative in mapping.items():
        if relative.parts and relative.parts[0] == "bin":
            # Skip generated binaries that already contain ``fn main`` entries.
            continue
        wrapper = EXAMPLES_DIR / f"{example_name}.rs"
        module_path = build_module_path(relative)
        header = "\n".join(
            [
                "#![allow(dead_code)]",
                "#![allow(unused_imports)]",
                "#![allow(clippy::all)]",
                "",
                "#[path = \"src/lib.rs\"]",
                "mod source;",
                "",
                "pub use source::*;",
                "",
            ]
        )
        with wrapper.open("w", encoding="utf-8") as handle:
            handle.write(header)
            handle.write(f"use {module_path} as target_module;\n\n")
            handle.write("fn main() {}\n")


def build_module_path(relative: Path) -> str:
    parts = list(relative.parts)
    if not parts:
        return "source"

    *parents, filename = parts
    parents = [p.replace("-", "_") for p in parents if p and p != "."]

    if filename == "lib.rs":
        module_parts: list[str] = []
    elif filename == "mod.rs":
        module_parts = parents
    else:
        stem = Path(filename).stem.replace("-", "_")
        module_parts = parents + [stem]

    if module_parts:
        return "::".join(["source", *module_parts])
    return "source"


def ensure_dev_profile_inhibits_inlining() -> None:
    """Add a dev profile to the manifest that keeps optimisation at level 0."""

    content = CARGO_TOML.read_text(encoding="utf-8")
    if "[profile.dev]" in content:
        return

    addition = (
        "\n"
        "[profile.dev]\n"
        "# Keep optimisations (and therefore inlining) disabled for the generated\n"
        "# example binaries so the compiled output mirrors the source closely.\n"
        "opt-level = 0\n"
        "debug = true\n"
        "codegen-units = 1\n"
    )

    with CARGO_TOML.open("a", encoding="utf-8") as handle:
        handle.write(addition)


def run_cargo_test_examples() -> str:
    """Execute ``cargo test --examples`` and return the combined stdout/stderr."""

    process = subprocess.run(
        ["cargo", "test", "--examples"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )

    print(process.stdout)
    process.check_returncode()
    return process.stdout


def rename_example_binaries(output: str, mapping: dict[str, Path]) -> None:
    """Rename hashed test binaries under ``target/debug/deps`` using the mapping."""

    pattern = re.compile(r"Running target/debug/deps/([A-Za-z0-9_+-]+)")
    seen: set[str] = set()

    for match in pattern.finditer(output):
        base = match.group(1).split("-")[0]
        seen.add(base)

    for example_name in seen:
        relative = mapping.get(example_name)
        if relative is None:
            continue

        readable = sanitise_example_name(relative)
        for artefact in TARGET_DEPS.glob(f"{example_name}-*"):
            target = artefact.with_name(readable + artefact.suffix)
            if target.exists():
                if target.is_file() or target.is_symlink():
                    target.unlink()
                else:
                    shutil.rmtree(target)
            artefact.rename(target)


def main() -> None:
    mapping = copy_source_tree()
    emit_example_wrappers(mapping)
    ensure_dev_profile_inhibits_inlining()
    output = run_cargo_test_examples()
    rename_example_binaries(output, mapping)


if __name__ == "__main__":
    main()
