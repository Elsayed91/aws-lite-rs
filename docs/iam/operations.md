# IAM Operations

## Read Operations

### list_users

**Signature**: `pub async fn list_users(&self) -> Result<ListUsersResponse>`

Lists all IAM users in the account.

**Returns**: `Result<ListUsersResponse>` with `users: Vec<User>`

---

### list_attached_user_policies

**Signature**: `pub async fn list_attached_user_policies(&self, user_name: &str) -> Result<ListAttachedUserPoliciesResponse>`

Lists all managed policies attached to the specified IAM user.

| Parameter | Type | Description |
|-----------|------|-------------|
| `user_name` | `&str` | IAM user name (friendly name, not ARN) |

**Returns**: `Result<ListAttachedUserPoliciesResponse>` with `attached_policies: Vec<AttachedPolicy>`

---

## Write Operations

### detach_user_policy

**Signature**: `pub async fn detach_user_policy(&self, user_name: &str, policy_arn: &str) -> Result<()>`

Removes a managed policy from an IAM user.

| Parameter | Type | Description |
|-----------|------|-------------|
| `user_name` | `&str` | IAM user name |
| `policy_arn` | `&str` | ARN of the policy to detach |

**Returns**: `Result<()>`

---

### delete_access_key

**Signature**: `pub async fn delete_access_key(&self, user_name: &str, access_key_id: &str) -> Result<()>`

Deletes an access key pair for an IAM user.

| Parameter | Type | Description |
|-----------|------|-------------|
| `user_name` | `&str` | IAM user name |
| `access_key_id` | `&str` | Access key ID to delete |

**Returns**: `Result<()>`
