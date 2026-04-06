# 4.3: Missing ETag/If-Match Header Support

**Status:** open
**Blocked operations:** UpdateDistribution, DeleteDistribution, DeleteOriginAccessControl (CloudFront)

**Root cause:**
CloudFront operations that modify or delete resources require the `If-Match` HTTP header
containing the resource's current ETag (returned in response headers). The AwsHttpClient doesn't:
1. Expose response headers (no way to read the ETag from create/get responses)
2. Support custom request headers on PUT/DELETE operations

**Suggested resolution:**
1. Add `headers()` method to `AwsResponse` that returns response headers
2. Add custom header support to `put()` and `delete()` client methods
3. Or: use `signed_request()` directly with custom headers (already exists internally)
