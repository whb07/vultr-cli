//! Vultr CLI - A production-ready command-line interface for managing Vultr cloud resources

mod api;
mod commands;
mod config;
mod error;
mod models;
mod output;

use crate::api::{VultrClient, WaitOptions};
use crate::commands::*;
use crate::config::{resolve_api_key, OutputFormat, SecureStorage};
use crate::error::{VultrError, VultrResult};
use crate::models::*;
use crate::output::*;

use clap::Parser;
use dialoguer::Confirm;
use std::io::Write;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    let cli = Cli::parse();
    let result = run(cli).await;

    if let Err(e) = result {
        print_error(&e.to_string());
        std::process::exit(e.exit_code());
    }
}

async fn run(cli: Cli) -> VultrResult<()> {
    match cli.command {
        Commands::Auth(args) => handle_auth(args, &cli.profile).await,
        Commands::Completions(args) => {
            generate_completions(args.shell);
            Ok(())
        }
        _ => {
            // Load config and merge with CLI flags.
            let cfg = crate::config::Config::load()?;
            let effective_profile = if cli.profile == "default" && cfg.default_profile != "default" {
                cfg.default_profile.clone()
            } else {
                cli.profile.clone()
            };
            let profile_cfg = cfg.profiles.get(&effective_profile);

            let output = cli
                .output
                .or_else(|| profile_cfg.and_then(|p| p.output_format))
                .unwrap_or(cfg.settings.output_format);

            let wait_timeout = cli.wait_timeout.unwrap_or(cfg.settings.wait_timeout);
            let poll_interval = cli.poll_interval.unwrap_or(cfg.settings.poll_interval);

            // Confirm destructive operations unless user passed --yes or config disables confirmations.
            let skip_confirm = cli.yes || !cfg.settings.confirm_destructive;

            let api_key = resolve_api_key(cli.api_key.as_deref(), &effective_profile)?
                .ok_or_else(|| VultrError::AuthenticationRequired)?;
            let client = VultrClient::new(
                api_key,
                cfg.settings.http.timeout,
                cfg.settings.http.max_retries,
                cfg.settings.http.backoff_initial_ms,
                cfg.settings.http.backoff_max_ms,
            )?;

            let wait_opts = WaitOptions {
                timeout: wait_timeout,
                poll_interval,
                show_progress: true,
            };

            match cli.command {

                Commands::Instance(args) => {
                    handle_instance(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::SshKey(args) => handle_ssh_key(args, &client, output, skip_confirm, &wait_opts).await,
                Commands::StartupScript(args) => {
                    handle_startup_script(args, &client, output, skip_confirm, &wait_opts).await
                }
                Commands::Snapshot(args) => {
                    handle_snapshot(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::BlockStorage(args) => {
                    handle_block_storage(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::Firewall(args) => handle_firewall(args, &client, output, skip_confirm, &wait_opts).await,
                Commands::Vpc(args) => handle_vpc(args, &client, output, skip_confirm, &wait_opts).await,
                Commands::Regions => {
                    let regions = client.list_regions().await?;
                    print_output(&regions, output);
                    Ok(())
                }
                Commands::Plans(args) => {
                    let plans = client.list_plans(args.plan_type.as_deref()).await?;
                    print_output(&plans, output);
                    Ok(())
                }
                Commands::Os => {
                    let os_list = client.list_os().await?;
                    print_output(&os_list, output);
                    Ok(())
                }
                _ => unreachable!(),
            }
        }
    }
}

async fn handle_auth(args: AuthArgs, profile: &str) -> VultrResult<()> {
    // Load config to apply HTTP retry/timeout policy during auth checks.
    let cfg = crate::config::Config::load().unwrap_or_default();
    match args.command {
        AuthCommands::Login(login_args) => {
            let api_key = if let Some(key) = login_args.api_key {
                key
            } else {
                let key = dialoguer::Password::new()
                    .with_prompt("Enter your Vultr API key")
                    .allow_empty_password(false)
                    .interact()?;
                key.trim().to_string()
            };
            if api_key.is_empty() { return Err(VultrError::InvalidApiKey); }
            let client = VultrClient::new(
                api_key.clone(),
                cfg.settings.http.timeout,
                cfg.settings.http.max_retries,
                cfg.settings.http.backoff_initial_ms,
                cfg.settings.http.backoff_max_ms,
            )?;
            // A light call used as a credential sanity check.
            client.list_regions().await?;
            SecureStorage::store_token(profile, &api_key)?;
            print_success(&format!("API key stored securely for profile '{}'", profile));
            Ok(())
        }
        AuthCommands::Logout => {
            SecureStorage::delete_token(profile)?;
            print_success(&format!("API key removed for profile '{}'", profile));
            Ok(())
        }
        AuthCommands::Status => {
            match SecureStorage::get_token(profile)? {
                Some(_) => print_success(&format!("Authenticated (profile: {})", profile)),
                None => {
                    if std::env::var("VULTR_API_KEY").is_ok() {
                        print_info("Using API key from VULTR_API_KEY environment variable");
                    } else {
                        print_warning("Not authenticated. Run 'vultr-cli auth login' to authenticate.");
                    }
                }
            }
            Ok(())
        }
    }
}

async fn handle_instance(args: InstanceArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        InstanceCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_instances(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (instances, _) = client.list_instances(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&instances, output);
            }
        }
        InstanceCommands::Get { id } => {
            let instance = client.get_instance(&id).await?;
            print_output(&instance, output);
        }
        InstanceCommands::Create(args) => {
            let request = CreateInstanceRequest {
                region: args.region, plan: args.plan, os_id: args.os_id, snapshot_id: args.snapshot_id,
                app_id: args.app_id, label: args.label, hostname: args.hostname, sshkey_id: args.ssh_keys,
                script_id: args.script_id, enable_ipv6: if args.enable_ipv6 { Some(true) } else { None },
                backups: if args.backups { Some("enabled".into()) } else { None },
                ddos_protection: if args.ddos_protection { Some(true) } else { None },
                attach_vpc: args.vpc, firewall_group_id: args.firewall_group_id, tags: args.tags,
                user_data: args.user_data.as_deref().map(|v| {
                    let raw = read_file_or_bytes(v)?;
                    Ok::<String, VultrError>(base64::engine::general_purpose::STANDARD.encode(raw))
                }).transpose()?, ..Default::default()
            };
            let instance = client.create_instance(request).await?;
            print_success(&format!("Instance {} created", instance.id));
            if wait {
                let instance = api::wait_for_instance_ready(client, &instance.id, wait_opts).await?;
                print_output(&instance, output);
            } else { print_output(&instance, output); }
        }
        InstanceCommands::Update(args) => {
            let request = UpdateInstanceRequest {
                label: args.label, plan: args.plan, firewall_group_id: args.firewall_group_id, tags: args.tags,
                backups: args.backups.map(|b| if b { "enabled" } else { "disabled" }.into()),
                ddos_protection: args.ddos_protection, ..Default::default()
            };
            let instance = client.update_instance(&args.id, request).await?;
            print_success(&format!("Instance {} updated", instance.id));
            print_output(&instance, output);
        }
        InstanceCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("instance", &id)? { return Err(VultrError::Cancelled); }
            client.delete_instance(&id).await?;
            print_success(&format!("Instance {} deletion initiated", id));
            api::verify_instance_deleted(client, &id, wait_opts).await?;
        }
        InstanceCommands::Start { id } => {
            client.start_instance(&id).await?;
            print_success(&format!("Instance {} start initiated", id));
            if wait { let i = api::wait_for_instance_ready(client, &id, wait_opts).await?; print_output(&i, output); }
        }
        InstanceCommands::Stop { id } => {
            client.halt_instance(&id).await?;
            print_success(&format!("Instance {} stop initiated", id));
            if wait { let i = api::wait_for_instance_stopped(client, &id, wait_opts).await?; print_output(&i, output); }
        }
        InstanceCommands::Reboot { id } => {
            client.reboot_instance(&id).await?;
            print_success(&format!("Instance {} reboot initiated", id));
            if wait { let i = api::wait_for_instance_ready(client, &id, wait_opts).await?; print_output(&i, output); }
        }
        InstanceCommands::Reinstall { id, hostname } => {
            if !skip_confirm && !Confirm::new().with_prompt(format!("Reinstall {}? All data will be lost.", id)).default(false).interact().unwrap_or(false) {
                return Err(VultrError::Cancelled);
            }
            let instance = client.reinstall_instance(&id, ReinstallInstanceRequest { hostname }).await?;
            print_success(&format!("Instance {} reinstall initiated", id));
            if wait { let i = api::wait_for_instance_ready(client, &instance.id, wait_opts).await?; print_output(&i, output); }
            else { print_output(&instance, output); }
        }
    }
    Ok(())
}

async fn handle_ssh_key(args: SshKeyArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        SshKeyCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_ssh_keys(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (keys, _) = client.list_ssh_keys(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&keys, output);
            }
        }
        SshKeyCommands::Get { id } => { let key = client.get_ssh_key(&id).await?; print_output(&key, output); }
        SshKeyCommands::Create { name, key } => {
            let ssh_key = read_file_or_string(&key)?;
            let key = client.create_ssh_key(CreateSshKeyRequest { name, ssh_key }).await?;
            print_success(&format!("SSH key {} created", key.id));
            print_output(&key, output);
        }
        SshKeyCommands::Update { id, name, key } => {
            client.update_ssh_key(&id, UpdateSshKeyRequest { name, ssh_key: key.map(|k| read_file_or_string(&k)).transpose()? }).await?;
            print_success(&format!("SSH key {} updated", id));
        }
        SshKeyCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("SSH key", &id)? { return Err(VultrError::Cancelled); }
            client.delete_ssh_key(&id).await?;
            print_success(&format!("SSH key {} deletion initiated", id));
            api::verify_ssh_key_deleted(client, &id, wait_opts).await?;
        }
    }
    Ok(())
}

