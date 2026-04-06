# AWS Organizations Operations

## ListOrganizationalUnitsForParent

Lists the organizational units in a specified parent OU or root.

```rust
let response = client.organizations()
    .list_organizational_units_for_parent(&ListOrganizationalUnitsForParentRequest {
        parent_id: "r-abcd".into(),
        ..Default::default()
    })
    .await?;
```

**Pagination stream:**

```rust
use tokio_stream::StreamExt;
use std::pin::pin;

let mut stream = pin!(client.organizations()
    .list_organizational_units_for_parent_stream("r-abcd"));
while let Some(ou) = stream.next().await {
    let ou = ou?;
    println!("{}: {}", ou.id.unwrap_or_default(), ou.name.unwrap_or_default());
}
```

## ListAccountsForParent

Lists the accounts contained by the specified parent OU or root.

```rust
let response = client.organizations()
    .list_accounts_for_parent(&ListAccountsForParentRequest {
        parent_id: "ou-root-11111111".into(),
        ..Default::default()
    })
    .await?;
```

**Pagination stream:**

```rust
use tokio_stream::StreamExt;
use std::pin::pin;

let mut stream = pin!(client.organizations()
    .list_accounts_for_parent_stream("ou-root-11111111"));
while let Some(account) = stream.next().await {
    let account = account?;
    println!("{}: {} ({})",
        account.id.unwrap_or_default(),
        account.name.unwrap_or_default(),
        account.status.map(|s| format!("{s:?}")).unwrap_or_default(),
    );
}
```
