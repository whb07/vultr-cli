//! Account command handlers

use crate::commands::{AccountArgs, AccountCommands};
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_output::print_output;

pub async fn handle_account(
    args: AccountArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        AccountCommands::Info => {
            let account = client.get_account().await?;
            print_output(&account, output);
        }

        AccountCommands::Bgp => {
            let bgp = client.get_account_bgp().await?;
            print_output(&bgp, output);
        }

        AccountCommands::Bandwidth => {
            let bandwidth = client.get_account_bandwidth().await?;
            print_output(&bandwidth, output);
        }
    }
    Ok(())
}
