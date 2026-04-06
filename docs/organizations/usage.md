# AWS Organizations Usage

## Listing All OUs in the Root

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::organizations::*;
use tokio_stream::StreamExt;
use std::pin::pin;

let client = AwsHttpClient::from_default_chain("us-east-1")?;

// Get the root ID first (typically "r-xxxx")
let root_id = "r-abcd";

let mut stream = pin!(client.organizations()
    .list_organizational_units_for_parent_stream(root_id));

while let Some(ou) = stream.next().await {
    let ou = ou?;
    println!("OU: {} ({})", ou.name.unwrap_or_default(), ou.id.unwrap_or_default());
}
```

## Listing All Accounts in an OU

```rust
let ou_id = "ou-root-11111111";

let mut stream = pin!(client.organizations()
    .list_accounts_for_parent_stream(ou_id));

while let Some(account) = stream.next().await {
    let account = account?;
    println!("Account: {} - {} ({:?})",
        account.id.unwrap_or_default(),
        account.name.unwrap_or_default(),
        account.status,
    );
}
```

## Required IAM Permissions

- `organizations:ListOrganizationalUnitsForParent`
- `organizations:ListAccountsForParent`