async fn handle_startup_script(args: StartupScriptArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        StartupScriptCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_startup_scripts(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (scripts, _) = client.list_startup_scripts(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&scripts, output);
            }
        }
        StartupScriptCommands::Get { id } => { let s = client.get_startup_script(&id).await?; print_output(&s, output); }
        StartupScriptCommands::Create { name, script, script_type } => {
            let content = read_file_or_string(&script)?;
            let req = CreateStartupScriptRequest::new(name, &content, Some(script_type.parse().map_err(|e: String| VultrError::InvalidInput(e))?));
            let s = client.create_startup_script(req).await?;
            print_success(&format!("Startup script {} created", s.id));
            print_output(&s, output);
        }
        StartupScriptCommands::Update { id, name, script, script_type } => {
            let mut req = UpdateStartupScriptRequest { name, script_type, ..Default::default() };
            if let Some(s) = script { req = req.with_raw_script(&read_file_or_string(&s)?); }
            client.update_startup_script(&id, req).await?;
            print_success(&format!("Startup script {} updated", id));
        }
        StartupScriptCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("startup script", &id)? { return Err(VultrError::Cancelled); }
            client.delete_startup_script(&id).await?;
            print_success(&format!("Startup script {} deletion initiated", id));
            api::verify_startup_script_deleted(client, &id, wait_opts).await?;
        }
    }
    Ok(())
}

