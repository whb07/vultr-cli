//! Instance command handlers

use base64::Engine;
use dialoguer::Confirm;

use crate::api::{self, VultrClient, WaitOptions};
use crate::commands::{
    InstanceArgs, InstanceBackupCommands, InstanceBulkCommands, InstanceCommands,
    InstanceIpv4Commands, InstanceIpv6Commands, InstanceIsoCommands, InstanceVpc2Commands,
    InstanceVpcCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::{confirm_delete, read_file_or_bytes};
use crate::models::{
    AttachIsoRequest, AttachVpc2Request, AttachVpcRequest, BulkInstancesRequest,
    CreateInstanceRequest, CreateIpv4Request, DetachVpc2Request, DetachVpcRequest,
    ReinstallInstanceRequest, RestoreInstanceRequest, SetBackupScheduleRequest,
    SetDefaultReverseIpv4Request, SetReverseIpv4Request, SetReverseIpv6Request,
    UpdateInstanceRequest,
};
use crate::output::{print_output, print_success};

pub async fn handle_instance(
    args: InstanceArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        InstanceCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_instances(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (instances, _) = client
                    .list_instances(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&instances, output);
            }
        }

        InstanceCommands::Get { id } => {
            let instance = client.get_instance(&id).await?;
            print_output(&instance, output);
        }

        InstanceCommands::Create(create_args) => {
            let create_args = *create_args;
            let request = CreateInstanceRequest {
                region: create_args.region,
                plan: create_args.plan,
                os_id: create_args.os_id,
                snapshot_id: create_args.snapshot_id,
                iso_id: create_args.iso_id,
                app_id: create_args.app_id,
                image_id: create_args.image_id,
                label: create_args.label,
                hostname: create_args.hostname,
                sshkey_id: create_args.ssh_keys,
                script_id: create_args.script_id,
                enable_ipv6: if create_args.enable_ipv6 {
                    Some(true)
                } else {
                    None
                },
                disable_public_ipv4: if create_args.disable_public_ipv4 {
                    Some(true)
                } else {
                    None
                },
                backups: if create_args.backups {
                    Some("enabled".into())
                } else {
                    None
                },
                ddos_protection: if create_args.ddos_protection {
                    Some(true)
                } else {
                    None
                },
                activation_email: if create_args.activation_email {
                    Some(true)
                } else {
                    None
                },
                attach_vpc: create_args.vpc,
                firewall_group_id: create_args.firewall_group_id,
                reserved_ipv4: create_args.reserved_ipv4,
                tags: create_args.tags,
                user_data: create_args
                    .user_data
                    .as_deref()
                    .map(|v| {
                        let raw = read_file_or_bytes(v)?;
                        Ok::<String, VultrError>(
                            base64::engine::general_purpose::STANDARD.encode(raw),
                        )
                    })
                    .transpose()?,
                user_scheme: create_args.user_scheme,
                ..Default::default()
            };

            let instance = client.create_instance(request).await?;
            print_success(&format!("Instance {} created", instance.id));

            if wait {
                let instance =
                    api::wait_for_instance_ready(client, &instance.id, wait_opts).await?;
                print_output(&instance, output);
            } else {
                print_output(&instance, output);
            }
        }

        InstanceCommands::Update(update_args) => {
            let update_args = *update_args;
            let request = UpdateInstanceRequest {
                label: update_args.label,
                plan: update_args.plan,
                firewall_group_id: update_args.firewall_group_id,
                tags: update_args.tags,
                backups: update_args
                    .backups
                    .map(|b| if b { "enabled" } else { "disabled" }.into()),
                ddos_protection: update_args.ddos_protection,
                ..Default::default()
            };

            let instance = client.update_instance(&update_args.id, request).await?;
            print_success(&format!("Instance {} updated", instance.id));
            print_output(&instance, output);
        }

        InstanceCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("instance", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_instance(&id).await?;
            print_success(&format!("Instance {} deletion initiated", id));
            if wait {
                api::verify_instance_deleted(client, &id, wait_opts).await?;
            }
        }

        InstanceCommands::Start { id } => {
            client.start_instance(&id).await?;
            print_success(&format!("Instance {} start initiated", id));
            if wait {
                let instance = api::wait_for_instance_ready(client, &id, wait_opts).await?;
                print_output(&instance, output);
            }
        }

        InstanceCommands::Stop { id } => {
            client.halt_instance(&id).await?;
            print_success(&format!("Instance {} stop initiated", id));
            if wait {
                let instance = api::wait_for_instance_stopped(client, &id, wait_opts).await?;
                print_output(&instance, output);
            }
        }

        InstanceCommands::Reboot { id } => {
            client.reboot_instance(&id).await?;
            print_success(&format!("Instance {} reboot initiated", id));
            if wait {
                let instance = api::wait_for_instance_ready(client, &id, wait_opts).await?;
                print_output(&instance, output);
            }
        }

        InstanceCommands::Reinstall { id, hostname } => {
            if !skip_confirm
                && !Confirm::new()
                    .with_prompt(format!("Reinstall {}? All data will be lost.", id))
                    .default(false)
                    .interact()
                    .unwrap_or(false)
            {
                return Err(VultrError::Cancelled);
            }

            let instance = client
                .reinstall_instance(&id, ReinstallInstanceRequest { hostname })
                .await?;
            print_success(&format!("Instance {} reinstall initiated", id));

            if wait {
                let instance =
                    api::wait_for_instance_ready(client, &instance.id, wait_opts).await?;
                print_output(&instance, output);
            } else {
                print_output(&instance, output);
            }
        }

        InstanceCommands::Bandwidth { id } => {
            let bandwidth = client.get_instance_bandwidth(&id).await?;
            print_output(&bandwidth, output);
        }

        InstanceCommands::Neighbors { id } => {
            let neighbors = client.get_instance_neighbors(&id).await?;
            print_output(&neighbors, output);
        }

        InstanceCommands::Upgrades { id } => {
            let upgrades = client.get_instance_upgrades(&id).await?;
            print_output(&upgrades, output);
        }

        InstanceCommands::UserData { id } => {
            let user_data = client.get_instance_user_data(&id).await?;
            print_output(&user_data, output);
        }

        InstanceCommands::Restore {
            id,
            backup_id,
            snapshot_id,
        } => {
            if backup_id.is_none() && snapshot_id.is_none() {
                return Err(VultrError::InvalidInput(
                    "Either --backup-id or --snapshot-id is required".into(),
                ));
            }
            let status = client
                .restore_instance(
                    &id,
                    RestoreInstanceRequest {
                        backup_id,
                        snapshot_id,
                    },
                )
                .await?;
            print_success(&format!("Instance {} restore initiated", id));
            print_output(&status, output);
        }

        InstanceCommands::Ipv4(ipv4_args) => {
            handle_instance_ipv4(ipv4_args.command, client, output, skip_confirm).await?;
        }

        InstanceCommands::Ipv6(ipv6_args) => {
            handle_instance_ipv6(ipv6_args.command, client, output).await?;
        }

        InstanceCommands::Iso(iso_args) => {
            handle_instance_iso(iso_args.command, client, output).await?;
        }

        InstanceCommands::Backup(backup_args) => {
            handle_instance_backup(backup_args.command, client, output).await?;
        }

        InstanceCommands::Vpc(vpc_args) => {
            handle_instance_vpc(vpc_args.command, client, output).await?;
        }

        InstanceCommands::Vpc2(vpc2_args) => {
            handle_instance_vpc2(vpc2_args.command, client, output).await?;
        }

        InstanceCommands::Bulk(bulk_args) => {
            handle_instance_bulk(bulk_args.command, client).await?;
        }
    }
    Ok(())
}

