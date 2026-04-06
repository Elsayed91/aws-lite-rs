# AWS Key Management Service (KMS) API

## Overview

KMS provides cryptographic keys for encrypting and decrypting data, and manages key rotation for compliance. This client covers key discovery, metadata inspection, rotation status checks, and enabling automatic rotation.

## Client Access

```rust
let client = AwsHttpClient::from_default_chain("us-east-1")?;
let kms = client.kms();
```

## Features

- **List keys** — enumerate all KMS keys in the account/region
- **Describe key** — get full metadata: state, manager, spec, creation date
- **Key rotation status** — check whether automatic rotation is enabled (for CIS compliance)
- **Enable key rotation** — turn on annual key material rotation for customer-managed keys

## Wire Format

`json_target` — AWS KMS JSON 1.1 protocol with PascalCase field names. Requests use `X-Amz-Target` header with `TrentService.{Operation}`.

## Types

| Type | Description |
|------|-------------|
| `KeyListEntry` | Key ID and ARN from a list operation |
| `KeyMetadata` | Full key metadata: state, manager, usage, spec, timestamps |
| `ListKeysResponse` | Paginated list of `KeyListEntry` |
| `DescribeKeyResponse` | Wrapper containing `KeyMetadata` |
| `GetKeyRotationStatusResponse` | Rotation state, period, next rotation date |

## Error Handling

Common errors:
- `NotFoundException` — the key ID or ARN does not exist
- `InvalidArnException` — malformed key ARN
- `DisabledException` — operation not allowed on a disabled key
- `KMSInvalidStateException` — key is in wrong state (e.g., pending deletion)
- `UnsupportedOperationException` — operation not supported for this key type (e.g., rotation on asymmetric keys)

## Notes

- `EnableKeyRotation` only works on **symmetric customer-managed keys** — not AWS-managed keys (`KeyManager = AWS`) or asymmetric/HMAC keys
- Key deletion is deferred via `schedule-key-deletion` (7-30 day waiting period) — there is no immediate delete
- `GetKeyRotationStatus` returns `None` for `rotation_period_in_days` if rotation is disabled
