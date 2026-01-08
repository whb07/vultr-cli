//! Reference data command handlers (regions, plans, OS)

use crate::api::VultrClient;
use crate::commands::{OsArgs, PriceMode};
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

pub async fn handle_os(args: OsArgs, client: &VultrClient, output: OutputFormat) -> VultrResult<()> {
    let mut os_list = client.list_os().await?;

    if let Some(id) = args.id {
        os_list.retain(|os| os.id == id);
    }
    if let Some(ref family) = args.family {
        let needle = family.trim().to_ascii_lowercase();
        if !needle.is_empty() {
            os_list.retain(|os| {
                os.family
                    .as_deref()
                    .unwrap_or("")
                    .to_ascii_lowercase()
                    == needle
            });
        }
    }
    if let Some(ref arch) = args.arch {
        let needle = arch.trim().to_ascii_lowercase();
        if !needle.is_empty() {
            os_list.retain(|os| {
                os.arch
                    .as_deref()
                    .unwrap_or("")
                    .to_ascii_lowercase()
                    == needle
            });
        }
    }
    if let Some(ref name) = args.name {
        let needle = name.trim().to_ascii_lowercase();
        if !needle.is_empty() {
            os_list.retain(|os| {
                os.name
                    .as_deref()
                    .unwrap_or("")
                    .to_ascii_lowercase()
                    .contains(&needle)
            });
        }
    }

    print_output(&os_list, output);
    Ok(())
}