async fn handle_snapshot(args: SnapshotArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        SnapshotCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_snapshots(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (snaps, _) = client.list_snapshots(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&snaps, output);
            }
        }
        SnapshotCommands::Get { id } => { let s = client.get_snapshot(&id).await?; print_output(&s, output); }
        SnapshotCommands::Create { instance_id, description } => {
            let s = client.create_snapshot(CreateSnapshotRequest { instance_id, description }).await?;
            print_success(&format!("Snapshot {} creation initiated", s.id));
            if wait { let s = api::wait_for_snapshot_complete(client, &s.id, wait_opts).await?; print_output(&s, output); }
            else { print_output(&s, output); }
        }
        SnapshotCommands::CreateFromUrl { url, description } => {
            let s = client.create_snapshot_from_url(CreateSnapshotFromUrlRequest { url, description }).await?;
            print_success(&format!("Snapshot {} creation from URL initiated", s.id));
            if wait { let s = api::wait_for_snapshot_complete(client, &s.id, wait_opts).await?; print_output(&s, output); }
            else { print_output(&s, output); }
        }
        SnapshotCommands::Update { id, description } => {
            client.update_snapshot(&id, UpdateSnapshotRequest { description: Some(description) }).await?;
            print_success(&format!("Snapshot {} updated", id));
        }
        SnapshotCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("snapshot", &id)? { return Err(VultrError::Cancelled); }
            client.delete_snapshot(&id).await?;
            print_success(&format!("Snapshot {} deletion initiated", id));
            api::verify_snapshot_deleted(client, &id, wait_opts).await?;
        }
    }
    Ok(())
}

