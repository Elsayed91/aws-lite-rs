#!/usr/bin/env python3
"""Validate AWS manifests and generated code.

Usage:
    python3 codegen/verify.py
"""
from __future__ import annotations

import json
import subprocess
import sys
import tomllib
from pathlib import Path

MANIFESTS_DIR = Path("codegen/manifests")
MODEL_CACHE_DIR = Path("codegen/model_cache")


def validate_manifest(manifest_path: Path) -> list[str]:
    """Validate a single manifest against its botocore model."""
    errors: list[str] = []

    with open(manifest_path, "rb") as f:
        manifest = tomllib.load(f)

    api = manifest.get("api", {})
    api_name = api.get("name", "")
    botocore_service = api.get("botocore_service", api_name)

    # Check botocore model exists
    model_file = MODEL_CACHE_DIR / f"{botocore_service}.service-2.json"
    if not model_file.exists():
        errors.append(f"Botocore model not found: {model_file}")
        return errors

    with open(model_file) as f:
        model = json.load(f)

    shapes = model.get("shapes", {})
    operations = model.get("operations", {})

    # Validate types
    for type_entry in manifest.get("types", []):
        shape_name = type_entry.get("shape", "")
        if shape_name not in shapes:
            errors.append(f"Shape '{shape_name}' not found in botocore model")
            continue

        shape = shapes[shape_name]
        members = shape.get("members", {})
        include_fields = type_entry.get("include_fields")
        if include_fields:
            for field in include_fields:
                if field not in members:
                    errors.append(f"Field '{field}' not found in shape '{shape_name}'")

        # Validate enum_type overrides
        for field_name, override in type_entry.get("field_overrides", {}).items():
            if isinstance(override, dict) and "enum_type" in override:
                member = members.get(field_name, {})
                member_shape_name = member.get("shape", "")
                member_shape = shapes.get(member_shape_name, {})
                # Check direct enum or list-of-enum (follow list -> member shape)
                has_enum = "enum" in member_shape
                if not has_enum and member_shape.get("type") == "list":
                    list_member_name = member_shape.get("member", {}).get("shape", "")
                    list_member_shape = shapes.get(list_member_name, {})
                    has_enum = "enum" in list_member_shape
                if not has_enum:
                    errors.append(
                        f"enum_type declared for '{field_name}' but shape "
                        f"'{member_shape_name}' has no enum values"
                    )

    # Validate operations
    for op_entry in manifest.get("operations", []):
        op_name = op_entry.get("name", "")
        if op_name not in operations:
            errors.append(f"Operation '{op_name}' not found in botocore model")

    return errors


def main() -> int:
    print("=== AWS Manifest Validation ===\n")

    all_errors: list[str] = []
    for manifest_path in sorted(MANIFESTS_DIR.glob("*.toml")):
        print(f"Validating: {manifest_path.name}")
        errors = validate_manifest(manifest_path)
        if errors:
            for e in errors:
                print(f"  ERROR: {e}")
            all_errors.extend(errors)
        else:
            print("  OK")

    if all_errors:
        print(f"\n{len(all_errors)} validation error(s) found!")
        return 1

    print("\n=== Cargo Check ===")
    result = subprocess.run(["cargo", "check", "-p", "aws-lite"], capture_output=True, text=True)
    if result.returncode != 0:
        print(result.stderr)
        return 1
    print("  OK")

    print("\n=== Cargo Clippy ===")
    result = subprocess.run(
        ["cargo", "clippy", "-p", "aws-lite", "--", "-D", "warnings"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(result.stderr)
        return 1
    print("  OK")

    print("\n=== Cargo Test ===")
    result = subprocess.run(
        ["cargo", "test", "-p", "aws-lite", "--lib"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(result.stdout)
        print(result.stderr)
        return 1
    # Print test summary line
    for line in result.stdout.split("\n"):
        if "test result:" in line:
            print(f"  {line.strip()}")
    print("  OK")

    print("\nAll checks passed!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