async fn handle_instance_ipv4(
    cmd: InstanceIpv4Commands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        InstanceIpv4Commands::List { id } => {
            let ipv4s = client.list_instance_ipv4(&id).await?;
            print_output(&ipv4s, output);
        }

        InstanceIpv4Commands::Create { id, reboot } => {
            let ipv4 = client
                .create_instance_ipv4(
                    &id,
                    CreateIpv4Request {
                        reboot: if reboot { Some(true) } else { None },
                    },
                )
                .await?;
            print_success(&format!("IPv4 {} added to instance {}", ipv4.ip, id));
            print_output(&ipv4, output);
        }

        InstanceIpv4Commands::Delete { id, ipv4 } => {
            if !skip_confirm && !confirm_delete("IPv4 address", &ipv4)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_instance_ipv4(&id, &ipv4).await?;
            print_success(&format!("IPv4 {} deleted from instance {}", ipv4, id));
        }

        InstanceIpv4Commands::Reverse { id, ip, reverse } => {
            client
                .set_instance_reverse_ipv4(
                    &id,
                    SetReverseIpv4Request {
                        ip: ip.clone(),
                        reverse,
                    },
                )
                .await?;
            print_success(&format!("Reverse DNS set for {} on instance {}", ip, id));
        }

        InstanceIpv4Commands::DefaultReverse { id, ip } => {
            client
                .set_instance_default_reverse_ipv4(
                    &id,
                    SetDefaultReverseIpv4Request { ip: ip.clone() },
                )
                .await?;
            print_success(&format!(
                "Default reverse DNS set for {} on instance {}",
                ip, id
            ));
        }
    }
    Ok(())
}

