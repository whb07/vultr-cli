//! Private network command handlers

use vultr_api::VultrClient;
use crate::commands::{PrivateNetworkArgs, PrivateNetworkCommands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{CreateNetworkRequest, UpdateNetworkRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_private_network(
    args: PrivateNetworkArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        PrivateNetworkCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_networks(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (nets, _) = client
                    .list_networks(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&nets, output);
            }
        }
        PrivateNetworkCommands::Get { id } => {
            let network = client.get_network(&id).await?;
            print_output(&network, output);
        }
        PrivateNetworkCommands::Create {
            region,
            description,
            subnet,
            subnet_mask,
        } => {
            let request = CreateNetworkRequest {
                region,
                description,
                v4_subnet: subnet,
                v4_subnet_mask: subnet_mask,
            };
            let network = client.create_network(request).await?;
            print_success(&format!("Private network {} created", network.id));
            print_output(&network, output);
        }
        PrivateNetworkCommands::Update { id, description } => {
            client
                .update_network(&id, UpdateNetworkRequest { description })
                .await?;
            print_success(&format!("Private network {} updated", id));
        }
        PrivateNetworkCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("private network", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_network(&id).await?;
            print_success(&format!("Private network {} deleted", id));
        }
    }
    Ok(())
}
