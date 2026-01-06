//! Kubernetes command handlers

use base64::Engine;

use crate::api::VultrClient;
use crate::commands::{
    KubernetesArgs, KubernetesCommands, KubernetesNodeCommands, KubernetesNodePoolCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use crate::models::{
    CreateClusterRequest, CreateNodePoolRequest, UpdateClusterRequest, UpdateNodePoolRequest,
    UpgradeClusterRequest,
};
use crate::output::{print_output, print_success};

pub async fn handle_kubernetes(
    args: KubernetesArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        KubernetesCommands::List => {
            let clusters = client.list_kubernetes_clusters().await?;
            print_output(&clusters, output);
        }

        KubernetesCommands::Get { id } => {
            let cluster = client.get_kubernetes_cluster(&id).await?;
            print_output(&cluster, output);
        }

        KubernetesCommands::Create(create_args) => {
            // Build node pools if specified
            let node_pools = if let (Some(label), Some(plan)) =
                (&create_args.pool_label, &create_args.pool_plan)
            {
                Some(vec![CreateNodePoolRequest {
                    node_quantity: create_args.pool_quantity,
                    label: label.clone(),
                    plan: plan.clone(),
                    tag: None,
                    auto_scaler: if create_args.pool_auto_scaler {
                        Some(true)
                    } else {
                        None
                    },
                    min_nodes: create_args.pool_min_nodes,
                    max_nodes: create_args.pool_max_nodes,
                    labels: None,
                    taints: None,
                    user_data: None,
                }])
            } else {
                None
            };

            let request = CreateClusterRequest {
                region: create_args.region,
                version: create_args.version,
                label: create_args.label,
                ha_controlplanes: if create_args.ha_controlplanes {
                    Some(true)
                } else {
                    None
                },
                enable_firewall: if create_args.enable_firewall {
                    Some(true)
                } else {
                    None
                },
                oidc: None,
                node_pools,
            };

            let cluster = client.create_kubernetes_cluster(request).await?;
            print_success(&format!("Kubernetes cluster {} created", cluster.id));
            print_output(&cluster, output);
        }

        KubernetesCommands::Update { id, label } => {
            let request = UpdateClusterRequest { label };
            let cluster = client.update_kubernetes_cluster(&id, request).await?;
            print_success(&format!("Kubernetes cluster {} updated", cluster.id));
            print_output(&cluster, output);
        }

        KubernetesCommands::Delete { id, with_resources } => {
            if !skip_confirm && !confirm_delete("Kubernetes cluster", &id)? {
                return Err(VultrError::Cancelled);
            }

            if with_resources {
                client.delete_kubernetes_cluster_with_resources(&id).await?;
                print_success(&format!(
                    "Kubernetes cluster {} and linked resources deletion initiated",
                    id
                ));
            } else {
                client.delete_kubernetes_cluster(&id).await?;
                print_success(&format!("Kubernetes cluster {} deletion initiated", id));
            }
        }

        KubernetesCommands::Config { id, decode } => {
            let config = client.get_kubernetes_config(&id).await?;
            if decode {
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(&config)
                    .map_err(|e| {
                        VultrError::InvalidInput(format!("Failed to decode config: {}", e))
                    })?;
                let decoded_str = String::from_utf8(decoded).map_err(|e| {
                    VultrError::InvalidInput(format!("Invalid UTF-8 in config: {}", e))
                })?;
                println!("{}", decoded_str);
            } else {
                println!("{}", config);
            }
        }

        KubernetesCommands::Versions => {
            let versions = client.get_kubernetes_versions().await?;
            print_output(&versions, output);
        }

        KubernetesCommands::Upgrades { id } => {
            let upgrades = client.get_kubernetes_available_upgrades(&id).await?;
            if upgrades.is_empty() {
                println!("No upgrades available for cluster {}", id);
            } else {
                print_output(&upgrades, output);
            }
        }

        KubernetesCommands::Upgrade { id, version } => {
            let request = UpgradeClusterRequest {
                upgrade_version: version.clone(),
            };
            client.upgrade_kubernetes_cluster(&id, request).await?;
            print_success(&format!(
                "Kubernetes cluster {} upgrade to {} initiated",
                id, version
            ));
        }

        KubernetesCommands::Resources { id } => {
            let resources = client.get_kubernetes_resources(&id).await?;
            print_output(&resources, output);
        }

        KubernetesCommands::NodePool(pool_args) => {
            handle_node_pool(pool_args.command, client, output, skip_confirm).await?;
        }

        KubernetesCommands::Node(node_args) => {
            handle_node(node_args.command, client, output, skip_confirm).await?;
        }
    }

    Ok(())
}

