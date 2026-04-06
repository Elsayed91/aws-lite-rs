//! AWS XML response parsing utilities.
//!
//! Handles the common AWS XML response format where the actual result
//! is nested inside a `<{Action}Response><{Action}Result>` wrapper,
//! and unwraps `<member>` wrapper elements used for list items.

use serde::de::DeserializeOwned;

/// Parse an AWS XML response body, handling the common wrapper elements.
///
/// AWS Query/XML responses follow this pattern:
/// ```xml
/// <GetMetricStatisticsResponse xmlns="...">
///   <GetMetricStatisticsResult>
///     <!-- actual data here -->
///   </GetMetricStatisticsResult>
///   <ResponseMetadata>
///     <RequestId>...</RequestId>
///   </ResponseMetadata>
/// </GetMetricStatisticsResponse>
/// ```
///
/// Lists use `<member>` wrappers which are unwrapped automatically:
/// ```xml
/// <Datapoints>
///   <member><Timestamp>...</Timestamp></member>
///   <member><Timestamp>...</Timestamp></member>
/// </Datapoints>
/// ```
///
/// This function extracts the inner result element, unwraps `<member>`
/// elements, and deserializes the result.
pub fn parse_xml_response<T: DeserializeOwned>(body: &str) -> Result<T, String> {
    // Strip xmlns attributes first
    let body = strip_xmlns(body);

    // Try to find and extract the *Result element content
    // The pattern is: <ActionResult>...</ActionResult>
    let xml = if let Some(result_content) = extract_result_element(&body) {
        result_content
    } else if let Some(response_content) = extract_response_content(&body) {
        // EC2-style response: no <Result> wrapper, data is directly in <Response>
        // with extra elements like <requestId> that we need to strip
        response_content
    } else {
        body.to_string()
    };

    // Transform AWS XML patterns (member/item unwrapping, empty element removal)
    let xml = transform_aws_xml(&xml);

    quick_xml::de::from_str(&xml).map_err(|e| format!("XML parse error: {e}"))
}

/// Parse a REST-XML response body directly.
///
/// REST-XML services (CloudFront, S3) return the payload shape directly
/// as the root element without `<ActionResult>` wrappers. Lists use a
/// container-child pattern (e.g. `<Items><Origin>...</Origin></Items>`)
/// instead of `<member>` wrappers. This function:
/// 1. Strips xmlns attributes
/// 2. Transforms container-child list patterns for quick_xml
pub fn parse_rest_xml_response<T: DeserializeOwned>(body: &str) -> Result<T, String> {
    let xml = strip_xmlns(body);
    let xml = transform_rest_xml_lists(&xml);
    quick_xml::de::from_str(&xml).map_err(|e| format!("XML parse error: {e}"))
}

/// Transform REST-XML list patterns for quick_xml deserialization.
///
/// REST-XML uses a container-child pattern for lists where a wrapper
/// element contains uniform child elements:
/// ```xml
/// <Items>
///   <DistributionSummary>...</DistributionSummary>
///   <DistributionSummary>...</DistributionSummary>
/// </Items>
/// ```
///
/// Quick_xml's serde expects repeated elements with the parent field name:
/// ```xml
/// <Items>...</Items>
/// <Items>...</Items>
/// ```
///
/// This unwraps any container element whose children are all the same
/// tag type (e.g. `<Buckets><Bucket>...</Bucket></Buckets>` becomes
/// `<Buckets>...</Buckets><Buckets>...</Buckets>`).
///
/// The root element is NOT unwrapped (quick_xml uses it as the type wrapper).
/// Also removes empty leaf elements (self-closing or empty content)
/// to avoid phantom Vec entries in quick_xml.
fn transform_rest_xml_lists(xml: &str) -> String {
    // Find the root element and process only its inner content
    let stripped = xml.trim();

    // Skip XML declaration if present
    let stripped = if stripped.starts_with("<?xml") {
        if let Some(end) = stripped.find("?>") {
            stripped[end + 2..].trim()
        } else {
            stripped
        }
    } else {
        stripped
    };

    if !stripped.starts_with('<') {
        return xml.to_string();
    }

    // Find the root opening tag
    let Some(gt_pos) = stripped.find('>') else {
        return xml.to_string();
    };
    if stripped.as_bytes()[gt_pos - 1] == b'/' {
        // Self-closing root — nothing to transform
        return xml.to_string();
    }

    let name_start = 1;
    let tag_region = &stripped[name_start..gt_pos];
    let root_tag_name = tag_region
        .split_whitespace()
        .next()
        .unwrap_or(tag_region)
        .trim();

    let close_tag = format!("</{root_tag_name}>");
    let Some(close_pos) = stripped.rfind(&close_tag) else {
        return xml.to_string();
    };

    let inner = &stripped[gt_pos + 1..close_pos];
    let transformed_inner = transform_rest_xml_inner(inner);

    format!("<{root_tag_name}>{transformed_inner}{close_tag}")
}