async fn handle_block_storage(args: BlockStorageArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        BlockStorageCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_block_storage(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (blocks, _) = client.list_block_storage(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&blocks, output);
            }
        }
        BlockStorageCommands::Get { id } => { let b = client.get_block_storage(&id).await?; print_output(&b, output); }
        BlockStorageCommands::Create { region, size, label, block_type } => {
            let b = client.create_block_storage(CreateBlockStorageRequest { region, size_gb: size, label, block_type }).await?;
            print_success(&format!("Block storage {} created", b.id));
            if wait { let b = api::wait_for_block_storage_active(client, &b.id, wait_opts).await?; print_output(&b, output); }
            else { print_output(&b, output); }
        }
        BlockStorageCommands::Update { id, label, size } => {
            client.update_block_storage(&id, UpdateBlockStorageRequest { label, size_gb: size }).await?;
            print_success(&format!("Block storage {} updated", id));
        }
        BlockStorageCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("block storage", &id)? { return Err(VultrError::Cancelled); }
            client.delete_block_storage(&id).await?;
            print_success(&format!("Block storage {} deletion initiated", id));
            api::verify_block_storage_deleted(client, &id, wait_opts).await?;
        }
        BlockStorageCommands::Attach { id, instance_id, live } => {
            client.attach_block_storage(&id, AttachBlockStorageRequest { instance_id: instance_id.clone(), live: if live { Some(true) } else { None } }).await?;
            print_success(&format!("Block storage {} attached to {}", id, instance_id));
        }
        BlockStorageCommands::Detach { id, live } => {
            client.detach_block_storage(&id, DetachBlockStorageRequest { live: if live { Some(true) } else { None } }).await?;
            print_success(&format!("Block storage {} detached", id));
        }
    }
    Ok(())
}

async fn handle_firewall(args: FirewallArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        FirewallCommands::Group(g) => handle_firewall_group(g, client, output, skip_confirm, wait_opts).await,
        FirewallCommands::Rule(r) => handle_firewall_rule(r, client, output, skip_confirm, wait_opts).await,
    }
}

async fn handle_firewall_group(args: FirewallGroupArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        FirewallGroupCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_firewall_groups(Some(list_args.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (groups, _) = client.list_firewall_groups(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
                print_output(&groups, output);
            }
        }
        FirewallGroupCommands::Get { id } => { let g = client.get_firewall_group(&id).await?; print_output(&g, output); }
        FirewallGroupCommands::Create { description } => {
            let g = client.create_firewall_group(CreateFirewallGroupRequest { description }).await?;
            print_success(&format!("Firewall group {} created", g.id));
            print_output(&g, output);
        }
        FirewallGroupCommands::Update { id, description } => {
            client.update_firewall_group(&id, UpdateFirewallGroupRequest { description: Some(description) }).await?;
            print_success(&format!("Firewall group {} updated", id));
        }
        FirewallGroupCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("firewall group", &id)? { return Err(VultrError::Cancelled); }
            client.delete_firewall_group(&id).await?;
            print_success(&format!("Firewall group {} deletion initiated", id));
            api::verify_firewall_group_deleted(client, &id, wait_opts).await?;
        }
    }
    Ok(())
}

