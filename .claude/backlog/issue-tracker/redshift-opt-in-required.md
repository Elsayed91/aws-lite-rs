# 11.2: Redshift OptInRequired — AWS account not subscribed

**Status:** blocked
**Blocked tasks:** 11.2, 11.3

**Error:**
```
An error occurred (OptInRequired) when calling the CreateCluster operation: The AWS Access Key Id needs a subscription for the service
```

**Root cause:**
The AWS account used for integration testing has not opted in to the Amazon Redshift service.
This is an account-level configuration issue — the service needs to be explicitly enabled.

**What works:**
- Manifest, codegen, scaffold, and generated tests all pass
- All API methods are scaffolded and compile correctly

**What's needed to fix:**
1. Enable Redshift service in the AWS account (requires account admin)
2. Re-run integration tests after service is enabled

**Workaround:**
Unit tests can still be written based on the query_xml wire format (same as RDS, ElastiCache).
The generated ops tests confirm serialization/deserialization of the request/response types works.
