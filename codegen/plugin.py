"""AWS provider plugin for the codegen pipeline.

Reads TOML manifests + botocore service-2.json models and produces
fully-resolved IR dataclasses for the shared Rust emitter.
"""

from __future__ import annotations

import json
import re
import tomllib
from pathlib import Path

from cloud_lite_codegen.ir import (
    ApiDef,
    ClientConfig,
    EnumDef,
    EnumVariant,
    FieldDef,
    FieldFormat,
    HttpMethod,
    OperationDef,
    ProviderDef,
    TypeDef,
)
from cloud_lite_codegen.plugin import ProviderPlugin


def _to_snake_case(name: str) -> str:
    """Convert camelCase or PascalCase to snake_case."""
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def _serde_pascal_case(snake: str) -> str:
    """Simulate serde's PascalCase rename_all conversion from snake_case.

    serde splits on underscores and capitalizes the first letter of each word:
      db_instances -> DbInstances
    This may differ from the actual botocore name (DBInstances).
    """
    return "".join(word.capitalize() for word in snake.split("_"))


def _strip_html(text: str) -> str:
    """Remove HTML tags from botocore documentation strings."""
    if not text:
        return ""
    return re.sub(r"<[^>]+>", "", text).strip()


def _sanitize_variant_name(value: str) -> str:
    """Convert an arbitrary enum string value into a valid Rust PascalCase identifier.

    Handles all botocore patterns:
    - SCREAMING_SNAKE_CASE: "INFREQUENT_ACCESS" -> "InfrequentAccess"
    - PascalCase (passthrough): "SampleCount" -> "SampleCount"
    - lowercase: "json" -> "Json"
    - Special characters: "Bytes/Second" -> "BytesPerSecond"
    """
    # Replace / with Per (AWS convention: "Bytes/Second" -> "BytesPerSecond")
    s = value.replace("/", "Per")
    # Replace any remaining non-alphanumeric/underscore chars with underscore
    s = re.sub(r"[^a-zA-Z0-9_]", "_", s)
    # Remove leading/trailing underscores
    s = s.strip("_")
    if not s:
        return "Empty"

    # If it contains underscores, split and capitalize each word
    if "_" in s:
        return "".join(word.capitalize() for word in s.split("_") if word)

    # If all uppercase and single "word" (no underscores), capitalize properly
    # e.g. "OK" -> "Ok", "ALARM" -> "Alarm"
    if s == s.upper() and len(s) > 1:
        return s[0] + s[1:].lower()

    # Otherwise capitalize first letter, leave rest as-is
    # Handles PascalCase passthrough and lowercase like "json" -> "Json"
    return s[0].upper() + s[1:]


def _extract_enums_from_type(
    type_entry: dict,
    shapes: dict,
    api_name: str,
) -> list[EnumDef]:
    """Extract EnumDef instances from field_overrides that declare enum_type.

    Only generates enums for fields with explicit enum_type overrides in the
    manifest. Mirrors the GCP plugin's _extract_enums() pattern.
    """
    field_overrides = type_entry.get("field_overrides", {})
    shape_name = type_entry["shape"]
    shape = shapes.get(shape_name, {})
    members = shape.get("members", {})
    include_fields = type_entry.get("include_fields")
    enums: list[EnumDef] = []

    field_order = include_fields if include_fields else list(members.keys())

    for member_name in field_order:
        override = field_overrides.get(member_name, {})
        if not isinstance(override, dict):
            continue
        enum_type_name = override.get("enum_type", "")
        if not enum_type_name:
            continue

        # Find the member's shape to get enum values.
        # Handles both direct string enums and list-of-enum (e.g. Statistics -> list of Statistic).
        member = members.get(member_name, {})
        member_shape_name = member.get("shape", "")
        member_shape = shapes.get(member_shape_name, {})

        enum_values = member_shape.get("enum", [])
        if not enum_values and member_shape.get("type") == "list":
            # Follow through list to its member shape
            list_member_shape_name = member_shape.get("member", {}).get("shape", "")
            list_member_shape = shapes.get(list_member_shape_name, {})
            enum_values = list_member_shape.get("enum", [])

        if not enum_values:
            print(
                f"  WARNING: enum_type '{enum_type_name}' declared for "
                f"'{member_name}' but shape '{member_shape_name}' has no enum values"
            )
            continue

        # Determine if all values are SCREAMING_SNAKE_CASE
        # (only alphanumeric + underscore, all uppercase letters)
        all_screaming = all(
            re.fullmatch(r"[A-Z0-9_]+", v) is not None for v in enum_values if v
        )

        variants: list[EnumVariant] = []
        for val in enum_values:
            rust_variant = _sanitize_variant_name(val)
            variants.append(
                EnumVariant(
                    api_name=val,
                    rust_name=rust_variant,
                    description="",
                )
            )

        enums.append(
            EnumDef(
                name=enum_type_name,
                variants=variants,
                all_screaming=all_screaming,
                has_unknown=True,
                api_path=f"{api_name}.{shape_name}.{member_name}",
            )
        )

    return enums


