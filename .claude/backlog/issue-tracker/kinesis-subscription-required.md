# 20.2: Kinesis SubscriptionRequiredException — AWS account not subscribed

**Status:** blocked
**Blocked tasks:** 20.2

**Error:**
```
SubscriptionRequiredException: The AWS Access Key Id needs a subscription for the service
```
Confirmed in all regions: eu-central-1, us-east-1, us-west-2.

**Root cause:**
The AWS account has not subscribed to Amazon Kinesis. This is an account-level activation
requirement — Kinesis must be enabled for the account before any API calls are possible.

**What works:**
- Manifest, codegen, scaffold, all quality gates pass
- All API methods compile correctly
- Integration test is written and correct

**What's needed to fix:**
1. Enable Kinesis in the AWS account (requires account admin access)
2. Re-run integration tests after subscription is activated

**Note:**
Same class of issue as Redshift (OptInRequired/SubscriptionRequiredException). The implementation
is complete and correct; only the integration test is blocked by the account subscription status.
