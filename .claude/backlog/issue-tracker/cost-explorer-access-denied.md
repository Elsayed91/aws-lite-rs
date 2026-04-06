# 19.2: Cost Explorer AccessDeniedException — account-level feature gate

**Status:** blocked
**Blocked tasks:** 19.2

**Error:**
```
ServiceError { code: "AccessDeniedException", message: "User not enabled for cost explorer access", status: 400 }
```

**Root cause:**
AWS Cost Explorer requires an account-level setting to be enabled in the AWS Management Console
under "Cost Management Preferences → Cost Explorer → Enable Cost Explorer". This is a root-account
setting and cannot be enabled via IAM policies, CLI, or API.

**What works:**
- Manifest, codegen, scaffold all pass
- API method is correctly implemented
- Generated types and serde serialization/deserialization are correct
- The AWS CLI returns the same error (`AccessDeniedException: User not enabled for cost explorer access`)

**What's needed to fix:**
1. Enable Cost Explorer in the AWS Management Console (root account required):
   - Go to AWS Billing and Cost Management → Cost Explorer
   - Click "Enable Cost Explorer"
2. Re-run integration tests after Cost Explorer is enabled

**Note:**
The implementation is complete and correct. Only the integration test is blocked by this
account-level restriction. Unit tests can be written based on the known CE response structure.
