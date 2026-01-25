//! Object storage command handlers

use crate::commands::{ObjectStorageArgs, ObjectStorageCommands};
use crate::handlers::confirm_delete;
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use vultr_models::{CreateObjectStorageRequest, UpdateObjectStorageRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_object_storage(
    args: ObjectStorageArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        ObjectStorageCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_object_storages(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (storages, _) = client
                    .list_object_storages(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&storages, output);
            }
        }

        ObjectStorageCommands::Get { id } => {
            let storage = client.get_object_storage(&id).await?;
            print_output(&storage, output);
        }

        ObjectStorageCommands::Create {
            cluster_id,
            tier_id,
            label,
        } => {
            let storage = client
                .create_object_storage(CreateObjectStorageRequest {
                    cluster_id,
                    tier_id,
                    label,
                })
                .await?;
            print_success(&format!("Object storage {} created", storage.id));
            print_output(&storage, output);
        }

        ObjectStorageCommands::Update { id, label } => {
            client
                .update_object_storage(&id, UpdateObjectStorageRequest { label: Some(label) })
                .await?;
            print_success(&format!("Object storage {} updated", id));
        }

        ObjectStorageCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("object storage", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_object_storage(&id).await?;
            print_success(&format!("Object storage {} deleted", id));
        }

        ObjectStorageCommands::RegenerateKeys { id } => {
            let credentials = client.regenerate_object_storage_keys(&id).await?;
            print_success(&format!("Object storage {} keys regenerated", id));
            print_output(&credentials, output);
        }

        ObjectStorageCommands::Clusters(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_object_storage_clusters(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (clusters, _) = client
                    .list_object_storage_clusters(
                        Some(list_args.per_page),
                        list_args.cursor.as_deref(),
                    )
                    .await?;
                print_output(&clusters, output);
            }
        }

        ObjectStorageCommands::Tiers { cluster_id } => {
            if let Some(cid) = cluster_id {
                let tiers = client.list_cluster_tiers(cid).await?;
                print_output(&tiers, output);
            } else {
                let tiers = client.list_object_storage_tiers().await?;
                print_output(&tiers, output);
            }
        }
    }
    Ok(())
}
