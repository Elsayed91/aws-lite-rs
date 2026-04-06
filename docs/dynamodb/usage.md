# Amazon DynamoDB Usage Examples

## List All Tables

```rust
use aws_lite::AwsHttpClient;
use aws_lite::types::dynamodb::ListTablesInput;

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let response = client.dynamodb().list_tables(&ListTablesInput::default()).await?;

for name in &response.table_names {
    println!("{name}");
}
```

## Describe a Table

```rust
use aws_lite::types::dynamodb::DescribeTableInput;

let response = client.dynamodb().describe_table(&DescribeTableInput {
    table_name: "my-table".to_string(),
}).await?;

let table = response.table.unwrap();
println!("Status: {:?}", table.table_status);
println!("Items: {:?}", table.item_count);
println!("Size: {:?} bytes", table.table_size_bytes);

if let Some(billing) = &table.billing_mode_summary {
    println!("Billing: {:?}", billing.billing_mode);
}

for key in &table.key_schema {
    println!("Key: {} ({})", key.attribute_name, key.key_type);
}
```

## Switch Billing Mode to Provisioned

```rust
use aws_lite::types::dynamodb::{UpdateTableInput, ProvisionedThroughput};

let response = client.dynamodb().update_table(&UpdateTableInput {
    table_name: "my-table".to_string(),
    billing_mode: Some("PROVISIONED".to_string()),
    provisioned_throughput: Some(ProvisionedThroughput {
        read_capacity_units: 10,
        write_capacity_units: 5,
    }),
    ..Default::default()
}).await?;

println!("Status: {:?}", response.table_description.unwrap().table_status);
```

## Enable Deletion Protection

```rust
use aws_lite::types::dynamodb::UpdateTableInput;

client.dynamodb().update_table(&UpdateTableInput {
    table_name: "my-table".to_string(),
    deletion_protection_enabled: Some(true),
    ..Default::default()
}).await?;
```

## Delete a Table

```rust
use aws_lite::types::dynamodb::DeleteTableInput;

let response = client.dynamodb().delete_table(&DeleteTableInput {
    table_name: "my-table".to_string(),
}).await?;

println!("Status: {:?}", response.table_description.unwrap().table_status);
// Status: Some("DELETING")
```

## Paginate Through Tables

```rust
use aws_lite::types::dynamodb::ListTablesInput;

let mut start_table = None;
loop {
    let response = client.dynamodb().list_tables(&ListTablesInput {
        exclusive_start_table_name: start_table.clone(),
        limit: Some(10),
    }).await?;

    for name in &response.table_names {
        println!("{name}");
    }

    match response.last_evaluated_table_name {
        Some(last) => start_table = Some(last),
        None => break,
    }
}
```
