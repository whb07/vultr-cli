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

// =====================
// Instance Advanced Types
// =====================

/// Wrapper for displaying IPv4 info in a table
#[derive(Tabled)]
struct Ipv4InfoRow {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Netmask")]
    netmask: String,
    #[tabled(rename = "Gateway")]
    gateway: String,
    #[tabled(rename = "Type")]
    ip_type: String,
    #[tabled(rename = "Reverse")]
    reverse: String,
}

impl From<&Ipv4Info> for Ipv4InfoRow {
    fn from(i: &Ipv4Info) -> Self {
        Self {
            ip: i.ip.clone(),
            netmask: i.netmask.clone().unwrap_or_default(),
            gateway: i.gateway.clone().unwrap_or_default(),
            ip_type: i.ip_type.clone().unwrap_or_default(),
            reverse: i.reverse.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Ipv4Info {
    fn print_table(&self) {
        let rows = vec![Ipv4InfoRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Ipv4Info> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No IPv4 addresses found.".yellow());
            return;
        }
        let rows: Vec<Ipv4InfoRow> = self.iter().map(Ipv4InfoRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying IPv6 info in a table
#[derive(Tabled)]
struct Ipv6InfoRow {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Network")]
    network: String,
    #[tabled(rename = "Size")]
    network_size: String,
    #[tabled(rename = "Type")]
    ip_type: String,
}

impl From<&Ipv6Info> for Ipv6InfoRow {
    fn from(i: &Ipv6Info) -> Self {
        Self {
            ip: i.ip.clone(),
            network: i.network.clone().unwrap_or_default(),
            network_size: i
                .network_size
                .map(|s| format!("/{}", s))
                .unwrap_or_default(),
            ip_type: i.ip_type.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Ipv6Info> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No IPv6 addresses found.".yellow());
            return;
        }
        let rows: Vec<Ipv6InfoRow> = self.iter().map(Ipv6InfoRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying reverse IPv6 in a table
#[derive(Tabled)]
struct ReverseIpv6Row {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Reverse DNS")]
    reverse: String,
}

impl From<&ReverseIpv6> for ReverseIpv6Row {
    fn from(r: &ReverseIpv6) -> Self {
        Self {
            ip: r.ip.clone(),
            reverse: r.reverse.clone(),
        }
    }
}

impl TableDisplay for Vec<ReverseIpv6> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No reverse DNS entries found.".yellow());
            return;
        }
        let rows: Vec<ReverseIpv6Row> = self.iter().map(ReverseIpv6Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for BackupSchedule {
    fn print_table(&self) {
        println!("{}: {}", "Enabled".cyan(), self.enabled);
        if let Some(ref t) = self.schedule_type {
            println!("{}: {}", "Type".cyan(), t);
        }
        if let Some(h) = self.hour {
            println!("{}: {}", "Hour".cyan(), h);
        }
        if let Some(d) = self.dow {
            println!("{}: {}", "Day of Week".cyan(), d);
        }
        if let Some(d) = self.dom {
            println!("{}: {}", "Day of Month".cyan(), d);
        }
        if let Some(ref next) = self.next_scheduled_time_utc {
            println!("{}: {}", "Next Scheduled".cyan(), next);
        }
    }
}

impl TableDisplay for IsoStatus {
    fn print_table(&self) {
        println!(
            "{}: {}",
            "ISO ID".cyan(),
            self.iso_id.as_deref().unwrap_or("None")
        );
        println!(
            "{}: {}",
            "State".cyan(),
            self.state.as_deref().unwrap_or("None")
        );
    }
}

impl TableDisplay for AvailableUpgrades {
    fn print_table(&self) {
        if !self.plans.is_empty() {
            println!("{}:", "Available Plans".cyan());
            for plan in &self.plans {
                println!("  - {}", plan);
            }
        }
        if !self.os.is_empty() {
            println!("{}:", "Available OS".cyan());
            for os in &self.os {
                println!("  - {} (ID: {})", os.name, os.id);
            }
        }
        if !self.applications.is_empty() {
            println!("{}:", "Available Applications".cyan());
            for app in &self.applications {
                println!("  - {} (ID: {})", app.name, app.id);
            }
        }
        if self.plans.is_empty() && self.os.is_empty() && self.applications.is_empty() {
            println!("{}", "No upgrades available.".yellow());
        }
    }
}

impl TableDisplay for UserData {
    fn print_table(&self) {
        println!("{}:", "User Data (base64)".cyan());
        println!("{}", self.data);
    }
}

impl TableDisplay for RestoreStatus {
    fn print_table(&self) {
        if let Some(ref t) = self.restore_type {
            println!("{}: {}", "Restore Type".cyan(), t);
        }
        if let Some(ref id) = self.restore_id {
            println!("{}: {}", "Restore ID".cyan(), id);
        }
        if let Some(ref s) = self.status {
            println!("{}: {}", "Status".cyan(), s);
        }
    }
}

/// Wrapper for displaying instance VPCs in a table
#[derive(Tabled)]
struct InstanceVpcRow {
    #[tabled(rename = "VPC ID")]
    id: String,
    #[tabled(rename = "MAC Address")]
    mac_address: String,
    #[tabled(rename = "IP Address")]
    ip_address: String,
}

impl From<&InstanceVpc> for InstanceVpcRow {
    fn from(v: &InstanceVpc) -> Self {
        Self {
            id: v.id.clone(),
            mac_address: v.mac_address.clone().unwrap_or_default(),
            ip_address: v.ip_address.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<InstanceVpc> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPCs attached.".yellow());
            return;
        }
        let rows: Vec<InstanceVpcRow> = self.iter().map(InstanceVpcRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying instance VPC2s in a table
#[derive(Tabled)]
struct InstanceVpc2Row {
    #[tabled(rename = "VPC2 ID")]
    id: String,
    #[tabled(rename = "MAC Address")]
    mac_address: String,
    #[tabled(rename = "IP Address")]
    ip_address: String,
}

impl From<&InstanceVpc2> for InstanceVpc2Row {
    fn from(v: &InstanceVpc2) -> Self {
        Self {
            id: v.id.clone(),
            mac_address: v.mac_address.clone().unwrap_or_default(),
            ip_address: v.ip_address.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<InstanceVpc2> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPC2s attached.".yellow());
            return;
        }
        let rows: Vec<InstanceVpc2Row> = self.iter().map(InstanceVpc2Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Bandwidth data table display
impl TableDisplay for std::collections::HashMap<String, BandwidthData> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No bandwidth data available.".yellow());
            return;
        }
        println!("{}", "Bandwidth Usage (by date):".cyan());
        for (date, data) in self {
            let incoming = data
                .incoming_bytes
                .map(format_bytes)
                .unwrap_or_else(|| "N/A".to_string());
            let outgoing = data
                .outgoing_bytes
                .map(format_bytes)
                .unwrap_or_else(|| "N/A".to_string());
            println!("  {}: In: {}, Out: {}", date, incoming, outgoing);
        }
    }
}

/// Format bytes into human-readable format
fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;
    const TB: i64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Neighbors display
impl TableDisplay for Vec<String> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No neighbors found (dedicated host).".green());
            return;
        }
        println!("{}:", "Neighbor Instance IDs".cyan());
        for neighbor in self {
            println!("  - {}", neighbor);
        }
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

// =====================
// Kubernetes Display Types
// =====================

/// Table row for Kubernetes clusters
#[derive(Tabled)]
pub struct KubernetesClusterRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "LABEL")]
    pub label: String,
    #[tabled(rename = "REGION")]
    pub region: String,
    #[tabled(rename = "VERSION")]
    pub version: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "HA")]
    pub ha: String,
    #[tabled(rename = "POOLS")]
    pub pools: String,
}

impl From<&KubernetesCluster> for KubernetesClusterRow {
    fn from(cluster: &KubernetesCluster) -> Self {
        Self {
            id: cluster.id.clone(),
            label: cluster.label.clone().unwrap_or_default(),
            region: cluster.region.clone().unwrap_or_default(),
            version: cluster.version.clone().unwrap_or_default(),
            status: cluster.status.clone().unwrap_or_default(),
            ha: if cluster.ha_controlplanes {
                "Yes"
            } else {
                "No"
            }
            .to_string(),
            pools: cluster.node_pools.len().to_string(),
        }
    }
}

impl TableDisplay for Vec<KubernetesCluster> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No Kubernetes clusters found.".yellow());
            return;
        }
        let rows: Vec<KubernetesClusterRow> = self.iter().map(KubernetesClusterRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for KubernetesCluster {
    fn print_table(&self) {
        println!("{}", "Kubernetes Cluster:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(label) = &self.label {
            println!("  {}: {}", "Label".green(), label);
        }
        if let Some(region) = &self.region {
            println!("  {}: {}", "Region".green(), region);
        }
        if let Some(version) = &self.version {
            println!("  {}: {}", "Version".green(), version);
        }
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        println!(
            "  {}: {}",
            "HA Control Planes".green(),
            if self.ha_controlplanes { "Yes" } else { "No" }
        );
        if let Some(ip) = &self.ip {
            println!("  {}: {}", "IP".green(), ip);
        }
        if let Some(endpoint) = &self.endpoint {
            println!("  {}: {}", "Endpoint".green(), endpoint);
        }
        if let Some(cluster_subnet) = &self.cluster_subnet {
            println!("  {}: {}", "Cluster Subnet".green(), cluster_subnet);
        }
        if let Some(service_subnet) = &self.service_subnet {
            println!("  {}: {}", "Service Subnet".green(), service_subnet);
        }
        if let Some(created) = &self.date_created {
            println!("  {}: {}", "Created".green(), created);
        }
        if !self.node_pools.is_empty() {
            println!("\n  {}:", "Node Pools".cyan());
            for pool in &self.node_pools {
                println!(
                    "    - {} ({}): {} nodes, plan: {}",
                    pool.id,
                    pool.label.as_deref().unwrap_or("no-label"),
                    pool.node_quantity.unwrap_or(0),
                    pool.plan.as_deref().unwrap_or("unknown")
                );
            }
        }
    }
}

/// Table row for node pools
#[derive(Tabled)]
pub struct NodePoolRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "LABEL")]
    pub label: String,
    #[tabled(rename = "PLAN")]
    pub plan: String,
    #[tabled(rename = "NODES")]
    pub nodes: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "AUTO-SCALER")]
    pub auto_scaler: String,
}

impl From<&NodePool> for NodePoolRow {
    fn from(pool: &NodePool) -> Self {
        let scaler = if pool.auto_scaler {
            format!(
                "{}-{}",
                pool.min_nodes.unwrap_or(0),
                pool.max_nodes.unwrap_or(0)
            )
        } else {
            "Off".to_string()
        };

        Self {
            id: pool.id.clone(),
            label: pool.label.clone().unwrap_or_default(),
            plan: pool.plan.clone().unwrap_or_default(),
            nodes: pool.node_quantity.unwrap_or(0).to_string(),
            status: pool.status.clone().unwrap_or_default(),
            auto_scaler: scaler,
        }
    }
}

impl TableDisplay for Vec<NodePool> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No node pools found.".yellow());
            return;
        }
        let rows: Vec<NodePoolRow> = self.iter().map(NodePoolRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for NodePool {
    fn print_table(&self) {
        println!("{}", "Node Pool:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(label) = &self.label {
            println!("  {}: {}", "Label".green(), label);
        }
        if let Some(plan) = &self.plan {
            println!("  {}: {}", "Plan".green(), plan);
        }
        if let Some(quantity) = &self.node_quantity {
            println!("  {}: {}", "Node Quantity".green(), quantity);
        }
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        println!(
            "  {}: {}",
            "Auto Scaler".green(),
            if self.auto_scaler {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        if self.auto_scaler {
            if let Some(min) = self.min_nodes {
                println!("  {}: {}", "Min Nodes".green(), min);
            }
            if let Some(max) = self.max_nodes {
                println!("  {}: {}", "Max Nodes".green(), max);
            }
        }
        if let Some(tag) = &self.tag {
            println!("  {}: {}", "Tag".green(), tag);
        }
        if let Some(created) = &self.date_created {
            println!("  {}: {}", "Created".green(), created);
        }
        if !self.nodes.is_empty() {
            println!("\n  {}:", "Nodes".cyan());
            for node in &self.nodes {
                println!(
                    "    - {} ({}): {}",
                    node.id,
                    node.label.as_deref().unwrap_or("no-label"),
                    node.status.as_deref().unwrap_or("unknown")
                );
            }
        }
    }
}

/// Table row for nodes
#[derive(Tabled)]
pub struct KubeNodeRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "LABEL")]
    pub label: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "CREATED")]
    pub created: String,
}