async fn handle_node_pool(
    cmd: KubernetesNodePoolCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        KubernetesNodePoolCommands::List { cluster_id } => {
            let pools = client.list_node_pools(&cluster_id).await?;
            print_output(&pools, output);
        }

        KubernetesNodePoolCommands::Get { cluster_id, id } => {
            let pool = client.get_node_pool(&cluster_id, &id).await?;
            print_output(&pool, output);
        }

        KubernetesNodePoolCommands::Create {
            cluster_id,
            label,
            plan,
            quantity,
            tag,
            auto_scaler,
            min_nodes,
            max_nodes,
        } => {
            let request = CreateNodePoolRequest {
                node_quantity: quantity,
                label: label.clone(),
                plan,
                tag,
                auto_scaler: if auto_scaler { Some(true) } else { None },
                min_nodes,
                max_nodes,
                labels: None,
                taints: None,
                user_data: None,
            };

            let pool = client.create_node_pool(&cluster_id, request).await?;
            print_success(&format!("Node pool {} created", pool.id));
            print_output(&pool, output);
        }

        KubernetesNodePoolCommands::Update {
            cluster_id,
            id,
            quantity,
            tag,
            auto_scaler,
            min_nodes,
            max_nodes,
        } => {
            let request = UpdateNodePoolRequest {
                node_quantity: quantity,
                tag,
                auto_scaler,
                min_nodes,
                max_nodes,
                labels: None,
                taints: None,
            };

            let pool = client.update_node_pool(&cluster_id, &id, request).await?;
            print_success(&format!("Node pool {} updated", pool.id));
            print_output(&pool, output);
        }

        KubernetesNodePoolCommands::Delete { cluster_id, id } => {
            if !skip_confirm && !confirm_delete("node pool", &id)? {
                return Err(VultrError::Cancelled);
            }

            client.delete_node_pool(&cluster_id, &id).await?;
            print_success(&format!("Node pool {} deletion initiated", id));
        }
    }

    Ok(())
}

async fn handle_node(
    cmd: KubernetesNodeCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        KubernetesNodeCommands::List {
            cluster_id,
            nodepool_id,
        } => {
            let nodes = client.list_nodes(&cluster_id, &nodepool_id).await?;
            print_output(&nodes, output);
        }

        KubernetesNodeCommands::Get {
            cluster_id,
            nodepool_id,
            id,
        } => {
            let node = client.get_node(&cluster_id, &nodepool_id, &id).await?;
            print_output(&node, output);
        }

        KubernetesNodeCommands::Delete {
            cluster_id,
            nodepool_id,
            id,
        } => {
            if !skip_confirm && !confirm_delete("node", &id)? {
                return Err(VultrError::Cancelled);
            }

            client.delete_node(&cluster_id, &nodepool_id, &id).await?;
            print_success(&format!("Node {} deletion initiated", id));
        }

        KubernetesNodeCommands::Recycle {
            cluster_id,
            nodepool_id,
            id,
        } => {
            if !skip_confirm && !confirm_delete("node (recycle)", &id)? {
                return Err(VultrError::Cancelled);
            }

            client.recycle_node(&cluster_id, &nodepool_id, &id).await?;
            print_success(&format!("Node {} recycle initiated", id));
        }
    }

    Ok(())
}
