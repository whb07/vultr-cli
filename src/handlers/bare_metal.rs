//! Bare metal command handlers

use base64::Engine;

use crate::api::VultrClient;
use crate::api::{self, WaitOptions};
use crate::commands::{
    BareMetalArgs, BareMetalBulkCommands, BareMetalCommands, BareMetalIpv4Commands,
    BareMetalIpv6Commands, BareMetalVpc2Commands, BareMetalVpcCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::{confirm_delete, read_file_or_bytes};
use crate::models::{
    AttachBareMetalVpc2Request, AttachBareMetalVpcRequest, BulkBareMetalRequest,
    CreateBareMetalRequest, DetachBareMetalVpc2Request, DetachBareMetalVpcRequest,
    ReinstallBareMetalRequest, SetBareMetalDefaultReverseIpv4Request,
    SetBareMetalReverseIpv4Request, SetBareMetalReverseIpv6Request, UpdateBareMetalRequest,
};
use crate::output::{print_output, print_success};
use std::future::Future;

pub async fn handle_bare_metal(
    args: BareMetalArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        BareMetalCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_bare_metals(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (bare_metals, _) = client
                    .list_bare_metals(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&bare_metals, output);
            }
        }

        BareMetalCommands::Get { id } => {
            let bare_metal = client.get_bare_metal(&id).await?;
            print_output(&bare_metal, output);
        }

        BareMetalCommands::Create(create_args) => {
            let request = CreateBareMetalRequest {
                region: create_args.region,
                plan: create_args.plan,
                os_id: create_args.os_id,
                snapshot_id: create_args.snapshot_id,
                app_id: create_args.app_id,
                image_id: create_args.image_id,
                sshkey_id: create_args.sshkey_id,
                script_id: create_args.script_id,
                label: create_args.label,
                enable_ipv6: create_args.enable_ipv6.then_some(true),
                attach_vpc: create_args.attach_vpc,
                attach_vpc2: create_args.attach_vpc2,
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
                reserved_ipv4: create_args.reserved_ipv4,
                persistent_pxe: create_args.persistent_pxe.then_some(true),
                activation_email: create_args.activation_email.then_some(true),
                hostname: create_args.hostname,
                mdisk_mode: create_args.mdisk_mode,
                user_scheme: create_args.user_scheme,
            };

            let bare_metal = client.create_bare_metal(request).await?;
            print_success(&format!("Bare metal server {} created", bare_metal.id));
            print_output(&bare_metal, output);
        }

        BareMetalCommands::Update(update_args) => {
            let request = UpdateBareMetalRequest {
                label: update_args.label,
                enable_ipv6: update_args.enable_ipv6,
                user_data: update_args.user_data,
                tags: update_args.tags,
                attach_vpc: update_args.attach_vpc,
                detach_vpc: update_args.detach_vpc,
                attach_vpc2: update_args.attach_vpc2,
                detach_vpc2: update_args.detach_vpc2,
            };

            let bare_metal = client.update_bare_metal(&update_args.id, request).await?;
            print_success(&format!("Bare metal server {} updated", bare_metal.id));
            print_output(&bare_metal, output);
        }

        BareMetalCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("bare metal server", &id)? {
                return Err(VultrError::Cancelled);
            }
            delete_bare_metal_impl(
                &id,
                || client.delete_bare_metal(&id),
                wait,
                || api::verify_bare_metal_deleted(client, &id, wait_opts),
            )
            .await?;
        }

        BareMetalCommands::Start { id } => {
            client.start_bare_metal(&id).await?;
            print_success(&format!("Bare metal server {} start initiated", id));
        }

        BareMetalCommands::Stop { id } => {
            client.halt_bare_metal(&id).await?;
            print_success(&format!("Bare metal server {} stop initiated", id));
        }

        BareMetalCommands::Reboot { id } => {
            client.reboot_bare_metal(&id).await?;
            print_success(&format!("Bare metal server {} reboot initiated", id));
        }

        BareMetalCommands::Reinstall { id, hostname } => {
            let request = ReinstallBareMetalRequest { hostname };
            let bare_metal = client.reinstall_bare_metal(&id, request).await?;
            print_success(&format!("Bare metal server {} reinstall initiated", id));
            print_output(&bare_metal, output);
        }

        BareMetalCommands::Bandwidth { id } => {
            let bandwidth = client.get_bare_metal_bandwidth(&id).await?;
            print_output(&bandwidth, output);
        }

        BareMetalCommands::Upgrades { id } => {
            let upgrades = client.get_bare_metal_upgrades(&id).await?;
            print_output(&upgrades, output);
        }

        BareMetalCommands::UserData { id } => {
            let user_data = client.get_bare_metal_user_data(&id).await?;
            print_output(&user_data, output);
        }

        BareMetalCommands::Vnc { id } => {
            let vnc = client.get_bare_metal_vnc(&id).await?;
            print_output(&vnc, output);
        }

        BareMetalCommands::Ipv4(ipv4_args) => {
            handle_bare_metal_ipv4(ipv4_args.command, client, output).await?;
        }

        BareMetalCommands::Ipv6(ipv6_args) => {
            handle_bare_metal_ipv6(ipv6_args.command, client, output).await?;
        }

        BareMetalCommands::Vpc(vpc_args) => {
            handle_bare_metal_vpc(vpc_args.command, client, output).await?;
        }

        BareMetalCommands::Vpc2(vpc2_args) => {
            handle_bare_metal_vpc2(vpc2_args.command, client, output).await?;
        }

        BareMetalCommands::Bulk(bulk_args) => {
            handle_bare_metal_bulk(bulk_args.command, client).await?;
        }
    }

    Ok(())
}

