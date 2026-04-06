# Integration Test Coverage Matrix

Per-API coverage of required edge cases from `CLAUDE.md` and testing guide.

## Legend

| Symbol | Meaning |
|--------|---------|
| Y | Covered |
| - | Not applicable |
| N | Not yet covered (future work) |

## AWS Coverage Table

| API | CRUD Lifecycle | Error Cases | List Ops | Mock Integration | Property Tests | Snapshot Tests |
|-----|---------------|-------------|----------|-----------------|---------------|---------------|
| **cloudwatch** | Y (get_metric_statistics) | Y (error parsing) | - | N | N | N |
| **logs** | Y (describe_log_groups, list_tags) | N | Y | N | N | N |

### AWS Gaps

The AWS crate has significant test coverage gaps:

| Gap | Description |
|-----|-------------|
| **No mock integration tests** | Multi-step workflows with MockClient not yet written |
| **No integration tests** | Real AWS API testing not yet established |
| **No property tests** | Serde round-trip, wire format property tests missing |
| **No snapshot tests** | Request body serialization snapshots missing |
| **Minimal unit tests** | API layer tests are thin pass-through wrappers |
| **No error case integration tests** | Error response handling only tested in unit tests |
