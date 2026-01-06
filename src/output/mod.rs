//! Output formatting for CLI responses

use crate::config::OutputFormat;
use crate::models::*;
use colored::Colorize;
use tabled::{settings::Style, Table, Tabled};

/// Format and print output based on the selected format
pub fn print_output<T: serde::Serialize + TableDisplay>(data: &T, format: OutputFormat) {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::Table => data.print_table(),
    }
}

/// Print data as JSON
pub fn print_json<T: serde::Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}: {}", "Error formatting JSON".red(), e),
    }
}

/// Print a list as JSON
#[allow(dead_code)]
pub fn print_list_json<T: serde::Serialize>(data: &[T]) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}: {}", "Error formatting JSON".red(), e),
    }
}

/// Trait for types that can be displayed as a table
pub trait TableDisplay {
    fn print_table(&self);
}

/// Wrapper for displaying instances in a table
#[derive(Tabled)]
struct InstanceRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Plan")]
    plan: String,
    #[tabled(rename = "IP")]
    main_ip: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Power")]
    power: String,
}

impl From<&Instance> for InstanceRow {
    fn from(i: &Instance) -> Self {
        Self {
            id: i.id.clone(),
            label: i.label.clone().unwrap_or_default(),
            region: i.region.clone().unwrap_or_default(),
            plan: i.plan.clone().unwrap_or_default(),
            main_ip: i.main_ip.clone().unwrap_or_default(),
            status: i.status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
            power: i
                .power_status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Instance {
    fn print_table(&self) {
        let rows = vec![InstanceRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Instance> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No instances found.".yellow());
            return;
        }
        let rows: Vec<InstanceRow> = self.iter().map(InstanceRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying SSH keys in a table
#[derive(Tabled)]
struct SshKeyRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&SshKey> for SshKeyRow {
    fn from(k: &SshKey) -> Self {
        Self {
            id: k.id.clone(),
            name: k.name.clone().unwrap_or_default(),
            date_created: k.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for SshKey {
    fn print_table(&self) {
        let rows = vec![SshKeyRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(key) = &self.ssh_key {
            println!("\n{}:\n{}", "SSH Key".cyan(), key);
        }
    }
}

impl TableDisplay for Vec<SshKey> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No SSH keys found.".yellow());
            return;
        }
        let rows: Vec<SshKeyRow> = self.iter().map(SshKeyRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying startup scripts in a table
#[derive(Tabled)]
struct StartupScriptRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    script_type: String,
    #[tabled(rename = "Modified")]
    date_modified: String,
}

impl From<&StartupScript> for StartupScriptRow {
    fn from(s: &StartupScript) -> Self {
        Self {
            id: s.id.clone(),
            name: s.name.clone().unwrap_or_default(),
            script_type: s
                .script_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default(),
            date_modified: s.date_modified.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for StartupScript {
    fn print_table(&self) {
        let rows = vec![StartupScriptRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(decoded) = self.decode_script() {
            println!("\n{}:\n{}", "Script Content".cyan(), decoded);
        }
    }
}

impl TableDisplay for Vec<StartupScript> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No startup scripts found.".yellow());
            return;
        }
        let rows: Vec<StartupScriptRow> = self.iter().map(StartupScriptRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying snapshots in a table
#[derive(Tabled)]
struct SnapshotRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Snapshot> for SnapshotRow {
    fn from(s: &Snapshot) -> Self {
        Self {
            id: s.id.clone(),
            description: s.description.clone().unwrap_or_default(),
            size: s.size_human(),
            status: s
                .status
                .as_ref()
                .map(|st| st.to_string())
                .unwrap_or_default(),
            date_created: s.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Snapshot {
    fn print_table(&self) {
        let rows = vec![SnapshotRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Snapshot> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No snapshots found.".yellow());
            return;
        }
        let rows: Vec<SnapshotRow> = self.iter().map(SnapshotRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying block storage in a table
#[derive(Tabled)]
struct BlockStorageRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Size (GB)")]
    size_gb: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Attached To")]
    attached_to: String,
}

impl From<&BlockStorage> for BlockStorageRow {
    fn from(b: &BlockStorage) -> Self {
        Self {
            id: b.id.clone(),
            label: b.label.clone().unwrap_or_default(),
            size_gb: b.size_gb.map(|s| s.to_string()).unwrap_or_default(),
            region: b.region.clone().unwrap_or_default(),
            status: b.status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
            attached_to: b
                .attached_to_instance
                .clone()
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

impl TableDisplay for BlockStorage {
    fn print_table(&self) {
        let rows = vec![BlockStorageRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BlockStorage> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No block storage volumes found.".yellow());
            return;
        }
        let rows: Vec<BlockStorageRow> = self.iter().map(BlockStorageRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying firewall groups in a table
#[derive(Tabled)]
struct FirewallGroupRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Rules")]
    rule_count: String,
    #[tabled(rename = "Instances")]
    instance_count: String,
    #[tabled(rename = "Modified")]
    date_modified: String,
}

impl From<&FirewallGroup> for FirewallGroupRow {
    fn from(g: &FirewallGroup) -> Self {
        Self {
            id: g.id.clone(),
            description: g.description.clone().unwrap_or_default(),
            rule_count: g.rule_count.map(|c| c.to_string()).unwrap_or_default(),
            instance_count: g.instance_count.map(|c| c.to_string()).unwrap_or_default(),
            date_modified: g.date_modified.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for FirewallGroup {
    fn print_table(&self) {
        let rows = vec![FirewallGroupRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<FirewallGroup> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No firewall groups found.".yellow());
            return;
        }
        let rows: Vec<FirewallGroupRow> = self.iter().map(FirewallGroupRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying firewall rules in a table
#[derive(Tabled)]
struct FirewallRuleRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Protocol")]
    protocol: String,
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "Source")]
    source: String,
    #[tabled(rename = "Notes")]
    notes: String,
}

impl From<&FirewallRule> for FirewallRuleRow {
    fn from(r: &FirewallRule) -> Self {
        let source = match (&r.source, r.cidr()) {
            (Some(s), _) if !s.is_empty() => s.clone(),
            (_, Some(cidr)) => cidr,
            _ => "-".to_string(),
        };
        Self {
            id: r.id.to_string(),
            protocol: r
                .protocol
                .as_ref()
                .map(|p| p.to_string())
                .unwrap_or_default(),
            port: r.port.clone().unwrap_or_else(|| "-".to_string()),
            source,
            notes: r.notes.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for FirewallRule {
    fn print_table(&self) {
        let rows = vec![FirewallRuleRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<FirewallRule> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No firewall rules found.".yellow());
            return;
        }
        let rows: Vec<FirewallRuleRow> = self.iter().map(FirewallRuleRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying VPCs in a table
#[derive(Tabled)]
struct VpcRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Subnet")]
    subnet: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Vpc> for VpcRow {
    fn from(v: &Vpc) -> Self {
        Self {
            id: v.id.clone(),
            description: v.description.clone().unwrap_or_default(),
            region: v.region.clone().unwrap_or_default(),
            subnet: v.cidr().unwrap_or_default(),
            date_created: v.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vpc {
    fn print_table(&self) {
        let rows = vec![VpcRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Vpc> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPCs found.".yellow());
            return;
        }
        let rows: Vec<VpcRow> = self.iter().map(VpcRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying regions in a table
#[derive(Tabled)]
struct RegionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "City")]
    city: String,
    #[tabled(rename = "Country")]
    country: String,
    #[tabled(rename = "Features")]
    features: String,
}

impl From<&Region> for RegionRow {
    fn from(r: &Region) -> Self {
        Self {
            id: r.id.clone(),
            city: r.city.clone().unwrap_or_default(),
            country: r.country.clone().unwrap_or_default(),
            features: r.options.join(", "),
        }
    }
}

impl TableDisplay for Vec<Region> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No regions found.".yellow());
            return;
        }
        let rows: Vec<RegionRow> = self.iter().map(RegionRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying plans in a table
#[derive(Tabled)]
struct PlanRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "vCPUs")]
    vcpus: String,
    #[tabled(rename = "RAM (MB)")]
    ram: String,
    #[tabled(rename = "Disk (GB)")]
    disk: String,
    #[tabled(rename = "$/month")]
    monthly: String,
    #[tabled(rename = "Type")]
    plan_type: String,
}

impl From<&Plan> for PlanRow {
    fn from(p: &Plan) -> Self {
        Self {
            id: p.id.clone(),
            vcpus: p.vcpu_count.map(|v| v.to_string()).unwrap_or_default(),
            ram: p.ram.map(|r| r.to_string()).unwrap_or_default(),
            disk: p.disk.map(|d| d.to_string()).unwrap_or_default(),
            monthly: p
                .monthly_cost
                .map(|c| format!("{:.2}", c))
                .unwrap_or_default(),
            plan_type: p.plan_type.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Plan> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No plans found.".yellow());
            return;
        }
        let rows: Vec<PlanRow> = self.iter().map(PlanRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying OS in a table
#[derive(Tabled)]
struct OsRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Arch")]
    arch: String,
    #[tabled(rename = "Family")]
    family: String,
}

impl From<&Os> for OsRow {
    fn from(o: &Os) -> Self {
        Self {
            id: o.id.to_string(),
            name: o.name.clone().unwrap_or_default(),
            arch: o.arch.clone().unwrap_or_default(),
            family: o.family.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Os> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No operating systems found.".yellow());
            return;
        }
        let rows: Vec<OsRow> = self.iter().map(OsRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".cyan(), message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_row_from_instance() {
        let instance = Instance {
            id: "inst-123".to_string(),
            label: Some("web-server".to_string()),
            region: Some("ewr".to_string()),
            plan: Some("vc2-1c-1gb".to_string()),
            main_ip: Some("192.168.1.1".to_string()),
            status: Some(InstanceStatus::Active),
            power_status: Some(PowerStatus::Running),
            os: None,
            ram: None,
            disk: None,
            vcpu_count: None,
            default_password: None,
            date_created: None,
            allowed_bandwidth: None,
            netmask_v4: None,
            gateway_v4: None,
            v6_networks: vec![],
            hostname: None,
            internal_ip: None,
            kvm: None,
            os_id: None,
            app_id: None,
            image_id: None,
            firewall_group_id: None,
            features: vec![],
            tags: vec![],
            user_scheme: None,
            server_status: None,
        };
        let row = InstanceRow::from(&instance);
        assert_eq!(row.id, "inst-123");
        assert_eq!(row.label, "web-server");
        assert_eq!(row.region, "ewr");
    }

    #[test]
    fn test_instance_row_from_instance_defaults() {
        let instance = Instance {
            id: "inst-456".to_string(),
            label: None,
            region: None,
            plan: None,
            main_ip: None,
            status: None,
            power_status: None,
            os: None,
            ram: None,
            disk: None,
            vcpu_count: None,
            default_password: None,
            date_created: None,
            allowed_bandwidth: None,
            netmask_v4: None,
            gateway_v4: None,
            v6_networks: vec![],
            hostname: None,
            internal_ip: None,
            kvm: None,
            os_id: None,
            app_id: None,
            image_id: None,
            firewall_group_id: None,
            features: vec![],
            tags: vec![],
            user_scheme: None,
            server_status: None,
        };
        let row = InstanceRow::from(&instance);
        assert_eq!(row.id, "inst-456");
        assert_eq!(row.label, "");
        assert_eq!(row.status, "");
    }

    #[test]
    fn test_ssh_key_row_from() {
        let key = SshKey {
            id: "key-123".to_string(),
            name: Some("My Key".to_string()),
            date_created: Some("2024-01-01".to_string()),
            ssh_key: None,
        };
        let row = SshKeyRow::from(&key);
        assert_eq!(row.id, "key-123");
        assert_eq!(row.name, "My Key");
        assert_eq!(row.date_created, "2024-01-01");
    }

    #[test]
    fn test_snapshot_row_from() {
        let snapshot = Snapshot {
            id: "snap-123".to_string(),
            description: Some("Daily backup".to_string()),
            size: Some(1073741824),
            status: Some(SnapshotStatus::Complete),
            date_created: Some("2024-01-01".to_string()),
            os_id: None,
            app_id: None,
        };
        let row = SnapshotRow::from(&snapshot);
        assert_eq!(row.id, "snap-123");
        assert_eq!(row.description, "Daily backup");
        assert_eq!(row.status, "complete");
    }

    #[test]
    fn test_block_storage_row_from() {
        let block = BlockStorage {
            id: "block-123".to_string(),
            label: Some("Data Volume".to_string()),
            size_gb: Some(100),
            region: Some("ewr".to_string()),
            status: Some(BlockStorageStatus::Active),
            attached_to_instance: Some("inst-456".to_string()),
            cost: None,
            date_created: None,
            mount_id: None,
            block_type: None,
        };
        let row = BlockStorageRow::from(&block);
        assert_eq!(row.id, "block-123");
        assert_eq!(row.label, "Data Volume");
        assert_eq!(row.size_gb, "100");
        assert_eq!(row.attached_to, "inst-456");
    }

    #[test]
    fn test_block_storage_row_not_attached() {
        let block = BlockStorage {
            id: "block-789".to_string(),
            label: None,
            size_gb: None,
            region: None,
            status: None,
            attached_to_instance: None,
            cost: None,
            date_created: None,
            mount_id: None,
            block_type: None,
        };
        let row = BlockStorageRow::from(&block);
        assert_eq!(row.attached_to, "-");
    }

    #[test]
    fn test_firewall_group_row_from() {
        let group = FirewallGroup {
            id: "fw-123".to_string(),
            description: Some("Web Servers".to_string()),
            rule_count: Some(5),
            instance_count: Some(3),
            date_created: None,
            date_modified: Some("2024-01-01".to_string()),
            max_rule_count: None,
        };
        let row = FirewallGroupRow::from(&group);
        assert_eq!(row.id, "fw-123");
        assert_eq!(row.description, "Web Servers");
        assert_eq!(row.rule_count, "5");
    }

    #[test]
    fn test_firewall_rule_row_from() {
        let rule = FirewallRule {
            id: 1,
            ip_type: None,
            action: None,
            protocol: Some(Protocol::Tcp),
            port: Some("443".to_string()),
            subnet: Some("0.0.0.0".to_string()),
            subnet_size: Some(0),
            source: None,
            notes: Some("HTTPS".to_string()),
        };
        let row = FirewallRuleRow::from(&rule);
        assert_eq!(row.id, "1");
        assert_eq!(row.protocol, "TCP");
        assert_eq!(row.port, "443");
        assert_eq!(row.notes, "HTTPS");
    }

    #[test]
    fn test_firewall_rule_row_with_source() {
        let rule = FirewallRule {
            id: 2,
            ip_type: None,
            action: None,
            protocol: Some(Protocol::Tcp),
            port: None,
            subnet: None,
            subnet_size: None,
            source: Some("cloudflare".to_string()),
            notes: None,
        };
        let row = FirewallRuleRow::from(&rule);
        assert_eq!(row.source, "cloudflare");
    }

    #[test]
    fn test_vpc_row_from() {
        let vpc = Vpc {
            id: "vpc-123".to_string(),
            description: Some("Production".to_string()),
            region: Some("ewr".to_string()),
            v4_subnet: Some("10.0.0.0".to_string()),
            v4_subnet_mask: Some(16),
            date_created: Some("2024-01-01".to_string()),
        };
        let row = VpcRow::from(&vpc);
        assert_eq!(row.id, "vpc-123");
        assert_eq!(row.description, "Production");
        assert_eq!(row.subnet, "10.0.0.0/16");
    }

    #[test]
    fn test_region_row_from() {
        let region = Region {
            id: "ewr".to_string(),
            city: Some("Newark".to_string()),
            country: Some("US".to_string()),
            continent: Some("North America".to_string()),
            options: vec!["ddos_protection".to_string(), "block_storage".to_string()],
        };
        let row = RegionRow::from(&region);
        assert_eq!(row.id, "ewr");
        assert_eq!(row.city, "Newark");
        assert!(row.features.contains("ddos_protection"));
    }

    #[test]
    fn test_plan_row_from() {
        let plan = Plan {
            id: "vc2-1c-1gb".to_string(),
            name: None,
            vcpu_count: Some(1),
            ram: Some(1024),
            disk: Some(25),
            disk_count: Some(1),
            bandwidth: None,
            monthly_cost: Some(5.0),
            hourly_cost: None,
            plan_type: Some("vc2".to_string()),
            locations: vec![],
        };
        let row = PlanRow::from(&plan);
        assert_eq!(row.id, "vc2-1c-1gb");
        assert_eq!(row.vcpus, "1");
        assert_eq!(row.ram, "1024");
        assert_eq!(row.monthly, "5.00");
    }

    #[test]
    fn test_os_row_from() {
        let os = Os {
            id: 215,
            name: Some("Ubuntu 22.04 LTS".to_string()),
            arch: Some("x64".to_string()),
            family: Some("ubuntu".to_string()),
        };
        let row = OsRow::from(&os);
        assert_eq!(row.id, "215");
        assert_eq!(row.name, "Ubuntu 22.04 LTS");
        assert_eq!(row.arch, "x64");
    }

    #[test]
    fn test_startup_script_row_from() {
        let script = StartupScript {
            id: "script-123".to_string(),
            name: Some("Setup".to_string()),
            script_type: Some(ScriptType::Boot),
            script: None,
            date_created: None,
            date_modified: Some("2024-01-01".to_string()),
        };
        let row = StartupScriptRow::from(&script);
        assert_eq!(row.id, "script-123");
        assert_eq!(row.name, "Setup");
        assert_eq!(row.script_type, "boot");
    }
}
