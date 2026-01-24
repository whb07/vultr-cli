//! VFS command handlers

use vultr_api::VultrClient;
use crate::commands::{VfsArgs, VfsAttachmentCommands, VfsCommands, VfsCreateArgs, VfsUpdateArgs};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{CreateVfsRequest, UpdateVfsRequest, VfsStorageSizeRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_vfs(
    args: VfsArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        VfsCommands::List => {
            let vfs = client.list_vfs().await?;
            print_output(&vfs, output);
        }
        VfsCommands::Regions => {
            let regions = client.list_vfs_regions().await?;
            print_output(&regions, output);
        }
        VfsCommands::Get { id } => {
            let vfs = client.get_vfs(&id).await?;
            print_output(&vfs, output);
        }
        VfsCommands::Create(create_args) => {
            let request = build_create_vfs_request(create_args);
            let vfs = client.create_vfs(request).await?;
            print_output(&vfs, output);
        }
        VfsCommands::Update(update_args) => {
            let request = build_update_vfs_request(update_args)?;
            let vfs = client.update_vfs(&request.0, request.1).await?;
            print_output(&vfs, output);
        }
        VfsCommands::Delete { id } => {
            if skip_confirm || confirm_delete("VFS", &id)? {
                client.delete_vfs(&id).await?;
                print_success(&format!("VFS {} deleted", id));
            }
        }
        VfsCommands::Attachment(attach_args) => {
            handle_vfs_attachment(attach_args, client, output, skip_confirm).await?;
        }
    }
    Ok(())
}

fn build_create_vfs_request(args: VfsCreateArgs) -> CreateVfsRequest {
    CreateVfsRequest {
        region: args.region,
        label: args.label,
        storage_size: VfsStorageSizeRequest { gb: args.size_gb },
        disk_type: args.disk_type,
        tags: args.tags,
    }
}

fn build_update_vfs_request(args: VfsUpdateArgs) -> VultrResult<(String, UpdateVfsRequest)> {
    if args.label.is_none() && args.size_gb.is_none() {
        return Err(VultrError::InvalidInput(
            "Provide --label or --size-gb".to_string(),
        ));
    }
    let request = UpdateVfsRequest {
        label: args.label,
        storage_size: args.size_gb.map(|gb| VfsStorageSizeRequest { gb }),
    };
    Ok((args.id, request))
}

async fn handle_vfs_attachment(
    args: crate::commands::VfsAttachmentArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        VfsAttachmentCommands::List { vfs_id } => {
            let attachments = client.list_vfs_attachments(&vfs_id).await?;
            print_output(&attachments, output);
        }
        VfsAttachmentCommands::Get { vfs_id, vps_id } => {
            let attachment = client.get_vfs_attachment(&vfs_id, &vps_id).await?;
            print_output(&attachment, output);
        }
        VfsAttachmentCommands::Attach { vfs_id, vps_id } => {
            let attachment = client.create_vfs_attachment(&vfs_id, &vps_id).await?;
            print_output(&attachment, output);
        }
        VfsAttachmentCommands::Detach { vfs_id, vps_id } => {
            if skip_confirm || confirm_delete("VFS attachment", &vps_id)? {
                client.delete_vfs_attachment(&vfs_id, &vps_id).await?;
                print_success(&format!("VFS attachment {} deleted", vps_id));
            }
        }
    }
    Ok(())
}
