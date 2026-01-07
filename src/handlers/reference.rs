//! Reference data command handlers (regions, plans, OS)

use crate::api::VultrClient;
use crate::commands::PriceMode;
use crate::config::OutputFormat;
use crate::error::VultrResult;
use crate::output::{print_bare_metal_plans, print_output};

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
    price: PriceMode,
    region: Option<&str>,
) -> VultrResult<()> {
    let region = region.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    if bare_metal {
        let mut plans = client.list_bare_metal_plans().await?;
        if let Some(region) = region {
            plans.retain(|plan| {
                plan.locations
                    .iter()
                    .any(|loc| loc.eq_ignore_ascii_case(region))
            });
        }
        print_bare_metal_plans(&plans, output, price == PriceMode::Monthly);
    } else {
        let mut plans = client.list_plans(plan_type).await?;
        if let Some(region) = region {
            plans.retain(|plan| {
                plan.locations
                    .iter()
                    .any(|loc| loc.eq_ignore_ascii_case(region))
            });
        }
        print_output(&plans, output);
    }
    Ok(())
}

pub async fn handle_os(client: &VultrClient, output: OutputFormat) -> VultrResult<()> {
    let os_list = client.list_os().await?;
    print_output(&os_list, output);
    Ok(())
}
