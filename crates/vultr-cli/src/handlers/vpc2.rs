//! VPC 2.0 command handlers

use vultr_api::VultrClient;
use crate::commands::{Vpc2Args, Vpc2Commands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{
    AttachVpc2NodesRequest, CreateVpc2Request, DetachVpc2NodesRequest, UpdateVpcRequest,
    Vpc2NodeAttachment,
};
use vultr_output::{print_output, print_success};

pub async fn handle_vpc2(
    args: Vpc2Args,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        Vpc2Commands::List(list_args) => {
            let (vpcs, _) = client
                .list_vpc2s(Some(list_args.per_page), list_args.cursor.as_deref())
                .await?;
            print_output(&vpcs, output);
        }

        Vpc2Commands::Get { id } => {
            let vpc = client.get_vpc2(&id).await?;
            print_output(&vpc, output);
        }

        Vpc2Commands::Create {
            region,
            description,
            ip_block,
            prefix_length,
        } => {
            let vpc = client
                .create_vpc2(CreateVpc2Request {
                    region,
                    description,
                    ip_type: Some("v4".to_string()),
                    ip_block,
                    prefix_length,
                })
                .await?;
            print_success(&format!("VPC 2.0 {} created", vpc.id));
            print_output(&vpc, output);
        }

        Vpc2Commands::Update { id, description } => {
            client
                .update_vpc2(
                    &id,
                    UpdateVpcRequest {
                        description: Some(description),
                    },
                )
                .await?;
            print_success(&format!("VPC 2.0 {} updated", id));
        }

        Vpc2Commands::Delete { id } => {
            if !skip_confirm && !confirm_delete("VPC 2.0", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_vpc2(&id).await?;
            print_success(&format!("VPC 2.0 {} deleted", id));
        }

        Vpc2Commands::Nodes { id, list } => {
            let (nodes, _) = client
                .list_vpc2_nodes(&id, Some(list.per_page), list.cursor.as_deref())
                .await?;
            print_output(&nodes, output);
        }

        Vpc2Commands::Attach { id, nodes } => {
            if nodes.is_empty() {
                return Err(VultrError::InvalidInput(
                    "At least one node ID must be provided".to_string(),
                ));
            }
            let attachments: Vec<Vpc2NodeAttachment> = nodes
                .into_iter()
                .map(|node_id| Vpc2NodeAttachment {
                    id: node_id,
                    ip_address: None,
                })
                .collect();
            client
                .attach_vpc2_nodes(&id, AttachVpc2NodesRequest { nodes: attachments })
                .await?;
            print_success(&format!("Nodes attached to VPC 2.0 {}", id));
        }

        Vpc2Commands::Detach { id, nodes } => {
            if nodes.is_empty() {
                return Err(VultrError::InvalidInput(
                    "At least one node ID must be provided".to_string(),
                ));
            }
            if !skip_confirm {
                println!(
                    "Are you sure you want to detach {} node(s) from VPC 2.0 {}? (y/N) ",
                    nodes.len(),
                    id
                );
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    return Err(VultrError::Cancelled);
                }
            }
            client
                .detach_vpc2_nodes(&id, DetachVpc2NodesRequest { nodes })
                .await?;
            print_success(&format!("Nodes detached from VPC 2.0 {}", id));
        }
    }
    Ok(())
}