/// Transform the inner content of a REST-XML element (not the root).
///
/// Processes all nested elements, unwrapping list containers
/// (elements whose children are all the same tag type).
fn transform_rest_xml_inner(xml: &str) -> String {
    let mut result = String::with_capacity(xml.len());
    let mut pos = 0;
    let bytes = xml.as_bytes();

    while pos < bytes.len() {
        if bytes[pos] != b'<' {
            result.push(bytes[pos] as char);
            pos += 1;
            continue;
        }

        // Skip comments, PIs, CDATA
        if xml[pos..].starts_with("<!--")
            || xml[pos..].starts_with("<?")
            || xml[pos..].starts_with("<![")
        {
            if let Some(end) = xml[pos..].find('>') {
                result.push_str(&xml[pos..pos + end + 1]);
                pos += end + 1;
            } else {
                result.push_str(&xml[pos..]);
                break;
            }
            continue;
        }

        // Closing tag — pass through
        if xml[pos..].starts_with("</") {
            if let Some(end) = xml[pos..].find('>') {
                result.push_str(&xml[pos..pos + end + 1]);
                pos += end + 1;
            } else {
                result.push_str(&xml[pos..]);
                break;
            }
            continue;
        }

        // Opening or self-closing tag
        let Some(gt_pos) = xml[pos..].find('>') else {
            result.push_str(&xml[pos..]);
            break;
        };
        let gt_abs = pos + gt_pos;
        let is_self_closing = bytes[gt_abs - 1] == b'/';

        let name_start = pos + 1;
        let tag_region = if is_self_closing {
            &xml[name_start..gt_abs - 1]
        } else {
            &xml[name_start..gt_abs]
        };
        let tag_name = tag_region
            .split_whitespace()
            .next()
            .unwrap_or(tag_region)
            .trim();

        if is_self_closing {
            // Drop self-closing list-container tags (e.g. <HealthChecks/>) entirely.
            //
            // When a list container is self-closing (empty), quick_xml would try to
            // deserialize it as a struct item rather than an empty Vec, triggering
            // "missing field" errors. Dropping it lets #[serde(default)] yield
            // an empty Vec correctly.
            //
            // Heuristic: plural tags (end with lowercase 's' but not 'ss'), or known
            // list markers (Items, *List) are treated as containers and dropped.
            // Non-container self-closing tags (leaf nodes like <Enabled/>) are
            // passed through unchanged.
            let t_bytes = tag_name.as_bytes();
            let n = t_bytes.len();
            let is_likely_container = (n >= 2
                && t_bytes[n - 1] == b's'
                && t_bytes[n - 2] != b's'
                && t_bytes[n - 1].is_ascii_lowercase())
                || tag_name == "Items"
                || tag_name.ends_with("List");
            if !is_likely_container {
                result.push_str(&xml[pos..gt_abs + 1]);
            }
            // Drop empty list containers — Vec fields use #[serde(default)] = vec![]
            pos = gt_abs + 1;
            continue;
        }

        // Opening tag — find matching close
        let close_tag = format!("</{tag_name}>");
        let Some(close_offset) = find_matching_close_raw(&xml[gt_abs + 1..], tag_name) else {
            result.push_str(&xml[pos..gt_abs + 1]);
            pos = gt_abs + 1;
            continue;
        };
        let close_abs = gt_abs + 1 + close_offset;
        let inner = &xml[gt_abs + 1..close_abs];

        // Handle empty containers.
        //
        // For list containers (plural tags like <HealthChecks></HealthChecks> or
        // <Items></Items>), drop the element entirely so #[serde(default)] yields
        // an empty Vec. Emitting an empty container causes quick_xml to attempt
        // deserializing a zero-content element as a struct, triggering "missing field".
        //
        // For non-container elements (leaf values, optional structs), pass through
        // unchanged so serde can handle them correctly.
        if inner.trim().is_empty() {
            let t_bytes = tag_name.as_bytes();
            let n = t_bytes.len();
            let is_container = (n >= 2
                && t_bytes[n - 1] == b's'
                && t_bytes[n - 2] != b's'
                && t_bytes[n - 1].is_ascii_lowercase())
                || tag_name == "Items"
                || tag_name.ends_with("List");
            if !is_container {
                // Keep non-container empty elements (e.g. <Comment/> → absent → None)
                result.push_str(&xml[pos..close_abs + close_tag.len()]);
            }
            // Drop empty list containers — Vec fields use #[serde(default)] = vec![]
            pos = close_abs + close_tag.len();
            continue;
        }

        // Check if this is a list container that should be unwrapped.
        //
        // REST-XML wraps list elements in containers:
        //   <Buckets><Bucket>...</Bucket></Buckets>
        //   <Items><DistributionSummary>...</DistributionSummary></Items>
        //   <AccessControlList><Grant>...</Grant></AccessControlList>
        //
        // A container is unwrapped when children are uniform-tagged AND:
        // - There are 2+ children (definitely a list), OR
        // - The container is a known plural form of the child tag
        //   (e.g. Buckets→Bucket, Grants→Grant)
        if let Some(child_tag) = get_uniform_child_tag(inner)
            && child_tag != tag_name
        {
            let child_count = count_direct_children(inner, &child_tag);
            let is_plural_of_child = tag_name
                .strip_suffix('s')
                .is_some_and(|singular| singular == child_tag)
                || tag_name
                    .strip_suffix("es")
                    .is_some_and(|singular| singular == child_tag);
            let is_known_container = tag_name == "Items" || tag_name.ends_with("List");
            let is_list = child_count >= 2 || is_plural_of_child || is_known_container;
            if is_list {
                for child_content in extract_child_contents(inner, &child_tag) {
                    result.push('<');
                    result.push_str(tag_name);
                    result.push('>');
                    result.push_str(&transform_rest_xml_inner(child_content));
                    result.push_str(&close_tag);
                }
                pos = close_abs + close_tag.len();
                continue;
            }
        }

        // Regular element — pass through the opening tag, transform inner content
        result.push_str(&xml[pos..gt_abs + 1]);
        pos = gt_abs + 1;
    }

    result
}

