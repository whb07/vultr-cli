//! Reference data command handlers (regions, plans, OS)

use crate::api::VultrClient;
use crate::config::OutputFormat;
use crate::error::VultrResult;
use crate::output::print_output;

pub async fn handle_regions(client: &VultrClient, output: OutputFormat) -> VultrResult<()> {
    let regions = client.list_regions().await?;
    print_output(&regions, output);
    Ok(())
}

pub async fn handle_plans(
    client: &VultrClient,
    output: OutputFormat,
    plan_type: Option<&str>,
    bare_metal: bool,
) -> VultrResult<()> {
    if bare_metal {
        let plans = client.list_bare_metal_plans().await?;
        print_output(&plans, output);
    } else {
        let plans = client.list_plans(plan_type).await?;
        print_output(&plans, output);
    }
    Ok(())
}

pub async fn handle_os(client: &VultrClient, output: OutputFormat) -> VultrResult<()> {
    let os_list = client.list_os().await?;
    print_output(&os_list, output);
    Ok(())
}
