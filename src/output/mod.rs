//! Output formatting for CLI responses

use crate::config::OutputFormat;
use crate::models::*;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

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
            power: i.power_status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
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
            script_type: s.script_type.as_ref().map(|t| t.to_string()).unwrap_or_default(),
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
            status: s.status.as_ref().map(|st| st.to_string()).unwrap_or_default(),
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
            attached_to: b.attached_to_instance.clone().unwrap_or_else(|| "-".to_string()),
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
            protocol: r.protocol.as_ref().map(|p| p.to_string()).unwrap_or_default(),
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
            monthly: p.monthly_cost.map(|c| format!("{:.2}", c)).unwrap_or_default(),
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