def _collect_shape_deps(
    type_entry: dict,
    shapes: dict,
    known_shapes: set[str],
) -> set[str]:
    """Collect structure shapes referenced by a type's fields that aren't already known.

    Handles direct structure references and list-of-structure references.
    """
    shape_name = type_entry["shape"]
    shape = shapes.get(shape_name, {})
    members = shape.get("members", {})
    include_fields = type_entry.get("include_fields")
    deps: set[str] = set()

    field_names = include_fields if include_fields else list(members.keys())
    for member_name in field_names:
        member = members.get(member_name, {})
        member_shape_name = member.get("shape", "")
        member_shape = shapes.get(member_shape_name, {})
        member_type = member_shape.get("type", "string")

        if member_type == "structure" and member_shape_name not in known_shapes:
            deps.add(member_shape_name)
        elif member_type == "list":
            item_shape_name = member_shape.get("member", {}).get("shape", "")
            item_shape = shapes.get(item_shape_name, {})
            if item_shape.get("type") == "structure" and item_shape_name not in known_shapes:
                deps.add(item_shape_name)

    return deps


def _resolve_shape_type(shape_name: str, shapes: dict) -> tuple[str, FieldFormat]:
    """Resolve a botocore shape reference to (Rust type string, FieldFormat)."""
    shape = shapes.get(shape_name, {})
    shape_type = shape.get("type", "string")

    if shape_type == "string":
        return ("String", FieldFormat.NONE)
    elif shape_type == "integer":
        return ("i32", FieldFormat.NONE)
    elif shape_type == "long":
        return ("i64", FieldFormat.INT64)
    elif shape_type == "double" or shape_type == "float":
        return ("f64", FieldFormat.NONE)
    elif shape_type == "boolean":
        return ("bool", FieldFormat.NONE)
    elif shape_type == "timestamp":
        return ("String", FieldFormat.DATE_TIME)
    elif shape_type == "blob":
        return ("String", FieldFormat.BYTES)
    elif shape_type == "list":
        member_shape = shape.get("member", {}).get("shape", "String")
        inner_type, _ = _resolve_shape_type(member_shape, shapes)
        # If inner resolves to a structure, use the shape name as the Rust type
        inner_shape = shapes.get(member_shape, {})
        if inner_shape.get("type") == "structure":
            inner_type = member_shape
        return (f"Vec<{inner_type}>", FieldFormat.NONE)
    elif shape_type == "map":
        key_shape = shape.get("key", {}).get("shape", "String")
        value_shape = shape.get("value", {}).get("shape", "String")
        key_type, _ = _resolve_shape_type(key_shape, shapes)
        value_type, _ = _resolve_shape_type(value_shape, shapes)
        return (f"HashMap<{key_type}, {value_type}>", FieldFormat.NONE)
    elif shape_type == "structure":
        return (shape_name, FieldFormat.NONE)
    else:
        return ("String", FieldFormat.NONE)