/// Find the matching close tag position at nesting level 0 in a raw XML string.
/// Returns the offset from the start of `xml` to the start of the closing tag.
fn find_matching_close_raw(xml: &str, tag_name: &str) -> Option<usize> {
    let open_tag = format!("<{tag_name}");
    let close_tag = format!("</{tag_name}>");
    let mut depth = 1i32;
    let mut pos = 0;

    while depth > 0 && pos < xml.len() {
        let next_close = xml[pos..].find(&close_tag).map(|p| p + pos);
        let next_open = xml[pos..].find(&open_tag).and_then(|p| {
            let abs = p + pos;
            let after = abs + open_tag.len();
            // Must be followed by > or space (not a different tag like <BucketArn>)
            if after < xml.len()
                && (xml.as_bytes()[after] == b'>' || xml.as_bytes()[after].is_ascii_whitespace())
            {
                Some(abs)
            } else {
                // Skip this false match and search further
                find_next_real_open(xml, after, &open_tag)
            }
        });

        match (next_open, next_close) {
            (_, Some(cp)) if next_open.is_none() || cp < next_open.unwrap() => {
                depth -= 1;
                if depth == 0 {
                    return Some(cp);
                }
                pos = cp + close_tag.len();
            }
            (Some(op), _) => {
                depth += 1;
                let after = op + open_tag.len();
                pos = xml[after..].find('>').map_or(xml.len(), |p| after + p + 1);
            }
            _ => break,
        }
    }
    None
}

/// Find the next real opening tag match (not a prefix match like <Bucket matching <BucketArn>).
fn find_next_real_open(xml: &str, start: usize, open_tag: &str) -> Option<usize> {
    let mut pos = start;
    while let Some(p) = xml[pos..].find(open_tag) {
        let abs = p + pos;
        let after = abs + open_tag.len();
        if after < xml.len()
            && (xml.as_bytes()[after] == b'>' || xml.as_bytes()[after].is_ascii_whitespace())
        {
            return Some(abs);
        }
        pos = abs + 1;
    }
    None
}

/// Count the number of direct child elements with a given tag name.
fn count_direct_children(inner: &str, tag_name: &str) -> usize {
    let open_tag = format!("<{tag_name}");
    let mut count = 0;
    let mut remaining = inner;

    while let Some(pos) = remaining.find(&open_tag) {
        let after = pos + open_tag.len();
        if after < remaining.len() {
            let next_byte = remaining.as_bytes()[after];
            if next_byte == b'>' || next_byte == b' ' || next_byte == b'/' {
                count += 1;
                // Skip past this element
                let close_tag = format!("</{tag_name}>");
                if remaining.as_bytes()[after] == b'/'
                    && after + 1 < remaining.len()
                    && remaining.as_bytes()[after + 1] == b'>'
                {
                    remaining = &remaining[after + 2..];
                } else if let Some(close_pos) = remaining[after..].find(&close_tag) {
                    remaining = &remaining[after + close_pos + close_tag.len()..];
                } else {
                    break;
                }
            } else {
                remaining = &remaining[after..];
            }
        } else {
            break;
        }
    }

    count
}

/// Find the position of the nesting-aware matching close tag within `xml`.
///
/// Searches for `</tag_name>` starting from the beginning of `xml`, tracking
/// nesting depth so that same-named nested elements are handled correctly.
/// Returns the byte offset of the start of the matching `</tag_name>`.
fn find_matching_close(xml: &str, tag_name: &str) -> Option<usize> {
    let open_tag = format!("<{tag_name}");
    let close_tag = format!("</{tag_name}>");
    let mut depth = 1usize;
    let mut pos = 0;

    while pos < xml.len() {
        // Find the next tag-like thing (open or close) for this tag_name
        let next_close = xml[pos..].find(&close_tag).map(|i| i + pos);
        let next_open = xml[pos..].find(&open_tag).and_then(|i| {
            let abs = i + pos;
            // Verify it's actually an opening tag: next char must be '>' or whitespace
            let after = abs + open_tag.len();
            if after < xml.len() {
                let ch = xml.as_bytes()[after];
                if ch == b'>' || ch == b' ' || ch == b'/' || ch == b'\t' || ch == b'\n' {
                    return Some(abs);
                }
            }
            None
        });

        match (next_open, next_close) {
            (Some(o), Some(c)) if o < c => {
                // Check if this open tag is self-closing
                if let Some(gt) = xml[o..].find('>') {
                    let gt_abs = o + gt;
                    if gt_abs > 0 && xml.as_bytes()[gt_abs - 1] == b'/' {
                        // Self-closing, don't change depth
                    } else {
                        depth += 1;
                    }
                    pos = gt_abs + 1;
                } else {
                    break;
                }
            }
            (_, Some(c)) => {
                depth -= 1;
                if depth == 0 {
                    return Some(c);
                }
                pos = c + close_tag.len();
            }
            _ => break,
        }
    }

    None
}

