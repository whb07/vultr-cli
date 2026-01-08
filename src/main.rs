//! Vultr CLI - A production-ready command-line interface for managing Vultr cloud resources

mod api;
mod commands;
mod config;
mod error;
mod handlers;
mod models;
mod output;

use crate::api::{VultrClient, WaitOptions};
use crate::commands::*;
use crate::config::resolve_api_key;
use crate::error::{VultrError, VultrResult};
use crate::handlers::*;
use crate::output::print_error;

use clap::Parser;

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
        Commands::Config(args) => handle_config(args, &cli.profile),
        _ => {
            // Load config and merge with CLI flags
            let cfg = crate::config::Config::load()?;
            let effective_profile = if cli.profile == "default" && cfg.default_profile != "default"
            {
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

            // Confirm destructive operations unless user passed --yes or config disables confirmations
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
                Commands::SshKey(args) => {
                    handle_ssh_key(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::StartupScript(args) => {
                    handle_startup_script(args, &client, output, skip_confirm, cli.wait, &wait_opts)
                        .await
                }
                Commands::Snapshot(args) => {
                    handle_snapshot(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::Backup(args) => handle_backup(args, &client, output).await,
                Commands::BareMetal(args) => {
                    handle_bare_metal(args, &client, output, skip_confirm).await
                }
                Commands::Iso(args) => handle_iso(args, &client, output, skip_confirm).await,
                Commands::BlockStorage(args) => {
                    handle_block_storage(args, &client, output, skip_confirm, cli.wait, &wait_opts)
                        .await
                }
                Commands::ObjectStorage(args) => {
                    handle_object_storage(args, &client, output, skip_confirm).await
                }
                Commands::StorageGateway(args) => {
                    handle_storage_gateway(args, &client, output, skip_confirm).await
                }
                Commands::Vfs(args) => handle_vfs(args, &client, output, skip_confirm).await,
                Commands::Firewall(args) => {
                    handle_firewall(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::Vpc(args) => handle_vpc(args, &client, output, skip_confirm).await,
                Commands::Vpc2(args) => handle_vpc2(args, &client, output, skip_confirm).await,
                Commands::PrivateNetwork(args) => {
                    handle_private_network(args, &client, output, skip_confirm).await
                }
                Commands::Kubernetes(args) => {
                    handle_kubernetes(args, &client, output, skip_confirm).await
                }
                Commands::LoadBalancer(args) => {
                    handle_load_balancer(args, &client, output, skip_confirm).await
                }
                Commands::Database(args) => {
                    handle_database(args, &client, output, skip_confirm).await
                }
                Commands::Cdn(args) => handle_cdn(args, &client, output, skip_confirm).await,
                Commands::Dns(args) => handle_dns(args, &client, output, skip_confirm).await,
                Commands::Registry(args) => {
                    handle_registry(args, &client, output, skip_confirm).await
                }
                Commands::ReservedIp(args) => {
                    handle_reserved_ip(args, &client, output, skip_confirm).await
                }
                Commands::Regions => handle_regions(&client, output).await,
                Commands::Plans(args) => {
                    handle_plans(
                        &client,
                        output,
                        args.plan_type.as_deref(),
                        args.bare_metal,
                        args.price,
                        args.region.as_deref(),
                    )
                    .await
                }
                Commands::Os => handle_os(&client, output).await,
                Commands::Applications(args) => handle_applications(args, &client, output).await,
                Commands::Inference(args) => {
                    handle_inference(args, &client, output, skip_confirm).await
                }
                Commands::Logs(args) => handle_logs(args, &client, output).await,
                Commands::Account(args) => handle_account(args, &client, output).await,
                Commands::Billing(args) => handle_billing(args, &client, output).await,
                Commands::User(args) => handle_user(args, &client, output, skip_confirm).await,
                Commands::Subaccount(args) => {
                    handle_subaccount(args, &client, output, skip_confirm).await
                }
                _ => unreachable!(),
            }
        }
    }
}

fn generate_completions(shell: Shell) {
    use clap::CommandFactory;
    let shell: clap_complete::Shell = shell.into();
    clap_complete::generate(
        shell,
        &mut Cli::command(),
        "vultr-cli",
        &mut std::io::stdout(),
    );
}
