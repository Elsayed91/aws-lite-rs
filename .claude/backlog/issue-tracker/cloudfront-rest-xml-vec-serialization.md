# 4.3: REST-XML Request Serialization for Vec Types

**Status:** open
**Blocked operations:** CreateDistribution, UpdateDistribution (CloudFront)

**Error:**
```
ServiceError { code: "MalformedInput", message: "Unexpected complex element termination", status: 400 }
```

**Root cause:**
`quick_xml::se::to_string` serializes `Vec<Origin>` using the field name as the element tag:
```xml
<Items><Id>...</Id><DomainName>...</DomainName></Items>
```
But CloudFront REST-XML expects each list item wrapped in its type name:
```xml
<Items><Origin><Id>...</Id><DomainName>...</DomainName></Origin></Items>
```

**What works:**
- Operations without Vec request fields (CreateOriginAccessControl)
- All read operations (ListDistributions, GetDistributionConfig)
- Response deserialization of REST-XML (transform_rest_xml_lists handles this)

**What's needed to fix:**
1. Custom XML serializer for REST-XML request bodies that wraps Vec items in their type tag
2. Or: restructure types to use wrapper structs for list items
