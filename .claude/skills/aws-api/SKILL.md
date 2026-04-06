---
name: aws-api
description: Add a new AWS API or extend an existing one. Auto-detects whether to bootstrap a new API or add operations to an existing one.
argument-hint: "[service_name]: [operations needed]"
---

# AWS API Workflow

You are adding or extending an AWS API in aws-lite-rs. The user's request: "$ARGUMENTS"

This workflow uses **integration-first development**: each operation group is validated against the real AWS API before unit tests are written.

## IMMUTABLE RULES

0. **Integration tests MUST pass for an operation group BEFORE writing unit tests.** Unit tests encode proven behavior, not assumptions.
1. **NEVER edit generated files** (`src/types/`, `src/ops/`, `src/test_support/`) — a hook blocks this. Flag issues → manifest fix → wait for approval.
2. **NEVER modify codegen scripts** without explicit user approval.
3. **NEVER skip or work around authentication failures** — ask user to re-authenticate.
4. **NEVER overwrite existing `src/api/*.rs` files** — only create new or add methods.
5. **NEVER choose the simplest fix by default** — explore solutions, choose the one that generalizes and avoids technical debt.

## Prerequisites

Read these docs:
1. `.claude/docs/architecture.md` — 3-layer pattern and wire formats
2. `.claude/docs/codegen-reference.md` — manifest format and tools
3. `.claude/docs/testing-guide.md` — test patterns

## Phase 1: Discovery

1. Parse the user's request: service name, requested operations.

2. **Detect add vs extend**:
   - Check for `codegen/manifests/{service_name}.toml` and `src/api/{service_name}.rs`
   - **Both exist** → EXTEND mode
   - **Neither exists** → ADD mode

3. **ADD mode**:
   ```
   cd codegen && uv run python -m codegen.cli fetch {service_name}
   cd codegen && uv run python -m codegen.cli bootstrap {service_name}
   ```
   Read draft manifest. Determine wire format from botocore model:
   - `protocol: "query"` → `wire_format = "query_xml"` (PascalCase, form-encoded, XML)
   - `protocol: "json"` → `wire_format = "json_target"` (camelCase, JSON, X-Amz-Target)
   - `protocol: "rest-json"` → `wire_format = "json"` (camelCase, JSON)

4. **EXTEND mode**: Read existing manifest and API client. Discover available additions:
   ```
   cd codegen && uv run python -m codegen.cli extend {service_name} --available-types
   cd codegen && uv run python -m codegen.cli extend {service_name} --available-ops
   ```

5. Present available operations (AskUserQuestion). Group by resource, let user select.

6. Plan operation groups for incremental development.

## Phase 2: Manifest

### ADD mode
Edit `codegen/manifests/{service_name}.toml`:
- `[api]` metadata (name, display_name, version, api_version, service_name, wire_format, endpoint_prefix)
- For `json_target`: also set `target_prefix`, `json_version`
- `[api.client]` (accessor_name, client_struct)
- `[[types]]`: botocore `shape`, `include_fields`, field overrides (enum_type, required)
- `[[operations]]`: botocore `name` (PascalCase), `rust_name` (snake_case)
- Compare with existing: `cloudwatch.toml` (query_xml), `logs.toml` (json_target)

### EXTEND mode
Add new `[[types]]` and/or `[[operations]]` following existing manifest conventions. Update existing types if adding fields.

**COMMIT**: `feat: {add|extend} {service_name} manifest`

## Phase 3: Generation

```
cd codegen && uv run python -m codegen.cli apply
cargo check
```

Fix manifest on failure (missing shape, wrong casing, missing dependencies).

**COMMIT**: `feat: generate types/ops for {service_name}`

## Phase 4: Registration & Scaffolding (ADD mode only)

Skip for EXTEND mode.

1. Verify codegen auto-registered in `api/mod.rs` and `client.rs`
2. Create `src/api/{service_name}.rs` with struct shell
3. `cargo check`

**COMMIT**: `feat: scaffold {service_name} API client`

## Phase 5: Incremental Development

For each operation group:

### Step A: Write API Methods
Add methods to `src/api/{service_name}.rs` for THIS GROUP ONLY. Thin wrappers delegating to ops. No LRO handling (AWS operations are synchronous).

### Step B: Write Integration Test
Test against real AWS API. Always-cleanup pattern, step-numbered `println!`, deterministic `cloud-lite-test-` names.

### Step C: Run Integration Test
```
cargo test --test integration {service_name} -- --ignored --test-threads=1 --nocapture
```
Fix failures and re-run.

### Step D: Write Unit Tests
Encode proven behavior with MockClient. Every test verifies actual data — never just `is_ok()`.

### Step E: Run Full Test Suite
```
cargo test --lib
```

### Step F: Commit
```
feat: add {service_name} {group_name} operations
```

**Repeat Steps A-F for each operation group.**

## Phase 6: Documentation

- **ADD**: Create `docs/{service_name}/` with api.md, operations.md, usage.md (see `.claude/docs/api-doc-template.md`)
- **EXTEND**: Update existing `docs/{service_name}/operations.md` and `usage.md`

**COMMIT**: `docs: {add|update} {service_name} documentation`

## Phase 7: Quality Gate

```
cargo check
cargo clippy -- -D warnings
cargo test --lib
```

Report results. Fix and re-run if anything fails.