impl From<&KubeNode> for KubeNodeRow {
    fn from(node: &KubeNode) -> Self {
        Self {
            id: node.id.clone(),
            label: node.label.clone().unwrap_or_default(),
            status: node.status.clone().unwrap_or_default(),
            created: node.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<KubeNode> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No nodes found.".yellow());
            return;
        }
        let rows: Vec<KubeNodeRow> = self.iter().map(KubeNodeRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for KubeNode {
    fn print_table(&self) {
        println!("{}", "Node:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(label) = &self.label {
            println!("  {}: {}", "Label".green(), label);
        }
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        if let Some(created) = &self.date_created {
            println!("  {}: {}", "Created".green(), created);
        }
    }
}

impl TableDisplay for ClusterResources {
    fn print_table(&self) {
        println!("{}", "Cluster Resources:".cyan());

        if !self.resources.block_storage.is_empty() {
            println!("\n  {}:", "Block Storage".green());
            for bs in &self.resources.block_storage {
                println!(
                    "    - {} ({}): {}",
                    bs.id.as_deref().unwrap_or("unknown"),
                    bs.label.as_deref().unwrap_or("no-label"),
                    bs.status.as_deref().unwrap_or("unknown")
                );
            }
        } else {
            println!("\n  {}: None", "Block Storage".green());
        }

        if !self.resources.load_balancer.is_empty() {
            println!("\n  {}:", "Load Balancers".green());
            for lb in &self.resources.load_balancer {
                println!(
                    "    - {} ({}): {}",
                    lb.id.as_deref().unwrap_or("unknown"),
                    lb.label.as_deref().unwrap_or("no-label"),
                    lb.status.as_deref().unwrap_or("unknown")
                );
            }
        } else {
            println!("\n  {}: None", "Load Balancers".green());
        }
    }
}

// =====================
// Database Display Types
// =====================

/// Table row for databases
#[derive(Tabled)]
pub struct DatabaseRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "LABEL")]
    pub label: String,
    #[tabled(rename = "ENGINE")]
    pub engine: String,
    #[tabled(rename = "VERSION")]
    pub version: String,
    #[tabled(rename = "REGION")]
    pub region: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "HOST")]
    pub host: String,
}

