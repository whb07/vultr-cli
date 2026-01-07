//! Application command handlers

use crate::api::VultrClient;
use crate::config::OutputFormat;
use crate::error::VultrResult;
use crate::output::print_output;

pub async fn handle_applications(client: &VultrClient, output: OutputFormat) -> VultrResult<()> {
    let applications = client.list_applications().await?;
    print_output(&applications, output);
    Ok(())
}
