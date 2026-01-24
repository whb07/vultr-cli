//! VPC command handlers

use vultr_api::VultrClient;
use crate::commands::{VpcArgs, VpcCommands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{CreateVpcRequest, UpdateVpcRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_vpc(
    args: VpcArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        VpcCommands::List(list_args) => {
            let (vpcs, _) = client
                .list_vpcs(Some(list_args.per_page), list_args.cursor.as_deref())
                .await?;
            print_output(&vpcs, output);
        }

        VpcCommands::Get { id } => {
            let vpc = client.get_vpc(&id).await?;
            print_output(&vpc, output);
        }

        VpcCommands::Create {
            region,
            description,
            subnet,
            subnet_mask,
        } => {
            let vpc = client
                .create_vpc(CreateVpcRequest {
                    region,
                    description,
                    v4_subnet: subnet,
                    v4_subnet_mask: subnet_mask,
                })
                .await?;
            print_success(&format!("VPC {} created", vpc.id));
            print_output(&vpc, output);
        }

        VpcCommands::Update { id, description } => {
            client
                .update_vpc(
                    &id,
                    UpdateVpcRequest {
                        description: Some(description),
                    },
                )
                .await?;
            print_success(&format!("VPC {} updated", id));
        }

        VpcCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("VPC", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_vpc(&id).await?;
            print_success(&format!("VPC {} deleted", id));
        }

        VpcCommands::Attachments { id, list } => {
            if list.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_vpc_attachments(&id, Some(list.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (attachments, _) = client
                    .list_vpc_attachments(&id, Some(list.per_page), list.cursor.as_deref())
                    .await?;
                print_output(&attachments, output);
            }
        }
    }
    Ok(())
}