class AwsPlugin(ProviderPlugin):
    """AWS provider plugin using botocore service-2.json models."""

    def __init__(self) -> None:
        self._base_dir = Path("codegen")
        self._manifests_dir = self._base_dir / "manifests"
        self._model_cache_dir = self._base_dir / "model_cache"

    def name(self) -> str:
        return "aws"

    def target_crate(self) -> str:
        return "."

    def resolve(self, manifest_path: str) -> ApiDef:
        """Resolve a single AWS manifest into an ApiDef."""
        with open(manifest_path, "rb") as f:
            manifest = tomllib.load(f)

        api_section = manifest["api"]
        api_name = api_section["name"]
        wire_format = api_section.get("wire_format", "json")

        # Load botocore model
        model = self._load_model(api_section.get("botocore_service", api_name))
        metadata = model.get("metadata", {})
        shapes = model.get("shapes", {})
        operations = model.get("operations", {})

        # Build base URL from endpoint prefix
        endpoint_prefix = api_section.get("endpoint_prefix", metadata.get("endpointPrefix", api_name))
        # Manifest can override with global_endpoint = "" to force regional URLs
        if "global_endpoint" in api_section:
            global_endpoint = api_section["global_endpoint"] or None
        else:
            global_endpoint = metadata.get("globalEndpoint")
        if global_endpoint:
            # Global services (e.g. IAM, STS) use a fixed endpoint
            base_url = f"https://{global_endpoint}"
        else:
            # Regional services use a per-region URL template
            base_url = f"https://{endpoint_prefix}.{{region}}.amazonaws.com"

        # Detect botocore protocol (ec2 protocol uses locationName for XML elements)
        botocore_protocol = metadata.get("protocol", "")

        # Resolve types from manifest + build shape-name -> rust-name mapping
        type_defs = []
        shape_rename_map: dict[str, str] = {}
        for type_entry in manifest.get("types", []):
            td = self._resolve_type(type_entry, shapes, api_name, botocore_protocol)
            if td:
                type_defs.append(td)
                # Track renames: original botocore shape -> manifest rust_name
                shape_rename_map[type_entry["shape"]] = td.name

        # Also generate types for operation input shapes that aren't in the manifest.
        # For rest_xml, auto-generate the payload shape (not the full input shape).
        all_manifest_shapes = {te["shape"] for te in manifest.get("types", [])}
        for op_entry in manifest.get("operations", []):
            op_name = op_entry["name"]
            op = operations.get(op_name, {})
            input_shape_name = op.get("input", {}).get("shape", "")
            if not input_shape_name:
                continue

            if wire_format == "rest_xml":
                # For REST-XML, auto-generate the payload struct (not the request wrapper)
                input_shape = shapes.get(input_shape_name, {})
                payload_member_name = input_shape.get("payload", "")
                if payload_member_name:
                    payload_member = input_shape.get("members", {}).get(payload_member_name, {})
                    payload_shape_name = payload_member.get("shape", "")
                    payload_shape = shapes.get(payload_shape_name, {})
                    if (
                        payload_shape.get("type") == "structure"
                        and payload_shape_name not in all_manifest_shapes
                    ):
                        auto_td = self._resolve_type({"shape": payload_shape_name}, shapes, api_name, botocore_protocol)
                        if auto_td:
                            type_defs.append(auto_td)
                            shape_rename_map[payload_shape_name] = auto_td.name
                            all_manifest_shapes.add(payload_shape_name)
            else:
                if input_shape_name not in all_manifest_shapes:
                    auto_td = self._resolve_type({"shape": input_shape_name}, shapes, api_name, botocore_protocol)
                    if auto_td:
                        type_defs.append(auto_td)
                        shape_rename_map[input_shape_name] = auto_td.name
                        all_manifest_shapes.add(input_shape_name)

        # Extract enums from manifest types (with deduplication by name)
        enums: list[EnumDef] = []
        seen_enum_names: set[str] = set()
        for type_entry in manifest.get("types", []):
            for enum_def in _extract_enums_from_type(type_entry, shapes, api_name):
                if enum_def.name not in seen_enum_names:
                    enums.append(enum_def)
                    seen_enum_names.add(enum_def.name)

        # Auto-discover structure dependencies from all resolved types
        needed_deps: set[str] = set()
        # Build synthetic type_entry dicts for all resolved types (manifest + auto-input)
        all_type_entries = list(manifest.get("types", []))
        for td in type_defs:
            if td.schema_name not in {te["shape"] for te in manifest.get("types", [])}:
                all_type_entries.append({"shape": td.schema_name})
        for type_entry in all_type_entries:
            deps = _collect_shape_deps(type_entry, shapes, all_manifest_shapes)
            needed_deps.update(deps)

        # One level of transitive deps
        more_deps: set[str] = set()
        for dep_name in needed_deps:
            dep_shape = shapes.get(dep_name, {})
            for _mname, member in dep_shape.get("members", {}).items():
                ref_shape_name = member.get("shape", "")
                ref_shape = shapes.get(ref_shape_name, {})
                if (
                    ref_shape.get("type") == "structure"
                    and ref_shape_name not in all_manifest_shapes
                    and ref_shape_name not in needed_deps
                ):
                    more_deps.add(ref_shape_name)
                # Also check list-of-structure
                if ref_shape.get("type") == "list":
                    item_name = ref_shape.get("member", {}).get("shape", "")
                    item_shape = shapes.get(item_name, {})
                    if (
                        item_shape.get("type") == "structure"
                        and item_name not in all_manifest_shapes
                        and item_name not in needed_deps
                    ):
                        more_deps.add(item_name)
        needed_deps.update(more_deps)

        # Generate auto-dependency types
        auto_types: list[TypeDef] = []
        for dep_name in sorted(needed_deps):
            auto_td = self._resolve_type({"shape": dep_name}, shapes, api_name, botocore_protocol)
            if auto_td:
                auto_td.is_auto_dependency = True
                auto_types.append(auto_td)
                shape_rename_map[dep_name] = auto_td.name

        # Resolve operations from manifest
        op_defs = []
        for op_entry in manifest.get("operations", []):
            od = self._resolve_operation(op_entry, operations, shapes, api_section, shape_rename_map)
            if od:
                op_defs.append(od)

        client_section = api_section.get("client", {})

        # Default: query_xml and rest_xml use PascalCase, json/json_target use camelCase.
        # Allow manifest to override (e.g. Config uses PascalCase with json_target).
        # EC2 protocol: "none" suppresses rename_all because every field has an explicit
        # serde_rename from botocore locationName (EC2 XML uses irregular camelCase names).
        if botocore_protocol == "ec2":
            default_rename = "none"
        elif wire_format in ("query_xml", "rest_xml"):
            default_rename = "PascalCase"
        else:
            default_rename = "camelCase"
        rename_all = api_section.get("rename_all", default_rename)

        return ApiDef(
            name=api_name,
            display_name=api_section.get("display_name", metadata.get("serviceFullName", api_name)),
            version=api_section.get("version", "v1"),
            base_url=base_url,
            doc_url=api_section.get("doc_url", ""),
            wire_format=wire_format,
            rename_all=rename_all,
            api_version=api_section.get("api_version", metadata.get("apiVersion", "")),
            service_name=api_section.get("service_name", metadata.get("endpointPrefix", "")),
            json_version=api_section.get("json_version", metadata.get("jsonVersion", "")),
            target_prefix=api_section.get("target_prefix", metadata.get("targetPrefix", "")),
            endpoint_prefix=endpoint_prefix,
            xml_namespace=api_section.get("xml_namespace", ""),
            client=ClientConfig(
                client_struct=client_section.get("client_struct", ""),
                accessor_name=client_section.get("accessor_name", api_name),
            ),
            types=type_defs,
            enums=enums,
            auto_types=auto_types,
            operations=op_defs,
        )

    def resolve_all(self) -> ProviderDef:
        """Resolve all manifests into a ProviderDef."""
        apis: list[ApiDef] = []
        for manifest_path in sorted(self._manifests_dir.glob("*.toml")):
            apis.append(self.resolve(str(manifest_path)))
        return ProviderDef(
            provider="aws",
            target_crate=".",
            client_struct="AwsHttpClient",
            apis=apis,
            rename_all="PascalCase",
            wire_format="json",
            spec_source_name="the AWS Botocore Model",
            api_doc_label="AWS API",
            error_invalid_response="crate::AwsError::InvalidResponse",
            error_type="crate::AwsError",
            result_type="crate::Result",
        )

    def _load_model(self, service_name: str) -> dict:
        """Load a botocore service-2.json from cache."""
        cache_file = self._model_cache_dir / f"{service_name}.service-2.json"
        if not cache_file.exists():
            raise FileNotFoundError(
                f"Botocore model not found: {cache_file}\n"
                f"Run: python3 codegen/fetch_models.py {service_name}"
            )
        with open(cache_file) as f:
            return json.load(f)

    def _resolve_type(
        self,
        type_entry: dict,
        shapes: dict,
        _api_name: str,
        botocore_protocol: str = "",
    ) -> TypeDef | None:
        """Resolve a [[types]] manifest entry into a TypeDef."""
        shape_name = type_entry["shape"]
        shape = shapes.get(shape_name)
        if not shape:
            print(f"  WARNING: Shape '{shape_name}' not found in botocore model")
            return None

        if shape.get("type") != "structure":
            print(f"  WARNING: Shape '{shape_name}' is not a structure (type={shape.get('type')})")
            return None

        rust_name = type_entry.get("rust_name", shape_name)
        include_fields = type_entry.get("include_fields")
        field_overrides = type_entry.get("field_overrides", {})
        members = shape.get("members", {})
        required_set = set(shape.get("required", []))

        fields: list[FieldDef] = []
        all_member_names = list(members.keys())
        included_names = include_fields if include_fields else all_member_names

        for member_name in included_names:
            member = members.get(member_name)
            if not member:
                print(f"  WARNING: Member '{member_name}' not found in shape '{shape_name}'")
                continue

            member_shape_name = member.get("shape", "String")
            rust_type, field_format = _resolve_shape_type(member_shape_name, shapes)
            member_shape = shapes.get(member_shape_name, {})
            is_required = member_name in required_set
            is_repeated = member_shape.get("type") == "list"
            is_map = member_shape.get("type") == "map"

            # Apply field overrides
            overrides = field_overrides.get(member_name, {})
            if not isinstance(overrides, dict):
                overrides = {}
            if "required" in overrides:
                is_required = overrides["required"]
            if "rust_type" in overrides:
                rust_type = overrides["rust_type"]

            # enum_type override: use the enum name as the Rust type.
            # For list fields (Vec<String>), wrap as Vec<EnumType>.
            enum_type_name = overrides.get("enum_type", "")
            if enum_type_name:
                if is_repeated:
                    rust_type = f"Vec<{enum_type_name}>"
                else:
                    rust_type = enum_type_name

            description = _strip_html(member.get("documentation", ""))
            serde_rename = overrides.get("serde_rename", "")

            # EC2 protocol: every field gets an explicit serde_rename because
            # we suppress struct-level rename_all (EC2 XML uses irregular names).
            #
            # Botocore EC2 serialization algorithm for query params:
            #   queryName > upper_first(locationName) > member_name
            # XML deserialization uses locationName directly.
            #
            # When serialize_name differs from deserialize_name (locationName),
            # we emit split serde renames:
            #   #[serde(rename(serialize = "X", deserialize = "Y"))]
            serde_rename_de = ""
            if botocore_protocol == "ec2" and not serde_rename:
                location_name = member.get("locationName", "")
                query_name = member.get("queryName", "")
                # Serialize: queryName > upper_first(locationName) > member_name
                if query_name:
                    serialize_name = query_name
                elif location_name:
                    serialize_name = location_name[0].upper() + location_name[1:]
                else:
                    serialize_name = member_name
                # Deserialize: locationName > member_name
                deserialize_name = location_name if location_name else member_name
                if serialize_name != deserialize_name:
                    serde_rename = serialize_name
                    serde_rename_de = deserialize_name
                else:
                    serde_rename = serialize_name

            # Non-EC2 PascalCase protocols (query, rest-xml): detect when serde's
            # rename_all PascalCase conversion of the snake_case field name
            # doesn't match the actual botocore member name.  This happens with
            # acronym-prefixed fields like DBInstances, IAMDatabaseAuthenticationEnabled,
            # ARN fields, etc.  Emit an explicit serde_rename for these.
            if not serde_rename and botocore_protocol in ("query", "rest-xml"):
                snake = _to_snake_case(member_name)
                round_tripped = _serde_pascal_case(snake)
                if round_tripped != member_name:
                    serde_rename = member_name

            fields.append(FieldDef(
                name=member_name,
                rust_name=_to_snake_case(member_name),
                rust_type=rust_type,
                required=is_required,
                repeated=is_repeated,
                is_map=is_map,
                format=field_format,
                enum_type=enum_type_name,
                serde_rename=serde_rename,
                serde_rename_deserialize=serde_rename_de,
                description=description,
            ))

        # Compute coverage
        total_fields = len(all_member_names)
        included_fields = len(fields)
        omitted = {}
        if include_fields:
            for m in all_member_names:
                if m not in include_fields:
                    omitted[m] = "not selected in manifest"

        return TypeDef(
            name=rust_name,
            schema_name=shape_name,
            fields=fields,
            description=_strip_html(shape.get("documentation", "")),
            total_fields=total_fields,
            included_fields=included_fields,
            omitted=omitted,
        )

    def _resolve_operation(
        self,
        op_entry: dict,
        operations: dict,
        shapes: dict,
        api_section: dict,
        shape_rename_map: dict[str, str] | None = None,
    ) -> OperationDef | None:
        """Resolve a [[operations]] manifest entry into an OperationDef."""
        op_name = op_entry["name"]
        op = operations.get(op_name)
        if not op:
            print(f"  WARNING: Operation '{op_name}' not found in botocore model")
            return None

        rust_name = op_entry.get("rust_name", _to_snake_case(op_name))
        description = op_entry.get("description", _strip_html(op.get("documentation", "")))
        rename = shape_rename_map or {}
        wire_format = api_section.get("wire_format", "json")

        # HTTP method from botocore
        http = op.get("http", {})
        method_str = http.get("method", "POST").upper()
        http_method = HttpMethod(method_str)

        # Input/output shapes — use rename map to get manifest rust_names
        input_shape_name = op.get("input", {}).get("shape", "")
        output_shape_name = op.get("output", {}).get("shape", "")

        if wire_format == "rest_xml":
            return self._resolve_rest_xml_operation(
                op, op_name, rust_name, description, http, http_method,
                input_shape_name, output_shape_name, shapes, rename,
            )

        if wire_format == "rest_json":
            return self._resolve_rest_json_operation(
                op, op_name, rust_name, description, http, http_method,
                input_shape_name, output_shape_name, shapes, rename,
            )

        # For AWS query/json protocols, request body is the full input shape
        request_body_type = ""
        if input_shape_name:
            input_shape = shapes.get(input_shape_name, {})
            if input_shape.get("type") == "structure":
                request_body_type = rename.get(input_shape_name, input_shape_name)

        # Response type is the output shape
        response_type = ""
        if output_shape_name:
            output_shape = shapes.get(output_shape_name, {})
            if output_shape.get("type") == "structure":
                response_type = rename.get(output_shape_name, output_shape_name)

        return OperationDef(
            name=rust_name,
            http_method=http_method,
            url_template="/",  # AWS services use POST to / with action in body/header
            request_body_type=request_body_type or "",
            response_type=response_type or "",
            description=description,
            original_name=op_name,
        )

    def _resolve_rest_xml_operation(
        self,
        op: dict,
        op_name: str,
        rust_name: str,
        description: str,
        http: dict,
        http_method: HttpMethod,
        input_shape_name: str,
        output_shape_name: str,
        shapes: dict,
        rename: dict[str, str],
    ) -> OperationDef:
        """Resolve a REST-XML operation with URI templates, path params, and payload."""
        from cloud_lite_codegen.ir import PathParam

        # Extract URI template from botocore (e.g. "/{Bucket}?policy")
        request_uri = http.get("requestUri", "/")

        # Parse path params from URI template
        path_params: list[PathParam] = []
        for match in re.finditer(r"\{(\w+)\}", request_uri):
            param_name = match.group(1)
            path_params.append(PathParam(
                name=param_name,
                rust_name=_to_snake_case(param_name),
            ))

        # Find payload member in input shape
        request_body_type = ""
        input_shape = shapes.get(input_shape_name, {}) if input_shape_name else {}
        payload_member_name = input_shape.get("payload", "")

        if payload_member_name:
            payload_member = input_shape.get("members", {}).get(payload_member_name, {})
            payload_shape_name = payload_member.get("shape", "")
            payload_shape = shapes.get(payload_shape_name, {})
            payload_type = payload_shape.get("type", "string")

            if payload_type == "structure":
                request_body_type = rename.get(payload_shape_name, payload_shape_name)
            elif payload_type == "string":
                request_body_type = "str"
        elif http_method in (HttpMethod.PUT, HttpMethod.POST, HttpMethod.PATCH) and input_shape_name:
            # No explicit payload key — check if there are body members (no location).
            # When all members are body fields (not uri/querystring/header), the entire
            # input struct is serialized as the XML body (e.g. Route53 CreateHealthCheck).
            path_param_names = {match.group(1) for match in re.finditer(r"\{(\w+)\}", request_uri)}
            has_body_members = any(
                member_data.get("location", "") not in ("uri", "querystring", "header", "headers")
                and member_name not in path_param_names
                for member_name, member_data in input_shape.get("members", {}).items()
            )
            if has_body_members:
                request_body_type = rename.get(input_shape_name, input_shape_name)

        # Response type — for REST-XML, the HTTP body IS the payload shape, not the wrapper.
        # Headers like ETag, Location are in HTTP headers, not the body.
        response_type = ""
        if output_shape_name:
            output_shape = shapes.get(output_shape_name, {})
            if output_shape.get("type") == "structure":
                payload_member_name = output_shape.get("payload", "")
                if payload_member_name:
                    # Use the payload shape as the response type (body is the payload directly)
                    payload_member = output_shape.get("members", {}).get(payload_member_name, {})
                    payload_shape_name = payload_member.get("shape", "")
                    if payload_shape_name:
                        response_type = rename.get(payload_shape_name, payload_shape_name)
                else:
                    # No payload — check if there are any body members
                    members = output_shape.get("members", {})
                    has_body_members = any(
                        "location" not in m for m in members.values()
                    )
                    if has_body_members:
                        response_type = rename.get(output_shape_name, output_shape_name)

        return OperationDef(
            name=rust_name,
            http_method=http_method,
            url_template=request_uri,
            path_params=path_params,
            request_body_type=request_body_type,
            response_type=response_type or "",
            description=description,
            original_name=op_name,
        )

    def _resolve_rest_json_operation(
        self,
        op: dict,
        op_name: str,
        rust_name: str,
        description: str,
        http: dict,
        http_method: HttpMethod,
        input_shape_name: str,
        output_shape_name: str,
        shapes: dict,
        rename: dict[str, str],
    ) -> OperationDef:
        """Resolve a REST-JSON operation with URI templates, path params, and JSON body."""
        from cloud_lite_codegen.ir import PathParam, QueryParam

        # Extract URI template from botocore (e.g. "/2015-03-31/functions/{FunctionName}/configuration")
        request_uri = http.get("requestUri", "/")
        # Strip query string placeholder from URI (e.g. "?marker={Marker}" parts)
        request_uri_path = request_uri.split("?")[0]

        # Parse path params from URI template (location=uri members)
        path_params: list[PathParam] = []
        path_param_names: set[str] = set()
        for match in re.finditer(r"\{(\w+)\}", request_uri_path):
            param_name = match.group(1)
            path_params.append(PathParam(
                name=param_name,
                rust_name=_to_snake_case(param_name),
            ))
            path_param_names.add(param_name)

        # Find query params and body fields from input shape members
        query_params: list[QueryParam] = []
        input_shape = shapes.get(input_shape_name, {}) if input_shape_name else {}
        has_body_fields = False

        for member_name, member_data in input_shape.get("members", {}).items():
            location = member_data.get("location", "")
            location_name = member_data.get("locationName", member_name)
            required_members = input_shape.get("required", [])
            is_required = member_name in required_members

            if location == "querystring":
                query_params.append(QueryParam(
                    name=location_name,
                    rust_name=_to_snake_case(member_name),
                    required=is_required,
                ))
            elif location == "uri":
                pass  # Already handled as path params
            elif location in ("header", "headers"):
                pass  # Skip header params for now
            else:
                # No location = body field
                if member_name not in path_param_names:
                    has_body_fields = True

        # Request body type: only for PUT/POST/PATCH with body fields
        request_body_type = ""
        if http_method in (HttpMethod.PUT, HttpMethod.POST, HttpMethod.PATCH) and has_body_fields:
            if input_shape_name:
                request_body_type = rename.get(input_shape_name, input_shape_name)

        # Response type is the output shape (REST-JSON returns JSON body directly)
        response_type = ""
        if output_shape_name:
            output_shape = shapes.get(output_shape_name, {})
            if output_shape.get("type") == "structure":
                response_type = rename.get(output_shape_name, output_shape_name)

        return OperationDef(
            name=rust_name,
            http_method=http_method,
            url_template=request_uri_path,
            path_params=path_params,
            query_params=query_params,
            request_body_type=request_body_type,
            response_type=response_type or "",
            description=description,
            original_name=op_name,
        )
