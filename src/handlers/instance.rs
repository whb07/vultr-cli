//! Instance command handlers

use base64::Engine;
use dialoguer::Confirm;

use crate::api::{self, VultrClient, WaitOptions};
use crate::commands::{InstanceArgs, InstanceCommands};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::{confirm_delete, read_file_or_bytes};
use crate::models::{CreateInstanceRequest, ReinstallInstanceRequest, UpdateInstanceRequest};
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
            let request = CreateInstanceRequest {
                region: create_args.region,
                plan: create_args.plan,
                os_id: create_args.os_id,
                snapshot_id: create_args.snapshot_id,
                app_id: create_args.app_id,
                label: create_args.label,
                hostname: create_args.hostname,
                sshkey_id: create_args.ssh_keys,
                script_id: create_args.script_id,
                enable_ipv6: if create_args.enable_ipv6 {
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
                attach_vpc: create_args.vpc,
                firewall_group_id: create_args.firewall_group_id,
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
            api::verify_instance_deleted(client, &id, wait_opts).await?;
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
    }
    Ok(())
}
