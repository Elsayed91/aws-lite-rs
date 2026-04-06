# SageMaker Usage Guide

## List All Notebook Instances

```rust
use aws_lite::{AwsHttpClient, types::sagemaker::ListNotebookInstancesInput};

let client = AwsHttpClient::from_default_chain("eu-central-1")?;
let resp = client.sagemaker()
    .list_notebook_instances(&ListNotebookInstancesInput::default())
    .await?;

for nb in &resp.notebook_instances {
    println!("{}: {:?}", nb.notebook_instance_name, nb.notebook_instance_status);
}
```

## Find Idle (InService) Notebooks

```rust
use aws_lite::types::sagemaker::ListNotebookInstancesInput;

let resp = client.sagemaker()
    .list_notebook_instances(&ListNotebookInstancesInput {
        status_equals: Some("InService".to_string()),
        ..Default::default()
    })
    .await?;

println!("Running notebooks: {}", resp.notebook_instances.len());
```

## Stop a Notebook Instance

```rust
use aws_lite::types::sagemaker::StopNotebookInstanceInput;

client.sagemaker()
    .stop_notebook_instance(&StopNotebookInstanceInput {
        notebook_instance_name: "my-idle-notebook".to_string(),
    })
    .await?;

println!("Stop requested for my-idle-notebook");
```

## Paginate Through All Notebooks

```rust
use aws_lite::types::sagemaker::{ListNotebookInstancesInput, NotebookInstanceSummary};

let mut all_notebooks: Vec<NotebookInstanceSummary> = Vec::new();
let mut next_token: Option<String> = None;

loop {
    let resp = client.sagemaker()
        .list_notebook_instances(&ListNotebookInstancesInput {
            next_token: next_token.clone(),
            max_results: Some(100),
            ..Default::default()
        })
        .await?;

    all_notebooks.extend(resp.notebook_instances);
    next_token = resp.next_token;
    if next_token.is_none() {
        break;
    }
}

println!("Total notebooks: {}", all_notebooks.len());
```
