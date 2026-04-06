#!/usr/bin/env python3
"""Extend an existing AWS manifest with additional types/operations.

Usage:
    python3 codegen/extend.py <api> --available-types
    python3 codegen/extend.py <api> --add-type <ShapeName>
    python3 codegen/extend.py <api> --diff
    python3 codegen/extend.py --diff-all
"""
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

MODEL_CACHE_DIR = Path("codegen/model_cache")
MANIFESTS_DIR = Path("codegen/manifests")


def load_manifest(api: str) -> tuple[dict, dict]:
    """Load manifest and its botocore model."""
    manifest_path = MANIFESTS_DIR / f"{api}.toml"
    if not manifest_path.exists():
        print(f"Manifest not found: {manifest_path}", file=sys.stderr)
        sys.exit(1)

    with open(manifest_path, "rb") as f:
        manifest = tomllib.load(f)

    api_section = manifest["api"]
    service_name = api_section.get("botocore_service", api_section["name"])
    model_file = MODEL_CACHE_DIR / f"{service_name}.service-2.json"

    with open(model_file) as f:
        model = json.load(f)

    return manifest, model


def available_types(api: str) -> None:
    """List all structure shapes in the botocore model."""
    _, model = load_manifest(api)
    shapes = model.get("shapes", {})

    print(f"Structure shapes in {api}:\n")
    for name, shape in sorted(shapes.items()):
        if shape.get("type") == "structure":
            members = list(shape.get("members", {}).keys())
            print(f"  {name} ({len(members)} fields)")


def add_type(api: str, shape_name: str) -> None:
    """Generate a TOML entry for a shape."""
    _, model = load_manifest(api)
    shapes = model.get("shapes", {})
    shape = shapes.get(shape_name)

    if not shape:
        print(f"Shape '{shape_name}' not found", file=sys.stderr)
        sys.exit(1)

    members = shape.get("members", {})
    fields = list(members.keys())

    # Check for enum fields
    enum_overrides = []
    for field_name, member in members.items():
        member_shape_name = member.get("shape", "")
        member_shape = shapes.get(member_shape_name, {})
        if "enum" in member_shape:
            enum_overrides.append((field_name, member_shape_name))

    print(f'[[types]]')
    print(f'shape = "{shape_name}"')
    print(f'include_fields = {fields}')

    if enum_overrides:
        print(f'[types.field_overrides]')
        for field_name, enum_shape in enum_overrides:
            print(f'{field_name} = {{ enum_type = "{enum_shape}" }}')

    print()


def diff(api: str) -> None:
    """Show operations in botocore not in manifest."""
    manifest, model = load_manifest(api)
    operations = model.get("operations", {})
    manifest_ops = {op["name"] for op in manifest.get("operations", [])}

    missing = sorted(set(operations.keys()) - manifest_ops)
    if missing:
        print(f"{api}: {len(missing)} operations not in manifest:")
        for op in missing:
            print(f"  - {op}")
    else:
        print(f"{api}: all operations covered")


def diff_all() -> None:
    """Show diff for all manifests."""
    for manifest_path in sorted(MANIFESTS_DIR.glob("*.toml")):
        api = manifest_path.stem
        diff(api)
        print()


def main() -> None:
    import argparse
    parser = argparse.ArgumentParser(description="Extend AWS manifests")
    parser.add_argument("api", nargs="?", help="API name (manifest stem)")
    parser.add_argument("--available-types", action="store_true")
    parser.add_argument("--add-type", metavar="SHAPE")
    parser.add_argument("--diff", action="store_true")
    parser.add_argument("--diff-all", action="store_true")
    args = parser.parse_args()

    if args.diff_all:
        diff_all()
    elif not args.api:
        parser.print_help()
        sys.exit(1)
    elif args.available_types:
        available_types(args.api)
    elif args.add_type:
        add_type(args.api, args.add_type)
    elif args.diff:
        diff(args.api)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
