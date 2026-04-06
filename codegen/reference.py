#!/usr/bin/env python3
"""Generate full reference manifests for all AWS APIs.

Outputs to codegen/reference/ (gitignored).
"""
from __future__ import annotations

import sys
from pathlib import Path

MANIFESTS_DIR = Path("codegen/manifests")
REFERENCE_DIR = Path("codegen/reference")


def main() -> None:
    REFERENCE_DIR.mkdir(parents=True, exist_ok=True)
    # For each manifest, generate a full-coverage version
    for manifest_path in sorted(MANIFESTS_DIR.glob("*.toml")):
        api = manifest_path.stem
        print(f"Generating reference for: {api}")
        # TODO: generate full manifest with all shapes and operations
        print(f"  (reference generation not yet implemented)")

    print(f"\nReference manifests written to: {REFERENCE_DIR}")


if __name__ == "__main__":
    main()