async fn delete_bare_metal_impl<FDelete, FDeleteFut, FWait, FWaitFut>(
    id: &str,
    delete_fn: FDelete,
    wait: bool,
    wait_fn: FWait,
) -> VultrResult<()>
where
    FDelete: FnOnce() -> FDeleteFut,
    FDeleteFut: Future<Output = VultrResult<()>>,
    FWait: FnOnce() -> FWaitFut,
    FWaitFut: Future<Output = VultrResult<()>>,
{
    delete_fn().await?;
    print_success(&format!("Bare metal server {} deletion initiated", id));
    if wait {
        wait_fn().await?;
    }
    Ok(())
}

async fn handle_bare_metal_ipv4(
    command: BareMetalIpv4Commands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match command {
        BareMetalIpv4Commands::List { id } => {
            let ipv4s = client.list_bare_metal_ipv4(&id).await?;
            print_output(&ipv4s, output);
        }

        BareMetalIpv4Commands::Reverse { id, ip, reverse } => {
            let request = SetBareMetalReverseIpv4Request { ip, reverse };
            client.set_bare_metal_reverse_ipv4(&id, request).await?;
            print_success("Reverse DNS set for IPv4");
        }

        BareMetalIpv4Commands::DefaultReverse { id, ip } => {
            let request = SetBareMetalDefaultReverseIpv4Request { ip };
            client
                .set_bare_metal_default_reverse_ipv4(&id, request)
                .await?;
            print_success("Default reverse DNS set for IPv4");
        }
    }

    Ok(())
}

async fn handle_bare_metal_ipv6(
    command: BareMetalIpv6Commands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match command {
        BareMetalIpv6Commands::List { id } => {
            let ipv6s = client.list_bare_metal_ipv6(&id).await?;
            print_output(&ipv6s, output);
        }

        BareMetalIpv6Commands::Reverse { id, ip, reverse } => {
            let request = SetBareMetalReverseIpv6Request { ip, reverse };
            client.set_bare_metal_reverse_ipv6(&id, request).await?;
            print_success("Reverse DNS set for IPv6");
        }

        BareMetalIpv6Commands::DeleteReverse { id, ip } => {
            client.delete_bare_metal_reverse_ipv6(&id, &ip).await?;
            print_success("Reverse DNS deleted for IPv6");
        }
    }

    Ok(())
}

async fn handle_bare_metal_vpc(
    command: BareMetalVpcCommands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match command {
        BareMetalVpcCommands::List { id } => {
            let vpcs = client.list_bare_metal_vpcs(&id).await?;
            print_output(&vpcs, output);
        }

        BareMetalVpcCommands::Attach { id, vpc_id } => {
            let request = AttachBareMetalVpcRequest { vpc_id };
            client.attach_bare_metal_vpc(&id, request).await?;
            print_success("VPC attached to bare metal server");
        }

        BareMetalVpcCommands::Detach { id, vpc_id } => {
            let request = DetachBareMetalVpcRequest { vpc_id };
            client.detach_bare_metal_vpc(&id, request).await?;
            print_success("VPC detached from bare metal server");
        }
    }

    Ok(())
}

async fn handle_bare_metal_vpc2(
    command: BareMetalVpc2Commands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match command {
        BareMetalVpc2Commands::List { id } => {
            let vpc2s = client.list_bare_metal_vpc2s(&id).await?;
            print_output(&vpc2s, output);
        }

        BareMetalVpc2Commands::Attach {
            id,
            vpc_id,
            ip_address,
        } => {
            let request = AttachBareMetalVpc2Request { vpc_id, ip_address };
            client.attach_bare_metal_vpc2(&id, request).await?;
            print_success("VPC2 attached to bare metal server");
        }

        BareMetalVpc2Commands::Detach { id, vpc_id } => {
            let request = DetachBareMetalVpc2Request { vpc_id };
            client.detach_bare_metal_vpc2(&id, request).await?;
            print_success("VPC2 detached from bare metal server");
        }
    }

    Ok(())
}

async fn handle_bare_metal_bulk(
    command: BareMetalBulkCommands,
    client: &VultrClient,
) -> VultrResult<()> {
    match command {
        BareMetalBulkCommands::Start { ids } => {
            let request = BulkBareMetalRequest { baremetal_ids: ids };
            client.bulk_start_bare_metals(request).await?;
            print_success("Bulk start initiated for bare metal servers");
        }

        BareMetalBulkCommands::Stop { ids } => {
            let request = BulkBareMetalRequest { baremetal_ids: ids };
            client.bulk_halt_bare_metals(request).await?;
            print_success("Bulk stop initiated for bare metal servers");
        }

        BareMetalBulkCommands::Reboot { ids } => {
            let request = BulkBareMetalRequest { baremetal_ids: ids };
            client.bulk_reboot_bare_metals(request).await?;
            print_success("Bulk reboot initiated for bare metal servers");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_delete_bare_metal_wait_calls_wait_fn() {
        let delete_called = AtomicBool::new(false);
        let wait_called = AtomicBool::new(false);

        let result = delete_bare_metal_impl(
            "bm-123",
            || async {
                delete_called.store(true, Ordering::SeqCst);
                Ok(())
            },
            true,
            || async {
                wait_called.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;

        assert!(result.is_ok());
        assert!(delete_called.load(Ordering::SeqCst));
        assert!(wait_called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_delete_bare_metal_no_wait_skips_wait_fn() {
        let delete_called = AtomicBool::new(false);
        let wait_called = AtomicBool::new(false);

        let result = delete_bare_metal_impl(
            "bm-123",
            || async {
                delete_called.store(true, Ordering::SeqCst);
                Ok(())
            },
            false,
            || async {
                wait_called.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;

        assert!(result.is_ok());
        assert!(delete_called.load(Ordering::SeqCst));
        assert!(!wait_called.load(Ordering::SeqCst));
    }
}