async fn handle_firewall_rule(args: FirewallRuleArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool, wait_opts: &WaitOptions) -> VultrResult<()> {
    match args.command {
        FirewallRuleCommands::List { group_id, list } => {
            if list.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client.list_firewall_rules(&group_id, Some(list.per_page), cursor.as_deref()).await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() { break; }
                }
                print_output(&all, output);
            } else {
                let (rules, _) = client.list_firewall_rules(&group_id, Some(list.per_page), list.cursor.as_deref()).await?;
                print_output(&rules, output);
            }
        }
        FirewallRuleCommands::Get { group_id, rule_id } => {
            let r = client.get_firewall_rule(&group_id, rule_id).await?;
            print_output(&r, output);
        }
        FirewallRuleCommands::Create { group_id, ip_type, protocol, subnet, subnet_size, port, source, notes } => {
            let r = client.create_firewall_rule(&group_id, CreateFirewallRuleRequest { ip_type, protocol, subnet, subnet_size, port, source, notes }).await?;
            print_success(&format!("Firewall rule {} created in group {}", r.id, group_id));
            print_output(&r, output);
        }
        FirewallRuleCommands::Delete { group_id, rule_id } => {
            if !skip_confirm && !Confirm::new().with_prompt(format!("Delete rule {} from group {}?", rule_id, group_id)).default(false).interact().unwrap_or(false) {
                return Err(VultrError::Cancelled);
            }
            client.delete_firewall_rule(&group_id, rule_id).await?;
            print_success(&format!("Firewall rule {} deletion initiated from group {}", rule_id, group_id));
            api::verify_firewall_rule_deleted(client, &group_id, rule_id, wait_opts).await?;
        }
    }
    Ok(())
}

async fn handle_vpc(args: VpcArgs, client: &VultrClient, output: OutputFormat, skip_confirm: bool) -> VultrResult<()> {
    match args.command {
        VpcCommands::List(list_args) => {
            let (vpcs, _) = client.list_vpcs(Some(list_args.per_page), list_args.cursor.as_deref()).await?;
            print_output(&vpcs, output);
        }
        VpcCommands::Get { id } => { let v = client.get_vpc(&id).await?; print_output(&v, output); }
        VpcCommands::Create { region, description, subnet, subnet_mask } => {
            let v = client.create_vpc(CreateVpcRequest { region, description, v4_subnet: subnet, v4_subnet_mask: subnet_mask }).await?;
            print_success(&format!("VPC {} created", v.id));
            print_output(&v, output);
        }
        VpcCommands::Update { id, description } => {
            client.update_vpc(&id, UpdateVpcRequest { description: Some(description) }).await?;
            print_success(&format!("VPC {} updated", id));
        }
        VpcCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("VPC", &id)? { return Err(VultrError::Cancelled); }
            client.delete_vpc(&id).await?;
            print_success(&format!("VPC {} deleted", id));
        }
    }
    Ok(())
}

fn read_file_or_string(input: &str) -> VultrResult<String> {
    if let Some(path) = input.strip_prefix('@') {
        std::fs::read_to_string(path).map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
    } else { Ok(input.to_string()) }
}


fn read_file_or_bytes(input: &str) -> VultrResult<Vec<u8>> {
    if let Some(path) = input.strip_prefix('@') {
        std::fs::read(path).map_err(|e| VultrError::InvalidInput(format!("Failed to read '{}': {}", path, e)))
    } else {
        Ok(input.as_bytes().to_vec())
    }
}

fn confirm_delete(resource_type: &str, id: &str) -> VultrResult<bool> {
    Ok(Confirm::new().with_prompt(format!("Delete {} {}?", resource_type, id)).default(false).interact().unwrap_or(false))
}

fn generate_completions(shell: Shell) {
    use clap::CommandFactory;
    let shell: clap_complete::Shell = shell.into();
    clap_complete::generate(shell, &mut Cli::command(), "vultr-cli", &mut std::io::stdout());
}
