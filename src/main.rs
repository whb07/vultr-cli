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
                    handle_ssh_key(args, &client, output, skip_confirm, &wait_opts).await
                }
                Commands::StartupScript(args) => {
                    handle_startup_script(args, &client, output, skip_confirm, &wait_opts).await
                }
                Commands::Snapshot(args) => {
                    handle_snapshot(args, &client, output, skip_confirm, cli.wait, &wait_opts).await
                }
                Commands::BlockStorage(args) => {
                    handle_block_storage(args, &client, output, skip_confirm, cli.wait, &wait_opts)
                        .await
                }
                Commands::Firewall(args) => {
                    handle_firewall(args, &client, output, skip_confirm, &wait_opts).await
                }
                Commands::Vpc(args) => handle_vpc(args, &client, output, skip_confirm).await,
                Commands::Kubernetes(args) => {
                    handle_kubernetes(args, &client, output, skip_confirm).await
                }
                Commands::Regions => handle_regions(&client, output).await,
                Commands::Plans(args) => {
                    handle_plans(&client, output, args.plan_type.as_deref()).await
                }
                Commands::Os => handle_os(&client, output).await,
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
