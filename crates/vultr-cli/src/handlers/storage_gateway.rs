//! Storage gateway command handlers

use vultr_api::VultrClient;
use crate::commands::{
    StorageGatewayArgs, StorageGatewayCommands, StorageGatewayCreateArgs,
    StorageGatewayExportCommands,
};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{
    CreateStorageGatewayRequest, StorageGatewayExport, StorageGatewayNetwork,
    StorageGatewayNetworkPrimary, StorageGatewayVpc, UpdateStorageGatewayRequest,
};
use vultr_output::{print_output, print_success};

pub async fn handle_storage_gateway(
    args: StorageGatewayArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        StorageGatewayCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_storage_gateways(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (gateways, _) = client
                    .list_storage_gateways(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&gateways, output);
            }
        }
        StorageGatewayCommands::Get { id } => {
            let gateway = client.get_storage_gateway(&id).await?;
            print_output(&gateway, output);
        }
        StorageGatewayCommands::Create(create_args) => {
            let request = build_storage_gateway_request(create_args)?;
            let gateway = client.create_storage_gateway(request).await?;
            print_output(&gateway, output);
        }
        StorageGatewayCommands::Update { id, label } => {
            client
                .update_storage_gateway(&id, UpdateStorageGatewayRequest { label })
                .await?;
            print_success(&format!("Storage gateway {} updated", id));
        }
        StorageGatewayCommands::Delete { id } => {
            if skip_confirm || confirm_delete("storage gateway", &id)? {
                client.delete_storage_gateway(&id).await?;
                print_success(&format!("Storage gateway {} deleted", id));
            }
        }
        StorageGatewayCommands::Export(export_args) => {
            handle_storage_gateway_export(export_args, client, output, skip_confirm).await?;
        }
    }
    Ok(())
}

fn build_storage_gateway_request(
    create_args: StorageGatewayCreateArgs,
) -> VultrResult<CreateStorageGatewayRequest> {
    if create_args.ipv4_public_enabled.is_none()
        && create_args.ipv6_public_enabled.is_none()
        && create_args.vpc_id.is_none()
    {
        return Err(VultrError::InvalidInput(
            "Provide --ipv4-public-enabled, --ipv6-public-enabled, or --vpc-id".to_string(),
        ));
    }

    let export = StorageGatewayExport {
        label: Some(create_args.export_label),
        vfs_uuid: Some(create_args.export_vfs_uuid),
        pseudo_root_path: create_args.export_pseudo_root_path,
        allowed_ips: create_args.export_allowed_ips,
    };

    let primary = StorageGatewayNetworkPrimary {
        ipv4_public_enabled: create_args.ipv4_public_enabled,
        ipv6_public_enabled: create_args.ipv6_public_enabled,
        vpc: create_args
            .vpc_id
            .map(|id| StorageGatewayVpc { vpc_uuid: Some(id) }),
    };

    Ok(CreateStorageGatewayRequest {
        label: create_args.label,
        gateway_type: create_args.gateway_type,
        region: create_args.region,
        export_config: vec![export],
        network_config: StorageGatewayNetwork {
            primary: Some(primary),
        },
    })
}

async fn handle_storage_gateway_export(
    args: crate::commands::StorageGatewayExportArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        StorageGatewayExportCommands::Add {
            gateway_id,
            label,
            vfs_uuid,
            pseudo_root_path,
            allowed_ips,
        } => {
            let export = StorageGatewayExport {
                label: Some(label),
                vfs_uuid: Some(vfs_uuid),
                pseudo_root_path,
                allowed_ips,
            };
            let created = client
                .add_storage_gateway_export(&gateway_id, vec![export])
                .await?;
            print_output(&created, output);
        }
        StorageGatewayExportCommands::Delete {
            gateway_id,
            export_id,
        } => {
            if skip_confirm || confirm_delete("storage gateway export", &export_id)? {
                client
                    .delete_storage_gateway_export(&gateway_id, &export_id)
                    .await?;
                print_success(&format!("Storage gateway export {} deleted", export_id));
            }
        }
    }
    Ok(())
}
