//! Block storage command handlers

use vultr_api::{self as api, VultrClient, WaitOptions};
use crate::commands::{BlockStorageArgs, BlockStorageCommands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{
    AttachBlockStorageRequest, CreateBlockStorageRequest, DetachBlockStorageRequest,
    UpdateBlockStorageRequest,
};
use vultr_output::{print_output, print_success};

pub async fn handle_block_storage(
    args: BlockStorageArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        BlockStorageCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_block_storage(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (blocks, _) = client
                    .list_block_storage(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&blocks, output);
            }
        }

        BlockStorageCommands::Get { id } => {
            let block = client.get_block_storage(&id).await?;
            print_output(&block, output);
        }

        BlockStorageCommands::Create {
            region,
            size,
            label,
            block_type,
        } => {
            let block = client
                .create_block_storage(CreateBlockStorageRequest {
                    region,
                    size_gb: size,
                    label,
                    block_type,
                })
                .await?;
            print_success(&format!("Block storage {} created", block.id));

            if wait {
                let block =
                    api::wait_for_block_storage_active(client, &block.id, wait_opts).await?;
                print_output(&block, output);
            } else {
                print_output(&block, output);
            }
        }

        BlockStorageCommands::Update { id, label, size } => {
            client
                .update_block_storage(
                    &id,
                    UpdateBlockStorageRequest {
                        label,
                        size_gb: size,
                    },
                )
                .await?;
            print_success(&format!("Block storage {} updated", id));
        }

        BlockStorageCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("block storage", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_block_storage(&id).await?;
            print_success(&format!("Block storage {} deletion initiated", id));
            if wait {
                api::verify_block_storage_deleted(client, &id, wait_opts).await?;
            }
        }

        BlockStorageCommands::Attach {
            id,
            instance_id,
            live,
        } => {
            client
                .attach_block_storage(
                    &id,
                    AttachBlockStorageRequest {
                        instance_id: instance_id.clone(),
                        live: live.then_some(true),
                    },
                )
                .await?;
            print_success(&format!("Block storage {} attached to {}", id, instance_id));
        }

        BlockStorageCommands::Detach { id, live } => {
            client
                .detach_block_storage(
                    &id,
                    DetachBlockStorageRequest {
                        live: live.then_some(true),
                    },
                )
                .await?;
            print_success(&format!("Block storage {} detached", id));
        }
    }
    Ok(())
}