async fn handle_instance_ipv6(
    cmd: InstanceIpv6Commands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        InstanceIpv6Commands::List { id } => {
            let ipv6s = client.list_instance_ipv6(&id).await?;
            print_output(&ipv6s, output);
        }

        InstanceIpv6Commands::ReverseList { id } => {
            let reverse = client.list_instance_reverse_ipv6(&id).await?;
            print_output(&reverse, output);
        }

        InstanceIpv6Commands::Reverse { id, ip, reverse } => {
            client
                .set_instance_reverse_ipv6(
                    &id,
                    SetReverseIpv6Request {
                        ip: ip.clone(),
                        reverse,
                    },
                )
                .await?;
            print_success(&format!("Reverse DNS set for {} on instance {}", ip, id));
        }

        InstanceIpv6Commands::DeleteReverse { id, ip } => {
            client.delete_instance_reverse_ipv6(&id, &ip).await?;
            print_success(&format!(
                "Reverse DNS deleted for {} on instance {}",
                ip, id
            ));
        }
    }
    Ok(())
}

async fn handle_instance_iso(
    cmd: InstanceIsoCommands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        InstanceIsoCommands::Status { id } => {
            let status = client.get_instance_iso_status(&id).await?;
            print_output(&status, output);
        }

        InstanceIsoCommands::Attach { id, iso_id } => {
            let status = client
                .attach_instance_iso(
                    &id,
                    AttachIsoRequest {
                        iso_id: iso_id.clone(),
                    },
                )
                .await?;
            print_success(&format!("ISO {} attached to instance {}", iso_id, id));
            print_output(&status, output);
        }

        InstanceIsoCommands::Detach { id } => {
            let status = client.detach_instance_iso(&id).await?;
            print_success(&format!("ISO detached from instance {}", id));
            print_output(&status, output);
        }
    }
    Ok(())
}

async fn handle_instance_backup(
    cmd: InstanceBackupCommands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        InstanceBackupCommands::Get { id } => {
            let schedule = client.get_instance_backup_schedule(&id).await?;
            print_output(&schedule, output);
        }

        InstanceBackupCommands::Set {
            id,
            schedule_type,
            hour,
            dow,
            dom,
        } => {
            client
                .set_instance_backup_schedule(
                    &id,
                    SetBackupScheduleRequest {
                        schedule_type,
                        hour,
                        dow,
                        dom,
                    },
                )
                .await?;
            print_success(&format!("Backup schedule set for instance {}", id));
        }
    }
    Ok(())
}

async fn handle_instance_vpc(
    cmd: InstanceVpcCommands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        InstanceVpcCommands::List { id } => {
            let vpcs = client.list_instance_vpcs(&id).await?;
            print_output(&vpcs, output);
        }

        InstanceVpcCommands::Attach { id, vpc_id } => {
            client
                .attach_instance_vpc(
                    &id,
                    AttachVpcRequest {
                        vpc_id: vpc_id.clone(),
                    },
                )
                .await?;
            print_success(&format!("VPC {} attached to instance {}", vpc_id, id));
        }

        InstanceVpcCommands::Detach { id, vpc_id } => {
            client
                .detach_instance_vpc(
                    &id,
                    DetachVpcRequest {
                        vpc_id: vpc_id.clone(),
                    },
                )
                .await?;
            print_success(&format!("VPC {} detached from instance {}", vpc_id, id));
        }
    }
    Ok(())
}

async fn handle_instance_vpc2(
    cmd: InstanceVpc2Commands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        InstanceVpc2Commands::List { id } => {
            let vpc2s = client.list_instance_vpc2s(&id).await?;
            print_output(&vpc2s, output);
        }

        InstanceVpc2Commands::Attach {
            id,
            vpc_id,
            ip_address,
        } => {
            client
                .attach_instance_vpc2(
                    &id,
                    AttachVpc2Request {
                        vpc_id: vpc_id.clone(),
                        ip_address,
                    },
                )
                .await?;
            print_success(&format!("VPC2 {} attached to instance {}", vpc_id, id));
        }

        InstanceVpc2Commands::Detach { id, vpc_id } => {
            client
                .detach_instance_vpc2(
                    &id,
                    DetachVpc2Request {
                        vpc_id: vpc_id.clone(),
                    },
                )
                .await?;
            print_success(&format!("VPC2 {} detached from instance {}", vpc_id, id));
        }
    }
    Ok(())
}

async fn handle_instance_bulk(cmd: InstanceBulkCommands, client: &VultrClient) -> VultrResult<()> {
    match cmd {
        InstanceBulkCommands::Start { ids } => {
            client
                .bulk_start_instances(BulkInstancesRequest {
                    instance_ids: ids.clone(),
                })
                .await?;
            print_success(&format!("Start initiated for {} instances", ids.len()));
        }

        InstanceBulkCommands::Stop { ids } => {
            client
                .bulk_halt_instances(BulkInstancesRequest {
                    instance_ids: ids.clone(),
                })
                .await?;
            print_success(&format!("Stop initiated for {} instances", ids.len()));
        }

        InstanceBulkCommands::Reboot { ids } => {
            client
                .bulk_reboot_instances(BulkInstancesRequest {
                    instance_ids: ids.clone(),
                })
                .await?;
            print_success(&format!("Reboot initiated for {} instances", ids.len()));
        }
    }
    Ok(())
}