impl From<&Database> for DatabaseRow {
    fn from(db: &Database) -> Self {
        Self {
            id: db.id.clone(),
            label: db.label.clone().unwrap_or_default(),
            engine: db.database_engine.clone().unwrap_or_default(),
            version: db.database_engine_version.clone().unwrap_or_default(),
            region: db.region.clone().unwrap_or_default(),
            status: db.status.clone().unwrap_or_default(),
            host: db.host.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Database> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No databases found.".yellow());
            return;
        }
        let rows: Vec<DatabaseRow> = self.iter().map(DatabaseRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Database {
    fn print_table(&self) {
        println!("{}", "Database:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(label) = &self.label {
            println!("  {}: {}", "Label".green(), label);
        }
        if let Some(engine) = &self.database_engine {
            println!("  {}: {}", "Engine".green(), engine);
        }
        if let Some(version) = &self.database_engine_version {
            println!("  {}: {}", "Version".green(), version);
        }
        if let Some(region) = &self.region {
            println!("  {}: {}", "Region".green(), region);
        }
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        if let Some(plan) = &self.plan {
            println!("  {}: {}", "Plan".green(), plan);
        }
        if let Some(host) = &self.host {
            println!("  {}: {}", "Host".green(), host);
        }
        if let Some(port) = &self.port {
            println!("  {}: {}", "Port".green(), port);
        }
        if let Some(user) = &self.user {
            println!("  {}: {}", "Default User".green(), user);
        }
        if let Some(dbname) = &self.dbname {
            println!("  {}: {}", "Default Database".green(), dbname);
        }
        if let Some(vcpus) = self.plan_vcpus {
            println!("  {}: {}", "vCPUs".green(), vcpus);
        }
        if let Some(ram) = self.plan_ram {
            println!("  {}: {} MB", "RAM".green(), ram);
        }
        if let Some(disk) = self.plan_disk {
            println!("  {}: {} GB", "Disk".green(), disk);
        }
        if let Some(replicas) = self.plan_replicas {
            println!("  {}: {}", "Replicas".green(), replicas);
        }
        if let Some(created) = &self.date_created {
            println!("  {}: {}", "Created".green(), created);
        }
        if !self.trusted_ips.is_empty() {
            println!(
                "  {}: {}",
                "Trusted IPs".green(),
                self.trusted_ips.join(", ")
            );
        }
    }
}

/// Table row for database plans
#[derive(Tabled)]
pub struct DatabasePlanRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "TYPE")]
    pub plan_type: String,
    #[tabled(rename = "NODES")]
    pub nodes: String,
    #[tabled(rename = "VCPUS")]
    pub vcpus: String,
    #[tabled(rename = "RAM")]
    pub ram: String,
    #[tabled(rename = "DISK")]
    pub disk: String,
    #[tabled(rename = "MONTHLY")]
    pub monthly: String,
}

