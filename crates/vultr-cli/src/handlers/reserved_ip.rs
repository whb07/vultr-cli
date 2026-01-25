//! Reserved IP command handlers

use crate::commands::{ReservedIpArgs, ReservedIpCommands};
use crate::handlers::confirm_delete;
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use vultr_models::{
    AttachReservedIpRequest, ConvertReservedIpRequest, CreateReservedIpRequest,
    UpdateReservedIpRequest,
};
use vultr_output::{print_output, print_success};

pub async fn handle_reserved_ip(
    args: ReservedIpArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        ReservedIpCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_reserved_ips(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (reserved_ips, _) = client
                    .list_reserved_ips(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&reserved_ips, output);
            }
        }

        ReservedIpCommands::Get { id } => {
            let reserved_ip = client.get_reserved_ip(&id).await?;
            print_output(&reserved_ip, output);
        }

        ReservedIpCommands::Create {
            region,
            ip_type,
            label,
        } => {
            let reserved_ip = client
                .create_reserved_ip(CreateReservedIpRequest {
                    region,
                    ip_type,
                    label,
                })
                .await?;
            print_success(&format!("Reserved IP {} created", reserved_ip.id));
            print_output(&reserved_ip, output);
        }

        ReservedIpCommands::Update { id, label } => {
            let reserved_ip = client
                .update_reserved_ip(&id, UpdateReservedIpRequest { label })
                .await?;
            print_success(&format!("Reserved IP {} updated", id));
            print_output(&reserved_ip, output);
        }

        ReservedIpCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("reserved IP", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_reserved_ip(&id).await?;
            print_success(&format!("Reserved IP {} deleted", id));
        }

        ReservedIpCommands::Attach { id, instance_id } => {
            client
                .attach_reserved_ip(
                    &id,
                    AttachReservedIpRequest {
                        instance_id: instance_id.clone(),
                    },
                )
                .await?;
            print_success(&format!(
                "Reserved IP {} attached to instance {}",
                id, instance_id
            ));
        }

        ReservedIpCommands::Detach { id } => {
            client.detach_reserved_ip(&id).await?;
            print_success(&format!("Reserved IP {} detached", id));
        }

        ReservedIpCommands::Convert { ip_address, label } => {
            let reserved_ip = client
                .convert_to_reserved_ip(ConvertReservedIpRequest {
                    ip_address: ip_address.clone(),
                    label,
                })
                .await?;
            print_success(&format!(
                "IP address {} converted to reserved IP {}",
                ip_address, reserved_ip.id
            ));
            print_output(&reserved_ip, output);
        }
    }
    Ok(())
}
