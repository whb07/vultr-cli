//! Snapshot command handlers

use crate::commands::{SnapshotArgs, SnapshotCommands};
use crate::handlers::confirm_delete;
use vultr_api::{self as api, VultrClient, WaitOptions};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use vultr_models::{CreateSnapshotFromUrlRequest, CreateSnapshotRequest, UpdateSnapshotRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_snapshot(
    args: SnapshotArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        SnapshotCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_snapshots(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (snapshots, _) = client
                    .list_snapshots(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&snapshots, output);
            }
        }

        SnapshotCommands::Get { id } => {
            let snapshot = client.get_snapshot(&id).await?;
            print_output(&snapshot, output);
        }

        SnapshotCommands::Create {
            instance_id,
            description,
        } => {
            let snapshot = client
                .create_snapshot(CreateSnapshotRequest {
                    instance_id,
                    description,
                })
                .await?;
            print_success(&format!("Snapshot {} creation initiated", snapshot.id));

            if wait {
                let snapshot =
                    api::wait_for_snapshot_complete(client, &snapshot.id, wait_opts).await?;
                print_output(&snapshot, output);
            } else {
                print_output(&snapshot, output);
            }
        }

        SnapshotCommands::CreateFromUrl { url, description } => {
            let snapshot = client
                .create_snapshot_from_url(CreateSnapshotFromUrlRequest { url, description })
                .await?;
            print_success(&format!(
                "Snapshot {} creation from URL initiated",
                snapshot.id
            ));

            if wait {
                let snapshot =
                    api::wait_for_snapshot_complete(client, &snapshot.id, wait_opts).await?;
                print_output(&snapshot, output);
            } else {
                print_output(&snapshot, output);
            }
        }

        SnapshotCommands::Update { id, description } => {
            client
                .update_snapshot(
                    &id,
                    UpdateSnapshotRequest {
                        description: Some(description),
                    },
                )
                .await?;
            print_success(&format!("Snapshot {} updated", id));
        }

        SnapshotCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("snapshot", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_snapshot(&id).await?;
            print_success(&format!("Snapshot {} deletion initiated", id));
            if wait {
                api::verify_snapshot_deleted(client, &id, wait_opts).await?;
            }
        }
    }
    Ok(())
}