/// If all direct child elements share the same tag name, return it.
fn get_uniform_child_tag(inner: &str) -> Option<String> {
    let trimmed = inner.trim();
    if trimmed.is_empty() || !trimmed.starts_with('<') {
        return None;
    }

    let mut first_tag: Option<String> = None;
    let mut remaining = trimmed;

    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            break;
        }
        if !remaining.starts_with('<') {
            return None;
        }

        let gt = remaining.find('>')?;
        let tag_content = &remaining[1..gt];
        let is_self_closing = tag_content.ends_with('/');
        let tag_region = if is_self_closing {
            &tag_content[..tag_content.len() - 1]
        } else {
            tag_content
        };
        let tag_name = tag_region.split_whitespace().next()?.trim();

        if tag_name.starts_with('/') {
            return None;
        }

        match &first_tag {
            None => first_tag = Some(tag_name.to_string()),
            Some(expected) => {
                if tag_name != expected.as_str() {
                    return None;
                }
            }
        }

        if is_self_closing {
            remaining = &remaining[gt + 1..];
        } else {
            let close_tag = format!("</{tag_name}>");
            let after_open = gt + 1;
            let close_offset = find_matching_close(&remaining[after_open..], tag_name)?;
            remaining = &remaining[after_open + close_offset + close_tag.len()..];
        }
    }

    first_tag
}

/// Extract the content of each child element with a given tag name.
fn extract_child_contents<'a>(xml: &'a str, tag_name: &str) -> Vec<&'a str> {
    let open_tag = format!("<{tag_name}>");
    let close_tag = format!("</{tag_name}>");
    let mut contents = Vec::new();
    let mut remaining = xml;

    while let Some(start) = remaining.find(&open_tag) {
        let content_start = start + open_tag.len();
        if let Some(end) = find_matching_close(&remaining[content_start..], tag_name) {
            contents.push(&remaining[content_start..content_start + end]);
            remaining = &remaining[content_start + end + close_tag.len()..];
        } else {
            break;
        }
    }

    contents
}

/// Inject an `xmlns="namespace"` attribute into the root element of an XML string.
///
/// Some REST-XML services (e.g. Route 53) require the XML namespace to be declared
/// in the root element of request bodies. `quick_xml::se::to_string` does not add
/// the namespace automatically, so this function inserts it after the root tag name.
///
/// Example:
/// ```
/// // Input:  <CreateHealthCheckRequest><CallerReference>...</CallerReference></CreateHealthCheckRequest>
/// // Output: <CreateHealthCheckRequest xmlns="https://route53.amazonaws.com/doc/2013-04-01/"><CallerReference>...</CallerReference></CreateHealthCheckRequest>
/// ```
pub fn inject_xml_namespace(xml: &str, namespace: &str) -> String {
    let trimmed = xml.trim_start();
    // Find the root opening tag: starts with '<', skip comments/PIs
    if !trimmed.starts_with('<') {
        return xml.to_string();
    }
    // Find the end of the root tag name (next space or '>')
    let tag_end = trimmed[1..].find(['>', ' ', '/']);
    if let Some(pos) = tag_end {
        let insert_at = 1 + pos; // position after the tag name
        format!(
            "{} xmlns=\"{namespace}\"{}",
            &trimmed[..insert_at],
            &trimmed[insert_at..]
        )
    } else {
        xml.to_string()
    }
}

