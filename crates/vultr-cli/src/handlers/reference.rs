//! Reference data command handlers (regions, plans, OS)

use crate::commands::{BareMetalPlanSort, OsArgs, PriceMode};
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_models::BareMetalPlan;
use vultr_output::{print_bare_metal_plans, print_output};

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
    sort: BareMetalPlanSort,
    region: Option<&str>,
) -> VultrResult<()> {
    let region = region.map(str::trim).filter(|s| !s.is_empty());

    if bare_metal {
        let mut plans = client.list_bare_metal_plans().await?;
        if let Some(region) = region {
            plans.retain(|plan| {
                plan.locations
                    .iter()
                    .any(|loc| loc.eq_ignore_ascii_case(region))
            });
        }
        if output == OutputFormat::Table {
            sort_bare_metal_plans(&mut plans, sort, &price);
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

fn sort_bare_metal_plans(plans: &mut [BareMetalPlan], sort: BareMetalPlanSort, price: &PriceMode) {
    match sort {
        BareMetalPlanSort::CpuMemory => {
            plans.sort_by(|a, b| {
                let cpu_a = cpu_key(a);
                let cpu_b = cpu_key(b);
                let ram_a = ram_key(a);
                let ram_b = ram_key(b);
                let threads_a = threads_key(a);
                let threads_b = threads_key(b);
                cpu_b
                    .cmp(&cpu_a)
                    .then_with(|| ram_b.cmp(&ram_a))
                    .then_with(|| threads_b.cmp(&threads_a))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
        BareMetalPlanSort::Cpu => {
            plans.sort_by(|a, b| {
                let cpu_a = cpu_key(a);
                let cpu_b = cpu_key(b);
                let threads_a = threads_key(a);
                let threads_b = threads_key(b);
                let ram_a = ram_key(a);
                let ram_b = ram_key(b);
                cpu_b
                    .cmp(&cpu_a)
                    .then_with(|| threads_b.cmp(&threads_a))
                    .then_with(|| ram_b.cmp(&ram_a))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
        BareMetalPlanSort::Memory => {
            plans.sort_by(|a, b| {
                let ram_a = ram_key(a);
                let ram_b = ram_key(b);
                let cpu_a = cpu_key(a);
                let cpu_b = cpu_key(b);
                let threads_a = threads_key(a);
                let threads_b = threads_key(b);
                ram_b
                    .cmp(&ram_a)
                    .then_with(|| cpu_b.cmp(&cpu_a))
                    .then_with(|| threads_b.cmp(&threads_a))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
        BareMetalPlanSort::Price => {
            plans.sort_by(|a, b| {
                let price_a = price_key(a, price);
                let price_b = price_key(b, price);
                let cpu_a = cpu_key(a);
                let cpu_b = cpu_key(b);
                let ram_a = ram_key(a);
                let ram_b = ram_key(b);
                price_b
                    .total_cmp(&price_a)
                    .then_with(|| cpu_b.cmp(&cpu_a))
                    .then_with(|| ram_b.cmp(&ram_a))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
        BareMetalPlanSort::Disk => {
            plans.sort_by(|a, b| {
                let disk_a = disk_key(a);
                let disk_b = disk_key(b);
                let disk_count_a = disk_count_key(a);
                let disk_count_b = disk_count_key(b);
                let cpu_a = cpu_key(a);
                let cpu_b = cpu_key(b);
                let ram_a = ram_key(a);
                let ram_b = ram_key(b);
                disk_b
                    .cmp(&disk_a)
                    .then_with(|| disk_count_b.cmp(&disk_count_a))
                    .then_with(|| cpu_b.cmp(&cpu_a))
                    .then_with(|| ram_b.cmp(&ram_a))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
        BareMetalPlanSort::Id => {
            plans.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }
}

fn cpu_key(plan: &BareMetalPlan) -> i32 {
    plan.cpu_count
        .or(plan.cpu_cores)
        .or(plan.physical_cpus)
        .unwrap_or(0)
}

fn threads_key(plan: &BareMetalPlan) -> i32 {
    plan.cpu_threads.unwrap_or(0)
}

fn ram_key(plan: &BareMetalPlan) -> i32 {
    plan.ram.unwrap_or(0)
}

fn disk_key(plan: &BareMetalPlan) -> i32 {
    plan.disk.unwrap_or(0)
}

fn disk_count_key(plan: &BareMetalPlan) -> i32 {
    plan.disk_count.unwrap_or(0)
}

fn price_key(plan: &BareMetalPlan, price: &PriceMode) -> f64 {
    match price {
        PriceMode::Monthly => plan.monthly_cost.unwrap_or(0.0),
        PriceMode::Hourly => plan.hourly_cost.unwrap_or(0.0),
    }
}

pub async fn handle_os(
    args: OsArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    let mut os_list = client.list_os().await?;

    if let Some(id) = args.id {
        os_list.retain(|os| os.id == id);
    }
    if let Some(ref family) = args.family {
        let needle = family.trim().to_ascii_lowercase();
        if !needle.is_empty() {
            os_list.retain(|os| os.family.as_deref().unwrap_or("").to_ascii_lowercase() == needle);
        }
    }
    if let Some(ref arch) = args.arch {
        let needle = arch.trim().to_ascii_lowercase();
        if !needle.is_empty() {
            os_list.retain(|os| os.arch.as_deref().unwrap_or("").to_ascii_lowercase() == needle);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn gb_to_mb(gb: i32) -> i32 {
        gb * 1024
    }

    struct PlanSpec {
        id: &'static str,
        cpu_count: i32,
        cpu_threads: i32,
        ram_gb: i32,
        disk_gb: i32,
        disk_count: i32,
        hourly_cost: f64,
        plan_type: &'static str,
    }

    fn plan(spec: PlanSpec) -> BareMetalPlan {
        let PlanSpec {
            id,
            cpu_count,
            cpu_threads,
            ram_gb,
            disk_gb,
            disk_count,
            hourly_cost,
            plan_type,
        } = spec;

        BareMetalPlan {
            id: id.to_string(),
            physical_cpus: None,
            cpu_count: Some(cpu_count),
            cpu_cores: None,
            cpu_threads: Some(cpu_threads),
            cpu_manufacturer: None,
            cpu_model: None,
            cpu_mhz: None,
            ram: Some(gb_to_mb(ram_gb)),
            disk: Some(disk_gb),
            disk_count: Some(disk_count),
            bandwidth: None,
            invoice_type: None,
            deploy_ondemand: None,
            deploy_preemptible: None,
            monthly_cost: None,
            hourly_cost: Some(hourly_cost),
            monthly_cost_preemptible: None,
            hourly_cost_preemptible: None,
            plan_type: Some(plan_type.to_string()),
            locations: Vec::new(),
        }
    }

    fn sample_plans() -> Vec<BareMetalPlan> {
        let specs = [
            PlanSpec {
                id: "vbm-4c-32gb",
                cpu_count: 4,
                cpu_threads: 8,
                ram_gb: 32,
                disk_gb: 240,
                disk_count: 2,
                hourly_cost: 0.16,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-8c-132gb",
                cpu_count: 8,
                cpu_threads: 16,
                ram_gb: 128,
                disk_gb: 1920,
                disk_count: 2,
                hourly_cost: 0.48,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-6c-32gb",
                cpu_count: 6,
                cpu_threads: 12,
                ram_gb: 32,
                disk_gb: 960,
                disk_count: 2,
                hourly_cost: 0.25,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-24c-256gb-amd",
                cpu_count: 24,
                cpu_threads: 48,
                ram_gb: 256,
                disk_gb: 480,
                disk_count: 2,
                hourly_cost: 0.99,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-8c-132gb-v2",
                cpu_count: 8,
                cpu_threads: 16,
                ram_gb: 128,
                disk_gb: 1920,
                disk_count: 2,
                hourly_cost: 0.48,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-128c-2048gb-amd",
                cpu_count: 128,
                cpu_threads: 256,
                ram_gb: 2048,
                disk_gb: 480,
                disk_count: 2,
                hourly_cost: 7.53,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-24c-384gb-amd",
                cpu_count: 24,
                cpu_threads: 48,
                ram_gb: 384,
                disk_gb: 480,
                disk_count: 2,
                hourly_cost: 1.13,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-32c-755gb-amd",
                cpu_count: 32,
                cpu_threads: 64,
                ram_gb: 768,
                disk_gb: 480,
                disk_count: 2,
                hourly_cost: 1.99,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-64c-1536gb-amd",
                cpu_count: 64,
                cpu_threads: 128,
                ram_gb: 1536,
                disk_gb: 480,
                disk_count: 1,
                hourly_cost: 4.01,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-48c-1024gb-4-a100-gpu",
                cpu_count: 48,
                cpu_threads: 96,
                ram_gb: 1024,
                disk_gb: 450,
                disk_count: 2,
                hourly_cost: 9.59,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-112c-2048gb-8-h100-gpu",
                cpu_count: 112,
                cpu_threads: 224,
                ram_gb: 2048,
                disk_gb: 960,
                disk_count: 2,
                hourly_cost: 23.92,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-112c-2048gb-8-a100-gpu",
                cpu_count: 112,
                cpu_threads: 224,
                ram_gb: 2048,
                disk_gb: 960,
                disk_count: 4,
                hourly_cost: 22.40,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-64c-2048gb-8-l40-gpu",
                cpu_count: 64,
                cpu_threads: 128,
                ram_gb: 2048,
                disk_gb: 480,
                disk_count: 2,
                hourly_cost: 13.37,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-72c-480gb-gh200-gpu",
                cpu_count: 72,
                cpu_threads: 72,
                ram_gb: 480,
                disk_gb: 960,
                disk_count: 1,
                hourly_cost: 1.99,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-6c-128gb",
                cpu_count: 6,
                cpu_threads: 12,
                ram_gb: 128,
                disk_gb: 960,
                disk_count: 2,
                hourly_cost: 0.33,
                plan_type: "SSD",
            },
            PlanSpec {
                id: "vbm-256c-2048gb-8-mi300x-gpu",
                cpu_count: 128,
                cpu_threads: 256,
                ram_gb: 2048,
                disk_gb: 3576,
                disk_count: 8,
                hourly_cost: 31.92,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-256c-3072gb-8-mi325x-gpu",
                cpu_count: 128,
                cpu_threads: 256,
                ram_gb: 3072,
                disk_gb: 3576,
                disk_count: 8,
                hourly_cost: 36.92,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-256c-3072gb-8-b200-gpu",
                cpu_count: 128,
                cpu_threads: 256,
                ram_gb: 3072,
                disk_gb: 3576,
                disk_count: 8,
                hourly_cost: 68.00,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-6c-32gb-amd",
                cpu_count: 6,
                cpu_threads: 12,
                ram_gb: 32,
                disk_gb: 960,
                disk_count: 2,
                hourly_cost: 0.41,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-8c-64gb-amd",
                cpu_count: 8,
                cpu_threads: 16,
                ram_gb: 64,
                disk_gb: 1945,
                disk_count: 2,
                hourly_cost: 0.55,
                plan_type: "NVMe",
            },
            PlanSpec {
                id: "vbm-256c-3072gb-8-mi355x-gpu",
                cpu_count: 128,
                cpu_threads: 256,
                ram_gb: 3072,
                disk_gb: 3576,
                disk_count: 8,
                hourly_cost: 20.72,
                plan_type: "NVMe",
            },
        ];

        specs.into_iter().map(plan).collect()
    }

    #[test]
    fn test_sort_cpu_memory_default() {
        let mut plans = sample_plans();
        sort_bare_metal_plans(&mut plans, BareMetalPlanSort::CpuMemory, &PriceMode::Hourly);

        assert_eq!(plans[0].id, "vbm-256c-3072gb-8-b200-gpu");
        assert_eq!(plans[1].id, "vbm-256c-3072gb-8-mi325x-gpu");
        assert_eq!(plans[2].id, "vbm-256c-3072gb-8-mi355x-gpu");
        assert_eq!(plans[plans.len() - 1].id, "vbm-4c-32gb");
    }

    #[test]
    fn test_sort_cpu_descending() {
        let mut plans = sample_plans();
        sort_bare_metal_plans(&mut plans, BareMetalPlanSort::Cpu, &PriceMode::Hourly);

        assert_eq!(plans[0].id, "vbm-256c-3072gb-8-b200-gpu");
        assert_eq!(plans[plans.len() - 1].id, "vbm-4c-32gb");

        let ids: Vec<&str> = plans.iter().map(|plan| plan.id.as_str()).collect();
        let idx_8c_132 = ids.iter().position(|id| *id == "vbm-8c-132gb").unwrap();
        let idx_8c_132_v2 = ids.iter().position(|id| *id == "vbm-8c-132gb-v2").unwrap();
        let idx_8c_64 = ids.iter().position(|id| *id == "vbm-8c-64gb-amd").unwrap();
        assert!(idx_8c_132 < idx_8c_132_v2);
        assert!(idx_8c_132_v2 < idx_8c_64);
    }

    #[test]
    fn test_sort_memory_descending() {
        let mut plans = sample_plans();
        sort_bare_metal_plans(&mut plans, BareMetalPlanSort::Memory, &PriceMode::Hourly);

        assert_eq!(plans[0].id, "vbm-256c-3072gb-8-b200-gpu");
        let tail: Vec<&str> = plans[plans.len() - 3..]
            .iter()
            .map(|plan| plan.id.as_str())
            .collect();
        assert_eq!(tail, vec!["vbm-6c-32gb", "vbm-6c-32gb-amd", "vbm-4c-32gb"]);
    }

    #[test]
    fn test_sort_price_hourly_descending() {
        let mut plans = sample_plans();
        sort_bare_metal_plans(&mut plans, BareMetalPlanSort::Price, &PriceMode::Hourly);

        assert_eq!(plans[0].id, "vbm-256c-3072gb-8-b200-gpu");
        assert_eq!(plans[plans.len() - 1].id, "vbm-4c-32gb");
    }

    #[test]
    fn test_sort_disk_descending() {
        let mut plans = sample_plans();
        sort_bare_metal_plans(&mut plans, BareMetalPlanSort::Disk, &PriceMode::Hourly);

        assert_eq!(plans[0].id, "vbm-256c-3072gb-8-b200-gpu");
        assert_eq!(plans[plans.len() - 1].id, "vbm-4c-32gb");
    }
}
