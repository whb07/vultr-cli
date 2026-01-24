//! Application command handlers

use vultr_api::VultrClient;
use crate::commands::{ApplicationsArgs, ApplicationsCommands};
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_output::print_output;

pub async fn handle_applications(
    args: ApplicationsArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        None | Some(ApplicationsCommands::List) => {
            let applications = client.list_applications().await?;
            print_output(&applications, output);
        }
        Some(ApplicationsCommands::Variables { image_id }) => {
            let variables = client.list_app_variables(&image_id).await?;
            print_output(&variables, output);
        }
    }
    Ok(())
}