impl From<&DatabasePlan> for DatabasePlanRow {
    fn from(plan: &DatabasePlan) -> Self {
        Self {
            id: plan.id.clone().unwrap_or_default(),
            plan_type: plan.plan_type.clone().unwrap_or_default(),
            nodes: plan
                .number_of_nodes
                .map(|n| n.to_string())
                .unwrap_or_default(),
            vcpus: plan.vcpu_count.map(|n| n.to_string()).unwrap_or_default(),
            ram: plan.ram.map(|n| n.to_string()).unwrap_or_default(),
            disk: plan.disk.map(|n| n.to_string()).unwrap_or_default(),
            monthly: plan
                .monthly_cost
                .map(|n| format!("${}", n))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<DatabasePlan> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No database plans found.".yellow());
            return;
        }
        let rows: Vec<DatabasePlanRow> = self.iter().map(DatabasePlanRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

/// Table row for database users
#[derive(Tabled)]
pub struct DatabaseUserRow {
    #[tabled(rename = "USERNAME")]
    pub username: String,
    #[tabled(rename = "ENCRYPTION")]
    pub encryption: String,
    #[tabled(rename = "PERMISSION")]
    pub permission: String,
}

impl From<&DatabaseUser> for DatabaseUserRow {
    fn from(user: &DatabaseUser) -> Self {
        Self {
            username: user.username.clone().unwrap_or_default(),
            encryption: user.encryption.clone().unwrap_or_else(|| "-".to_string()),
            permission: user.permission.clone().unwrap_or_else(|| "-".to_string()),
        }
    }
}

impl TableDisplay for Vec<DatabaseUser> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No database users found.".yellow());
            return;
        }
        let rows: Vec<DatabaseUserRow> = self.iter().map(DatabaseUserRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for DatabaseUser {
    fn print_table(&self) {
        println!("{}", "Database User:".cyan());
        if let Some(username) = &self.username {
            println!("  {}: {}", "Username".green(), username);
        }
        if let Some(password) = &self.password {
            println!("  {}: {}", "Password".green(), password);
        }
        if let Some(encryption) = &self.encryption {
            println!("  {}: {}", "Encryption".green(), encryption);
        }
        if let Some(permission) = &self.permission {
            println!("  {}: {}", "Permission".green(), permission);
        }
    }
}

/// Table row for logical databases
#[derive(Tabled)]
pub struct LogicalDatabaseRow {
    #[tabled(rename = "NAME")]
    pub name: String,
}

impl From<&LogicalDatabase> for LogicalDatabaseRow {
    fn from(db: &LogicalDatabase) -> Self {
        Self {
            name: db.name.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<LogicalDatabase> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No logical databases found.".yellow());
            return;
        }
        let rows: Vec<LogicalDatabaseRow> = self.iter().map(LogicalDatabaseRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for LogicalDatabase {
    fn print_table(&self) {
        println!("{}", "Logical Database:".cyan());
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
    }
}

/// Table row for connection pools
#[derive(Tabled)]
pub struct ConnectionPoolRow {
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "DATABASE")]
    pub database: String,
    #[tabled(rename = "USERNAME")]
    pub username: String,
    #[tabled(rename = "MODE")]
    pub mode: String,
    #[tabled(rename = "SIZE")]
    pub size: String,
}

impl From<&ConnectionPool> for ConnectionPoolRow {
    fn from(pool: &ConnectionPool) -> Self {
        Self {
            name: pool.name.clone().unwrap_or_default(),
            database: pool.database.clone().unwrap_or_default(),
            username: pool.username.clone().unwrap_or_default(),
            mode: pool.mode.clone().unwrap_or_default(),
            size: pool.size.map(|n| n.to_string()).unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<ConnectionPool> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No connection pools found.".yellow());
            return;
        }
        let rows: Vec<ConnectionPoolRow> = self.iter().map(ConnectionPoolRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for ConnectionPool {
    fn print_table(&self) {
        println!("{}", "Connection Pool:".cyan());
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
        if let Some(database) = &self.database {
            println!("  {}: {}", "Database".green(), database);
        }
        if let Some(username) = &self.username {
            println!("  {}: {}", "Username".green(), username);
        }
        if let Some(mode) = &self.mode {
            println!("  {}: {}", "Mode".green(), mode);
        }
        if let Some(size) = &self.size {
            println!("  {}: {}", "Size".green(), size);
        }
    }
}

/// Table row for Kafka topics
#[derive(Tabled)]
pub struct KafkaTopicRow {
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "PARTITIONS")]
    pub partitions: String,
    #[tabled(rename = "REPLICATION")]
    pub replication: String,
    #[tabled(rename = "RETENTION HRS")]
    pub retention_hours: String,
}

impl From<&KafkaTopic> for KafkaTopicRow {
    fn from(topic: &KafkaTopic) -> Self {
        Self {
            name: topic.name.clone().unwrap_or_default(),
            partitions: topic.partitions.map(|n| n.to_string()).unwrap_or_default(),
            replication: topic.replication.map(|n| n.to_string()).unwrap_or_default(),
            retention_hours: topic
                .retention_hours
                .map(|n| n.to_string())
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<KafkaTopic> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No Kafka topics found.".yellow());
            return;
        }
        let rows: Vec<KafkaTopicRow> = self.iter().map(KafkaTopicRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for KafkaTopic {
    fn print_table(&self) {
        println!("{}", "Kafka Topic:".cyan());
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
        if let Some(partitions) = &self.partitions {
            println!("  {}: {}", "Partitions".green(), partitions);
        }
        if let Some(replication) = &self.replication {
            println!("  {}: {}", "Replication".green(), replication);
        }
        if let Some(hours) = &self.retention_hours {
            println!("  {}: {}", "Retention Hours".green(), hours);
        }
        if let Some(bytes) = &self.retention_bytes {
            println!("  {}: {}", "Retention Bytes".green(), bytes);
        }
    }
}

/// Table row for Kafka connectors
#[derive(Tabled)]
pub struct KafkaConnectorRow {
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "CLASS")]
    pub class: String,
    #[tabled(rename = "TOPICS")]
    pub topics: String,
}

impl From<&KafkaConnector> for KafkaConnectorRow {
    fn from(connector: &KafkaConnector) -> Self {
        Self {
            name: connector.name.clone().unwrap_or_default(),
            class: connector.class.clone().unwrap_or_default(),
            topics: connector.topics.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<KafkaConnector> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No Kafka connectors found.".yellow());
            return;
        }
        let rows: Vec<KafkaConnectorRow> = self.iter().map(KafkaConnectorRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for KafkaConnector {
    fn print_table(&self) {
        println!("{}", "Kafka Connector:".cyan());
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
        if let Some(class) = &self.class {
            println!("  {}: {}", "Class".green(), class);
        }
        if let Some(topics) = &self.topics {
            println!("  {}: {}", "Topics".green(), topics);
        }
    }
}

/// Table row for available connectors
#[derive(Tabled)]
pub struct AvailableConnectorRow {
    #[tabled(rename = "CLASS")]
    pub class: String,
    #[tabled(rename = "TYPE")]
    pub connector_type: String,
    #[tabled(rename = "VERSION")]
    pub version: String,
}

impl From<&AvailableConnector> for AvailableConnectorRow {
    fn from(connector: &AvailableConnector) -> Self {
        Self {
            class: connector.class.clone().unwrap_or_default(),
            connector_type: connector.connector_type.clone().unwrap_or_default(),
            version: connector.version.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<AvailableConnector> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No available connectors found.".yellow());
            return;
        }
        let rows: Vec<AvailableConnectorRow> =
            self.iter().map(AvailableConnectorRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for ConnectorStatus {
    fn print_table(&self) {
        println!("{}", "Connector Status:".cyan());
        if let Some(connector) = &self.connector {
            if let Some(state) = &connector.state {
                println!("  {}: {}", "State".green(), state);
            }
        }
        if !self.tasks.is_empty() {
            println!("\n  {}:", "Tasks".cyan());
            for task in &self.tasks {
                let id = task.id.map(|i| i.to_string()).unwrap_or_default();
                let state = task.state.as_deref().unwrap_or("unknown");
                println!("    - Task {}: {}", id, state);
                if let Some(trace) = &task.trace {
                    println!("      {}: {}", "Trace".yellow(), trace);
                }
            }
        }
    }
}

/// Table row for database alerts
#[derive(Tabled)]
pub struct DatabaseAlertRow {
    #[tabled(rename = "TIMESTAMP")]
    pub timestamp: String,
    #[tabled(rename = "TYPE")]
    pub message_type: String,
    #[tabled(rename = "DESCRIPTION")]
    pub description: String,
}

impl From<&DatabaseAlert> for DatabaseAlertRow {
    fn from(alert: &DatabaseAlert) -> Self {
        Self {
            timestamp: alert.timestamp.clone().unwrap_or_default(),
            message_type: alert.message_type.clone().unwrap_or_default(),
            description: alert.description.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<DatabaseAlert> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No alerts found.".green());
            return;
        }
        let rows: Vec<DatabaseAlertRow> = self.iter().map(DatabaseAlertRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for DatabaseUsage {
    fn print_table(&self) {
        println!("{}", "Database Usage:".cyan());
        if let Some(disk) = &self.disk {
            println!("\n  {}:", "Disk".green());
            if let Some(current) = &disk.current_gb {
                println!("    Current: {} GB", current);
            }
            if let Some(max) = &disk.max_gb {
                println!("    Max: {} GB", max);
            }
            if let Some(pct) = &disk.percentage {
                println!("    Usage: {}%", pct);
            }
        }
        if let Some(memory) = &self.memory {
            println!("\n  {}:", "Memory".green());
            if let Some(current) = &memory.current_mb {
                println!("    Current: {} MB", current);
            }
            if let Some(max) = &memory.max_mb {
                println!("    Max: {} MB", max);
            }
            if let Some(pct) = &memory.percentage {
                println!("    Usage: {}%", pct);
            }
        }
        if let Some(cpu) = &self.cpu {
            println!("\n  {}:", "CPU".green());
            if let Some(pct) = &cpu.percentage {
                println!("    Usage: {}%", pct);
            }
        }
    }
}

impl TableDisplay for DatabaseBackupsResponse {
    fn print_table(&self) {
        println!("{}", "Database Backups:".cyan());
        if let Some(latest) = &self.latest_backup {
            println!("\n  {}:", "Latest Backup".green());
            if let Some(date) = &latest.date {
                println!("    Date: {}", date);
            }
            if let Some(time) = &latest.time {
                println!("    Time: {}", time);
            }
        }
        if let Some(oldest) = &self.oldest_backup {
            println!("\n  {}:", "Oldest Backup".green());
            if let Some(date) = &oldest.date {
                println!("    Date: {}", date);
            }
            if let Some(time) = &oldest.time {
                println!("    Time: {}", time);
            }
        }
        if self.latest_backup.is_none() && self.oldest_backup.is_none() {
            println!("{}", "  No backups available.".yellow());
        }
    }
}

impl TableDisplay for MaintenanceSchedule {
    fn print_table(&self) {
        println!("{}", "Maintenance Schedule:".cyan());
        if let Some(day) = &self.day {
            println!("  {}: {}", "Day".green(), day);
        }
        if let Some(hour) = &self.hour {
            println!("  {}: {}:00", "Hour".green(), hour);
        }
    }
}

impl TableDisplay for DatabaseMigration {
    fn print_table(&self) {
        println!("{}", "Database Migration:".cyan());
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        if let Some(method) = &self.method {
            println!("  {}: {}", "Method".green(), method);
        }
        if let Some(error) = &self.error {
            println!("  {}: {}", "Error".red(), error);
        }
        if let Some(creds) = &self.credentials {
            println!("\n  {}:", "Source".cyan());
            if let Some(host) = &creds.host {
                println!("    Host: {}", host);
            }
            if let Some(port) = &creds.port {
                println!("    Port: {}", port);
            }
            if let Some(username) = &creds.username {
                println!("    Username: {}", username);
            }
            if let Some(database) = &creds.database {
                println!("    Database: {}", database);
            }
            println!("    SSL: {}", if creds.ssl { "Yes" } else { "No" });
        }
    }
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
