# 22.2: EMR SubscriptionRequiredException — AWS account not subscribed

**Status:** blocked
**Blocked tasks:** 22.2

**Error:**
```
SubscriptionRequiredException: The AWS Access Key Id needs a subscription for the service
```
Confirmed in eu-central-1.

**Root cause:**
The AWS account has not subscribed to Amazon EMR. This is an account-level activation
requirement — EMR must be enabled for the account before any API calls are possible.

**Note about signing:**
EMR's SigV4 signing name must be `"elasticmapreduce"` (not `"emr"`), matching the
`endpointPrefix` from the botocore model. The manifest has been updated with
`service_name = "elasticmapreduce"` to fix the `InvalidSignatureException`.

**What works:**
- Manifest, codegen, scaffold, all quality gates pass
- All API methods compile correctly
- Integration test is written and correct
- Signing name fix confirmed correct (moved past InvalidSignatureException)

**What's needed to fix:**
1. Enable EMR in the AWS account (requires account admin access)
2. Re-run integration tests after subscription is activated