/// Remove xmlns="..." attributes from XML elements.
fn strip_xmlns(xml: &str) -> String {
    let mut result = String::with_capacity(xml.len());
    let mut i = 0;
    let bytes = xml.as_bytes();

    while i < bytes.len() {
        // Look for xmlns pattern: space(s) followed by xmlns
        if i + 6 < bytes.len() && bytes[i].is_ascii_whitespace() && &xml[i + 1..i + 6] == "xmlns" {
            // Check if this is xmlns= or xmlns:prefix=
            let attr_start = i;
            let mut j = i + 6;
            if j < bytes.len() && bytes[j] == b':' {
                // xmlns:prefix — skip the prefix name
                j += 1;
                while j < bytes.len() && bytes[j] != b'=' {
                    j += 1;
                }
            }
            if j < bytes.len() && bytes[j] == b'=' {
                j += 1;
                // Skip the quoted value
                if j < bytes.len() && bytes[j] == b'"' {
                    j += 1;
                    while j < bytes.len() && bytes[j] != b'"' {
                        j += 1;
                    }
                    if j < bytes.len() {
                        j += 1; // skip closing quote
                    }
                    // Successfully matched xmlns[:]...="..." — skip it
                    i = j;
                    continue;
                }
            }
            // Didn't match full pattern, keep the character
            result.push(bytes[attr_start] as char);
            i = attr_start + 1;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Extract the `*Result` element (including its tags) from an AWS XML response.
///
/// Returns the full `<ActionResult>...</ActionResult>` element, which quick_xml
/// can deserialize directly (it ignores the root element name).
fn extract_result_element(xml: &str) -> Option<String> {
    // Find a tag ending in "Result>"
    let result_start_pattern = "Result>";
    let pos = xml.find(result_start_pattern)?;

    // Walk backwards to find the opening <
    let tag_start = xml[..pos].rfind('<')?;
    let tag_name = &xml[tag_start + 1..pos + "Result".len()];

    // Now find the complete element including its tags
    let open_tag = format!("<{tag_name}>");
    let close_tag = format!("</{tag_name}>");

    let element_start = xml.find(&open_tag)?;
    let element_end = xml.rfind(&close_tag)? + close_tag.len();

    if element_start < element_end {
        Some(xml[element_start..element_end].to_string())
    } else {
        None
    }
}

/// Extract the inner content of an `*Response` element for EC2-style responses.
///
/// EC2 responses don't use a `<Result>` wrapper. Instead, the data is directly
/// inside `<ActionResponse>` alongside a `<requestId>` element:
/// ```xml
/// <DescribeVolumesResponse>
///   <requestId>abc-123</requestId>
///   <volumeSet>...</volumeSet>
///   <nextToken>...</nextToken>
/// </DescribeVolumesResponse>
/// ```
///
/// This function extracts the inner content (minus `<requestId>`) and wraps it
/// in a synthetic root element for deserialization.
fn extract_response_content(xml: &str) -> Option<String> {
    // Find the *Response element
    let response_pattern = "Response>";
    let pos = xml.find(response_pattern)?;

    // Walk backwards to find the opening <
    let tag_start = xml[..pos].rfind('<')?;
    let tag_name = &xml[tag_start + 1..pos + "Response".len()];

    let open_tag_prefix = format!("<{tag_name}");
    let close_tag = format!("</{tag_name}>");

    // Find opening tag (may have attributes)
    let element_start = xml.find(&open_tag_prefix)?;
    let open_end = xml[element_start..].find('>')? + element_start + 1;

    // Find closing tag
    let element_end = xml.rfind(&close_tag)?;

    if open_end >= element_end {
        return None;
    }

    let inner = &xml[open_end..element_end];

    // Strip <requestId>...</requestId> element
    let mut cleaned = String::from(inner);
    if let Some(req_start) = cleaned.find("<requestId>")
        && let Some(req_end) = cleaned.find("</requestId>")
    {
        cleaned = format!(
            "{}{}",
            &cleaned[..req_start],
            &cleaned[req_end + "</requestId>".len()..]
        );
    }

    // Wrap in a synthetic root for deserialization
    Some(format!("<Root>{cleaned}</Root>"))
}

/// Transform AWS XML for correct quick_xml serde deserialization.
///
/// Handles two AWS XML patterns that quick_xml doesn't natively support:
///
/// **1. `<member>` wrapper elements** — AWS wraps list items in `<member>` tags:
/// ```xml
/// <Datapoints>
///   <member><Timestamp>2024-01-01</Timestamp></member>
///   <member><Timestamp>2024-01-02</Timestamp></member>
/// </Datapoints>
/// ```
/// Transformed to repeated elements (what quick_xml expects for `Vec<T>`):
/// ```xml
/// <Datapoints><Timestamp>2024-01-01</Timestamp></Datapoints>
/// <Datapoints><Timestamp>2024-01-02</Timestamp></Datapoints>
/// ```
///
/// **2. Empty leaf elements** — `<Datapoints/>` or `<Datapoints></Datapoints>`
/// that quick_xml incorrectly deserializes as Vec with one default element.
/// Removed so `#[serde(default)]` provides the correct empty Vec/None.
fn transform_aws_xml(xml: &str) -> String {
    let mut result = String::with_capacity(xml.len());
    let mut pos = 0;
    let bytes = xml.as_bytes();

    while pos < bytes.len() {
        if bytes[pos] != b'<' {
            result.push(bytes[pos] as char);
            pos += 1;
            continue;
        }

        // We're at a '<'. Determine what kind of tag this is.
        let tag_start = pos;

        // Skip comments, PIs, CDATA
        if xml[pos..].starts_with("<!--")
            || xml[pos..].starts_with("<?")
            || xml[pos..].starts_with("<![")
        {
            if let Some(end) = xml[pos..].find('>') {
                result.push_str(&xml[pos..pos + end + 1]);
                pos += end + 1;
            } else {
                result.push_str(&xml[pos..]);
                break;
            }
            continue;
        }

        // Closing tag — pass through
        if xml[pos..].starts_with("</") {
            if let Some(end) = xml[pos..].find('>') {
                result.push_str(&xml[pos..pos + end + 1]);
                pos += end + 1;
            } else {
                result.push_str(&xml[pos..]);
                break;
            }
            continue;
        }

        // Opening or self-closing tag
        let Some(gt_pos) = xml[pos..].find('>') else {
            result.push_str(&xml[pos..]);
            break;
        };
        let gt_abs = pos + gt_pos;
        let is_self_closing = bytes[gt_abs - 1] == b'/';

        // Extract the tag name
        let name_start = pos + 1;
        let tag_region = if is_self_closing {
            &xml[name_start..gt_abs - 1]
        } else {
            &xml[name_start..gt_abs]
        };
        let tag_name = tag_region
            .split_whitespace()
            .next()
            .unwrap_or(tag_region)
            .trim();

        if is_self_closing {
            // Self-closing: <Name/> — check if it's an empty leaf
            // For AWS XML, empty leaf elements cause phantom Vec entries.
            // Removing them is safe: Option fields get None, Vec fields get empty Vec
            // via #[serde(default)], and text fields would never be self-closing in
            // valid AWS responses.
            //
            // EXCEPTION: preserve the root element (the *Result wrapper) since
            // quick_xml needs at least the root tag to deserialize.
            if tag_start == 0 {
                result.push_str(&xml[tag_start..gt_abs + 1]);
            }
            // Otherwise, skip it (removes empty element)
            pos = gt_abs + 1;
            continue;
        }

        // Opening tag: <Name> or <Name attr="val">
        let close_tag = format!("</{tag_name}>");
        let Some(close_offset) = xml[gt_abs + 1..].find(&close_tag) else {
            // No matching close tag — pass through
            result.push_str(&xml[tag_start..gt_abs + 1]);
            pos = gt_abs + 1;
            continue;
        };
        let close_abs = gt_abs + 1 + close_offset;
        let inner = &xml[gt_abs + 1..close_abs];

        // Check if DIRECT children are <entry> elements (AWS map format).
        // <Parent><entry><key>K</key><value>V</value></entry></Parent>
        // is transformed to <Parent><K>V</K></Parent> for HashMap deserialization.
        if has_direct_entry_children(inner) {
            result.push('<');
            result.push_str(tag_name);
            result.push('>');
            for (key, value) in extract_entry_pairs(inner) {
                result.push('<');
                result.push_str(&key);
                result.push('>');
                result.push_str(&value);
                result.push_str("</");
                result.push_str(&key);
                result.push('>');
            }
            result.push_str(&close_tag);
            pos = close_abs + close_tag.len();
            continue;
        }

        // Check if DIRECT children are <member> elements (not nested deeper).
        // A direct child <member> means the first non-whitespace content after
        // the opening tag starts with "<member>".
        if has_direct_member_children(inner) {
            // Transform: <Parent><member>c1</member><member>c2</member></Parent>
            // becomes: <Parent>c1</Parent><Parent>c2</Parent>
            for member_content in extract_member_contents(inner) {
                result.push('<');
                result.push_str(tag_name);
                result.push('>');
                // Recursively transform member content (handles nested lists)
                result.push_str(&transform_aws_xml(member_content));
                result.push_str(&close_tag);
            }
            pos = close_abs + close_tag.len();
            continue;
        }

        // Check for RDS-style container-child list pattern (no <member> wrappers).
        // Example: <DBInstances><DBInstance>...</DBInstance><DBInstance>...</DBInstance></DBInstances>
        // Transform to: <DBInstances>...</DBInstances><DBInstances>...</DBInstances>
        //
        // Only unwrap when it's clearly a list, not a wrapper struct:
        // - 2+ children of the same type (definitely a list)
        // - Container is a known plural form of the child (e.g. DBInstances -> DBInstance)
        if !inner.trim().is_empty()
            && let Some(child_tag) = get_uniform_child_tag(inner)
            && child_tag != "member"
            && child_tag != "item"
            && child_tag != "entry"
        {
            let child_count = count_direct_children(inner, &child_tag);
            let is_plural_of_child = tag_name
                .strip_suffix('s')
                .is_some_and(|singular| singular == child_tag)
                || tag_name
                    .strip_suffix("es")
                    .is_some_and(|singular| singular == child_tag);
            let is_known_container = tag_name.ends_with("List");
            let is_list = child_count >= 2 || is_plural_of_child || is_known_container;
            if is_list {
                for child_content in extract_child_contents(inner, &child_tag) {
                    result.push('<');
                    result.push_str(tag_name);
                    result.push('>');
                    result.push_str(&transform_aws_xml(child_content));
                    result.push_str(&close_tag);
                }
                pos = close_abs + close_tag.len();
                continue;
            }
        }

        // Check if this is an empty element (whitespace-only content)
        if inner.trim().is_empty() {
            // Same logic as self-closing: remove empty leaf elements.
            // This prevents phantom Vec entries from <X></X> in quick_xml.
            // String fields that might be empty should use #[serde(default)]
            // or be marked optional in the manifest.
            if tag_start == 0 {
                result.push_str(&xml[tag_start..close_abs + close_tag.len()]);
            }
            pos = close_abs + close_tag.len();
            continue;
        }

        // Regular element with content — pass through the opening tag
        result.push_str(&xml[tag_start..gt_abs + 1]);
        pos = gt_abs + 1;
    }

    result
}

/// Check if the direct children of an element are `<entry>` tags (AWS map format).
fn has_direct_entry_children(inner: &str) -> bool {
    let trimmed = inner.trim_start();
    trimmed.starts_with("<entry>")
}

/// Extract key-value pairs from `<entry><key>K</key><value>V</value></entry>` elements.
fn extract_entry_pairs(xml: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    let mut remaining = xml;

    while let Some(start) = remaining.find("<entry>") {
        let content_start = start + "<entry>".len();
        let Some(end) = remaining[content_start..].find("</entry>") else {
            break;
        };
        let entry_inner = &remaining[content_start..content_start + end];

        // Extract <key>...</key> and <value>...</value>
        if let Some(key) = extract_simple_tag_content(entry_inner, "key")
            && let Some(value) = extract_simple_tag_content(entry_inner, "value")
        {
            pairs.push((key.to_string(), value.to_string()));
        }

        remaining = &remaining[content_start + end + "</entry>".len()..];
    }

    pairs
}

/// Extract the text content of a simple tag like `<key>text</key>`.
fn extract_simple_tag_content<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(&xml[start..end])
}

/// Check if the direct children of an element are `<member>` or `<item>` tags.
///
/// Returns true if the first non-whitespace content starts with `<member>` or `<item>`.
/// This distinguishes `<Datapoints><member>...</member></Datapoints>` (direct)
/// from `<Result><Label>x</Label><Datapoints><member>...</member></Datapoints></Result>` (nested).
///
/// EC2 uses `<item>` while IAM/CloudWatch/STS use `<member>`.
fn has_direct_member_children(inner: &str) -> bool {
    let trimmed = inner.trim_start();
    trimmed.starts_with("<member>") || trimmed.starts_with("<item>")
}

/// Detect which wrapper tag is used: `<member>` or `<item>`.
fn list_wrapper_tag(inner: &str) -> &'static str {
    let trimmed = inner.trim_start();
    if trimmed.starts_with("<item>") {
        "item"
    } else {
        "member"
    }
}

/// Extract the content of each `<member>...</member>` or `<item>...</item>` element.
fn extract_member_contents(xml: &str) -> Vec<&str> {
    let tag = list_wrapper_tag(xml);
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut contents = Vec::new();
    let mut remaining = xml;

    while let Some(start) = remaining.find(open.as_str()) {
        let content_start = start + open.len();
        if let Some(end) = find_matching_close(&remaining[content_start..], tag) {
            contents.push(&remaining[content_start..content_start + end]);
            remaining = &remaining[content_start + end + close.len()..];
        } else {
            break;
        }
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct TestResult {
        label: Option<String>,
    }

    #[test]
    fn parse_wrapped_xml_response() {
        let xml = r#"<GetMetricStatisticsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
            <GetMetricStatisticsResult>
                <Label>CPUUtilization</Label>
            </GetMetricStatisticsResult>
            <ResponseMetadata>
                <RequestId>abc-123</RequestId>
            </ResponseMetadata>
        </GetMetricStatisticsResponse>"#;

        let result: TestResult = parse_xml_response(xml).unwrap();
        assert_eq!(result.label, Some("CPUUtilization".into()));
    }

    #[test]
    fn parse_empty_result() {
        let xml = r#"<GetMetricStatisticsResponse>
            <GetMetricStatisticsResult>
            </GetMetricStatisticsResult>
        </GetMetricStatisticsResponse>"#;

        let result: TestResult = parse_xml_response(xml).unwrap();
        assert_eq!(result.label, None);
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct TestWithList {
        label: Option<String>,
        #[serde(default)]
        datapoints: Vec<TestDatapoint>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct TestDatapoint {
        timestamp: Option<String>,
        average: Option<f64>,
    }

    #[test]
    fn parse_empty_list_self_closing() {
        let xml = r#"<GetMetricStatisticsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
            <GetMetricStatisticsResult>
                <Datapoints/>
                <Label>CPUUtilization</Label>
            </GetMetricStatisticsResult>
        </GetMetricStatisticsResponse>"#;

        let result: TestWithList = parse_xml_response(xml).unwrap();
        assert_eq!(result.label, Some("CPUUtilization".into()));
        assert!(
            result.datapoints.is_empty(),
            "self-closing <Datapoints/> should be empty vec, got {:?}",
            result.datapoints
        );
    }

    #[test]
    fn parse_empty_list_open_close() {
        // Test that <Datapoints></Datapoints> also produces empty vec
        let xml = r#"<GetMetricStatisticsResponse>
            <GetMetricStatisticsResult>
                <Datapoints></Datapoints>
                <Label>CPUUtilization</Label>
            </GetMetricStatisticsResult>
        </GetMetricStatisticsResponse>"#;

        let result: TestWithList = parse_xml_response(xml).unwrap();
        assert_eq!(result.label, Some("CPUUtilization".into()));
        assert!(
            result.datapoints.is_empty(),
            "empty <Datapoints></Datapoints> should be empty vec, got {:?}",
            result.datapoints
        );
    }

    #[test]
    fn parse_list_with_member_elements() {
        let xml = r#"<GetMetricStatisticsResponse>
            <GetMetricStatisticsResult>
                <Label>CPUUtilization</Label>
                <Datapoints>
                    <member>
                        <Timestamp>2024-01-01T00:00:00Z</Timestamp>
                        <Average>42.5</Average>
                    </member>
                    <member>
                        <Timestamp>2024-01-01T00:05:00Z</Timestamp>
                        <Average>38.2</Average>
                    </member>
                </Datapoints>
            </GetMetricStatisticsResult>
        </GetMetricStatisticsResponse>"#;

        let result: TestWithList = parse_xml_response(xml).unwrap();
        assert_eq!(result.label, Some("CPUUtilization".into()));
        assert_eq!(result.datapoints.len(), 2);
        assert_eq!(
            result.datapoints[0].timestamp,
            Some("2024-01-01T00:00:00Z".into())
        );
        assert_eq!(result.datapoints[0].average, Some(42.5));
    }

    // =====================================================================
    // REST-XML parsing tests (Route53, CloudFront, S3 style)
    // =====================================================================

    #[derive(Debug, Deserialize, Default, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct RestHealthCheck {
        id: String,
    }

    #[derive(Debug, Deserialize, Default)]
    #[serde(rename_all = "PascalCase")]
    struct RestListHealthChecksResponse {
        #[serde(default)]
        health_checks: Vec<RestHealthCheck>,
        #[serde(default)]
        is_truncated: bool,
        max_items: String,
    }

    #[test]
    fn parse_rest_xml_empty_list_self_closing() {
        // Route53 returns <HealthChecks/> when list is empty
        let xml = r#"<?xml version="1.0"?>
<ListHealthChecksResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/"><HealthChecks/><IsTruncated>false</IsTruncated><MaxItems>100</MaxItems></ListHealthChecksResponse>"#;
        let result: RestListHealthChecksResponse = parse_rest_xml_response(xml).unwrap();
        assert!(
            result.health_checks.is_empty(),
            "self-closing <HealthChecks/> should produce empty Vec, got {:?}",
            result.health_checks
        );
        assert!(!result.is_truncated);
        assert_eq!(result.max_items, "100");
    }

    #[test]
    fn parse_rest_xml_empty_list_open_close() {
        // Some services return <HealthChecks></HealthChecks> when empty
        let xml = r#"<ListHealthChecksResponse><HealthChecks></HealthChecks><IsTruncated>false</IsTruncated><MaxItems>100</MaxItems></ListHealthChecksResponse>"#;
        let result: RestListHealthChecksResponse = parse_rest_xml_response(xml).unwrap();
        assert!(
            result.health_checks.is_empty(),
            "empty <HealthChecks></HealthChecks> should produce empty Vec, got {:?}",
            result.health_checks
        );
    }

    #[derive(Debug, Deserialize, Default, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct RestItem {
        name: String,
    }

    #[derive(Debug, Deserialize, Default)]
    #[serde(rename_all = "PascalCase")]
    struct RestListResponse {
        #[serde(default)]
        items: Vec<RestItem>,
        max_items: String,
    }

    #[test]
    fn parse_rest_xml_non_empty_list() {
        // Verify normal non-empty list still works
        let xml = r#"<ListResponse>
  <Items>
    <Item><Name>foo</Name></Item>
    <Item><Name>bar</Name></Item>
  </Items>
  <MaxItems>100</MaxItems>
</ListResponse>"#;
        let result: RestListResponse = parse_rest_xml_response(xml).unwrap();
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].name, "foo");
        assert_eq!(result.items[1].name, "bar");
    }

    #[test]
    fn find_matching_close_handles_nesting() {
        let xml = "<Item>inner</Item>outer</Item>";
        // depth starts at 1 (we're inside an outer <Item>)
        // first <Item> bumps to 2, first </Item> drops to 1, second </Item> drops to 0
        let pos = find_matching_close(xml, "Item").unwrap();
        assert_eq!(pos, 23); // points to the outer </Item>
    }

    #[test]
    fn extract_child_contents_with_nested_same_name() {
        let xml = "<Root><Item><Item>nested</Item>outer</Item><Item>simple</Item></Root>";
        let contents = extract_child_contents(xml, "Item");
        assert_eq!(contents.len(), 2);
        assert_eq!(contents[0], "<Item>nested</Item>outer");
        assert_eq!(contents[1], "simple");
    }

    #[test]
    fn get_uniform_child_tag_with_nested_same_name() {
        let xml = "<Item><Item>nested</Item>outer</Item><Item>simple</Item>";
        let tag = get_uniform_child_tag(xml);
        assert_eq!(tag, Some("Item".to_string()));
    }
}
