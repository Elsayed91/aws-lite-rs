#!/usr/bin/env python3
"""Bootstrap a new AWS API manifest from a botocore service model.

Usage:
    python3 codegen/bootstrap.py <service-name>
    python3 codegen/bootstrap.py logs
    python3 codegen/bootstrap.py cloudwatch
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

MODEL_CACHE_DIR = Path("codegen/model_cache")
MANIFESTS_DIR = Path("codegen/manifests")


def _to_snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def bootstrap(service_name: str) -> str:
    """Generate a draft manifest for a botocore service."""
    model_file = MODEL_CACHE_DIR / f"{service_name}.service-2.json"
    if not model_file.exists():
        print(f"Model not found: {model_file}", file=sys.stderr)
        print(f"Run: python3 codegen/fetch_models.py {service_name}", file=sys.stderr)
        sys.exit(1)

    with open(model_file) as f:
        model = json.load(f)

    metadata = model.get("metadata", {})
    shapes = model.get("shapes", {})
    operations = model.get("operations", {})

    # Determine wire format
    protocol = metadata.get("protocol", "json")
    if protocol == "query":
        wire_format = "query_xml"
    elif protocol == "rest-xml":
        wire_format = "rest_xml"
    elif metadata.get("targetPrefix"):
        wire_format = "json_target"
    else:
        wire_format = "json"

    lines = []
    lines.append(f"# {metadata.get('serviceFullName', service_name)} manifest")
    lines.append(f"# Botocore model: {service_name}")
    lines.append("")
    lines.append("[api]")
    lines.append(f'name = "{service_name}"')
    lines.append(f'display_name = "{metadata.get("serviceFullName", service_name)}"')
    lines.append('version = "v1"')
    lines.append(f'api_version = "{metadata.get("apiVersion", "")}"')
    lines.append(f'service_name = "{metadata.get("endpointPrefix", service_name)}"')
    lines.append(f'wire_format = "{wire_format}"')

    if wire_format == "json_target":
        lines.append(f'target_prefix = "{metadata.get("targetPrefix", "")}"')
        lines.append(f'json_version = "{metadata.get("jsonVersion", "1.1")}"')

    lines.append(f'endpoint_prefix = "{metadata.get("endpointPrefix", service_name)}"')
    lines.append("")
    lines.append("[api.client]")
    lines.append(f'accessor_name = "{service_name}"')

    # PascalCase client struct
    client_name = "".join(w.capitalize() for w in service_name.replace("-", "_").split("_"))
    lines.append(f'client_struct = "{client_name}Client"')
    lines.append("")

    # Collect referenced types
    referenced_shapes: set[str] = set()
    for op_name, op in operations.items():
        input_shape = op.get("input", {}).get("shape", "")
        output_shape = op.get("output", {}).get("shape", "")
        if input_shape:
            referenced_shapes.add(input_shape)
        if output_shape:
            referenced_shapes.add(output_shape)

    # Types section
    lines.append("# === Types ===")
    lines.append("")
    for shape_name in sorted(referenced_shapes):
        shape = shapes.get(shape_name, {})
        if shape.get("type") != "structure":
            continue
        members = list(shape.get("members", {}).keys())
        lines.append(f"# [[types]]")
        lines.append(f'# shape = "{shape_name}"')
        lines.append(f"# include_fields = {members}")
        lines.append("")

    # Operations section
    lines.append("# === Operations ===")
    lines.append("")
    for op_name in sorted(operations.keys()):
        snake = _to_snake(op_name)
        op = operations[op_name]
        desc = op.get("documentation", "").replace('"', '\\"')[:80]
        lines.append(f"# [[operations]]")
        lines.append(f'# name = "{op_name}"')
        lines.append(f'# rust_name = "{snake}"')
        if desc:
            lines.append(f'# description = "{desc}"')
        lines.append("")

    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 codegen/bootstrap.py <service-name>")
        sys.exit(1)

    service_name = sys.argv[1]
    output = bootstrap(service_name)

    out_path = MANIFESTS_DIR / f"{service_name}.toml"
    if out_path.exists():
        print(f"Manifest already exists: {out_path}")
        print("Use extend.py to add types/operations to an existing manifest.")
        sys.exit(1)

    print(output)
    print(f"\n# To save: redirect to {out_path}")


if __name__ == "__main__":
    main()
