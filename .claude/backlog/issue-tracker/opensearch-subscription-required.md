# 26.2: OpenSearch SubscriptionRequiredException — AWS account not subscribed

**Status:** blocked
**Blocked tasks:** 26.2

**Error:**
```
SubscriptionRequiredException: The AWS Access Key Id needs a subscription for the service
```
Confirmed in eu-central-1 via both AWS CLI (`aws es list-domain-names`) and the library.

**Root cause:**
The AWS account has not subscribed to Amazon OpenSearch Service. This is an account-level
activation requirement — OpenSearch must be enabled for the account before any API calls are
possible. Neither `aws es` nor `aws opensearch` CLI commands work.

**What works:**
- Manifest, codegen, scaffold, all quality gates pass (413 unit tests)
- All API methods compile correctly (list_domain_names, describe_domain, delete_domain)
- Unit tests encode the wire format based on PascalCase JSON keys and ARN serde_rename
- The implementation is complete and correct

**What's needed to fix:**
1. Enable Amazon OpenSearch Service in the AWS account (requires account admin access)
2. Re-run integration tests after subscription is activated

**Note:**
Same class of issue as Kinesis (20.2) and EMR (22.2). The implementation is complete and
correct; only the integration test is blocked by the account subscription status.
