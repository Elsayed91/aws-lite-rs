# 3.2: EC2 XML locationName field mapping not supported by codegen

**Status:** open
**Blocked tasks:** 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 3.10

**Error:**
```
EC2 DescribeVpcs returns 0 VPCs even though the account has a default VPC.
The API call succeeds (200 OK) but all response fields are empty/default.
```

**Root cause:**
EC2 uses botocore `locationName` for XML element names. For example:
- Shape member `Vpcs` → XML element `<vpcSet>` (locationName)
- Shape member `VpcId` → XML element `<vpcId>` (locationName)
- List items use `<item>` (locationName) instead of `<member>`

Our codegen emits `#[serde(rename_all = "PascalCase")]` which expects `<Vpcs>`, `<VpcId>`, etc.
But the real EC2 XML response uses the locationName values: `<vpcSet>`, `<vpcId>`, `<item>`.

Other query_xml services (IAM, STS, CloudWatch, Autoscaling) do NOT use `locationName`,
so their XML field names match PascalCase directly. EC2 is the first service that exposes this gap.

**What works:**
- API calls succeed and return 200 OK
- Empty results parse correctly (empty lists, missing fields → defaults)
- The issue only manifests when there's actual data to parse

**What's needed to fix:**
1. The codegen emitter must detect when a botocore shape member
   has a `locationName` that differs from its PascalCase member name
2. When detected, emit `#[serde(rename = "locationName")]` on that field instead of relying
   on `rename_all = "PascalCase"`
3. The XML parser (`src/xml.rs`) `transform_aws_xml` must also handle
   `<item>` wrapper elements (same as `<member>` unwrapping) since EC2 uses `<item>` for lists

**Scope of impact:**
- ALL EC2 types are affected (~70+ type definitions)
- This requires codegen changes which need user approval
- Other query_xml services are NOT affected (they don't use locationName)

**Suggested resolution:**
1. Modify the AWS plugin to pass locationName from botocore model to IR fields
2. Modify the emitter to emit per-field `#[serde(rename = "...")]` when locationName differs
3. Extend `transform_aws_xml` to also unwrap `<item>` elements (same logic as `<member>`)
4. Re-run codegen for EC2 and verify with integration tests against real data
