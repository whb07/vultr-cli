//! Output formatting for CLI responses

use vultr_config::OutputFormat;
use vultr_models::*;
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
pub fn print_json<T: serde::Serialize + ?Sized>(data: &T) {
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
                .map(|t| format_script_type(&t.to_string()))
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

/// Wrapper for displaying backups in a table
#[derive(Tabled)]
struct BackupRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Ready")]
    ready: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Backup> for BackupRow {
    fn from(b: &Backup) -> Self {
        Self {
            id: b.id.clone(),
            description: b.description.clone().unwrap_or_default(),
            size: b.size_human(),
            status: b
                .status
                .as_ref()
                .map(|st| st.to_string())
                .unwrap_or_default(),
            ready: if b.is_ready() { "Yes" } else { "No" }.to_string(),
            date_created: b.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Backup {
    fn print_table(&self) {
        let rows = vec![BackupRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Backup> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No backups found.".yellow());
            return;
        }
        let rows: Vec<BackupRow> = self.iter().map(BackupRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying bare metal servers in a table
#[derive(Tabled)]
struct BareMetalRow {
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
}

impl From<&BareMetal> for BareMetalRow {
    fn from(bm: &BareMetal) -> Self {
        Self {
            id: bm.id.clone(),
            label: bm.label.clone().unwrap_or_default(),
            region: bm.region.clone().unwrap_or_default(),
            plan: bm.plan.clone().unwrap_or_default(),
            main_ip: bm.main_ip.clone().unwrap_or_default(),
            status: bm.status.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for BareMetal {
    fn print_table(&self) {
        let rows = vec![BareMetalRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BareMetal> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No bare metal servers found.".yellow());
            return;
        }
        let rows: Vec<BareMetalRow> = self.iter().map(BareMetalRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying bare metal IPv4 in a table
#[derive(Tabled)]
struct BareMetalIpv4Row {
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

impl From<&BareMetalIpv4> for BareMetalIpv4Row {
    fn from(ip: &BareMetalIpv4) -> Self {
        Self {
            ip: ip.ip.clone(),
            netmask: ip.netmask.clone().unwrap_or_default(),
            gateway: ip.gateway.clone().unwrap_or_default(),
            ip_type: ip
                .ip_type
                .as_deref()
                .map(format_ip_type)
                .unwrap_or_default(),
            reverse: ip.reverse.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for BareMetalIpv4 {
    fn print_table(&self) {
        let rows = vec![BareMetalIpv4Row::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BareMetalIpv4> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No IPv4 addresses found.".yellow());
            return;
        }
        let rows: Vec<BareMetalIpv4Row> = self.iter().map(BareMetalIpv4Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying bare metal IPv6 in a table
#[derive(Tabled)]
struct BareMetalIpv6Row {
    #[tabled(rename = "IP")]
    ip: String,
    #[tabled(rename = "Network")]
    network: String,
    #[tabled(rename = "Network Size")]
    network_size: String,
    #[tabled(rename = "Type")]
    ip_type: String,
}

impl From<&BareMetalIpv6> for BareMetalIpv6Row {
    fn from(ip: &BareMetalIpv6) -> Self {
        Self {
            ip: ip.ip.clone(),
            network: ip.network.clone().unwrap_or_default(),
            network_size: ip.network_size.map(|n| n.to_string()).unwrap_or_default(),
            ip_type: ip
                .ip_type
                .as_deref()
                .map(format_ip_type)
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for BareMetalIpv6 {
    fn print_table(&self) {
        let rows = vec![BareMetalIpv6Row::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BareMetalIpv6> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No IPv6 addresses found.".yellow());
            return;
        }
        let rows: Vec<BareMetalIpv6Row> = self.iter().map(BareMetalIpv6Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying bare metal VPC in a table
#[derive(Tabled)]
struct BareMetalVpcRow {
    #[tabled(rename = "VPC ID")]
    id: String,
    #[tabled(rename = "MAC Address")]
    mac_address: String,
    #[tabled(rename = "IP Address")]
    ip_address: String,
}

impl From<&BareMetalVpc> for BareMetalVpcRow {
    fn from(vpc: &BareMetalVpc) -> Self {
        Self {
            id: vpc.id.clone(),
            mac_address: vpc.mac_address.clone().unwrap_or_default(),
            ip_address: vpc.ip_address.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for BareMetalVpc {
    fn print_table(&self) {
        let rows = vec![BareMetalVpcRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BareMetalVpc> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPCs attached.".yellow());
            return;
        }
        let rows: Vec<BareMetalVpcRow> = self.iter().map(BareMetalVpcRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying bare metal VPC2 in a table
#[derive(Tabled)]
struct BareMetalVpc2Row {
    #[tabled(rename = "VPC2 ID")]
    id: String,
    #[tabled(rename = "MAC Address")]
    mac_address: String,
    #[tabled(rename = "IP Address")]
    ip_address: String,
}

impl From<&BareMetalVpc2> for BareMetalVpc2Row {
    fn from(vpc: &BareMetalVpc2) -> Self {
        Self {
            id: vpc.id.clone(),
            mac_address: vpc.mac_address.clone().unwrap_or_default(),
            ip_address: vpc.ip_address.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for BareMetalVpc2 {
    fn print_table(&self) {
        let rows = vec![BareMetalVpc2Row::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BareMetalVpc2> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPC2s attached.".yellow());
            return;
        }
        let rows: Vec<BareMetalVpc2Row> = self.iter().map(BareMetalVpc2Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for BareMetalUpgrades {
    fn print_table(&self) {
        println!("{}", "Available OS Upgrades:".cyan());
        if self.os.is_empty() {
            println!("  {}", "None available".yellow());
        } else {
            for os in &self.os {
                println!(
                    "  - {} (ID: {}, Arch: {}, Family: {})",
                    os.name,
                    os.id,
                    os.arch.as_deref().unwrap_or("-"),
                    os.family.as_deref().unwrap_or("-")
                );
            }
        }
        println!("\n{}", "Available Applications:".cyan());
        if self.applications.is_empty() {
            println!("  {}", "None available".yellow());
        } else {
            for app in &self.applications {
                println!("  - {} (ID: {})", app.name, app.id);
            }
        }
    }
}

impl TableDisplay for BareMetalVnc {
    fn print_table(&self) {
        println!("{}: {}", "VNC URL".cyan(), self.url);
    }
}

impl TableDisplay for BareMetalUserData {
    fn print_table(&self) {
        println!("{}", "User Data (Base64):".cyan());
        println!("{}", self.data);
    }
}

/// Wrapper for displaying ISOs in a table
#[derive(Tabled)]
struct IsoRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Filename")]
    filename: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Ready")]
    ready: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Iso> for IsoRow {
    fn from(i: &Iso) -> Self {
        Self {
            id: i.id.clone(),
            filename: i.filename.clone().unwrap_or_default(),
            size: i.size_human(),
            status: i
                .status
                .as_ref()
                .map(|st| st.to_string())
                .unwrap_or_default(),
            ready: if i.is_ready() { "Yes" } else { "No" }.to_string(),
            date_created: i.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Iso {
    fn print_table(&self) {
        let rows = vec![IsoRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(md5) = &self.md5sum {
            println!("\n{}: {}", "MD5".cyan(), md5);
        }
        if let Some(sha512) = &self.sha512sum {
            println!("{}: {}", "SHA512".cyan(), sha512);
        }
    }
}

impl TableDisplay for Vec<Iso> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No ISOs found.".yellow());
            return;
        }
        let rows: Vec<IsoRow> = self.iter().map(IsoRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying public ISOs in a table
#[derive(Tabled)]
struct PublicIsoRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
}

impl From<&PublicIso> for PublicIsoRow {
    fn from(i: &PublicIso) -> Self {
        Self {
            id: i.id.clone(),
            name: i.name.clone().unwrap_or_default(),
            description: i.description.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<PublicIso> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No public ISOs found.".yellow());
            return;
        }
        let rows: Vec<PublicIsoRow> = self.iter().map(PublicIsoRow::from).collect();
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
            attached_to: if b.is_attached() {
                b.attached_to_instance
                    .clone()
                    .unwrap_or_else(|| "-".to_string())
            } else {
                "-".to_string()
            },
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

/// Wrapper for displaying object storage in a table
#[derive(Tabled)]
struct ObjectStorageRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Hostname")]
    hostname: String,
}

impl From<&ObjectStorage> for ObjectStorageRow {
    fn from(o: &ObjectStorage) -> Self {
        Self {
            id: o.id.clone(),
            label: o.label.clone().unwrap_or_default(),
            region: o.region.clone().unwrap_or_default(),
            status: o.status.as_ref().map(|s| s.to_string()).unwrap_or_default(),
            hostname: o.s3_hostname.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for ObjectStorage {
    fn print_table(&self) {
        let rows = vec![ObjectStorageRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(access_key) = &self.s3_access_key {
            println!("\n{}: {}", "Access Key".cyan(), access_key);
        }
        if let Some(secret_key) = &self.s3_secret_key {
            println!("{}: {}", "Secret Key".cyan(), secret_key);
        }
    }
}

impl TableDisplay for Vec<ObjectStorage> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No object storages found.".yellow());
            return;
        }
        let rows: Vec<ObjectStorageRow> = self.iter().map(ObjectStorageRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying object storage clusters in a table
#[derive(Tabled)]
struct ObjectStorageClusterRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Hostname")]
    hostname: String,
    #[tabled(rename = "Deploy")]
    deploy: String,
}

impl From<&ObjectStorageCluster> for ObjectStorageClusterRow {
    fn from(c: &ObjectStorageCluster) -> Self {
        Self {
            id: c.id.to_string(),
            region: c.region.clone().unwrap_or_default(),
            hostname: c.hostname.clone().unwrap_or_default(),
            deploy: c.deploy.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for ObjectStorageCluster {
    fn print_table(&self) {
        let rows = vec![ObjectStorageClusterRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<ObjectStorageCluster> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No object storage clusters found.".yellow());
            return;
        }
        let rows: Vec<ObjectStorageClusterRow> =
            self.iter().map(ObjectStorageClusterRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying object storage tiers in a table
#[derive(Tabled)]
struct ObjectStorageTierRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "Default")]
    is_default: String,
    #[tabled(rename = "Slug")]
    slug: String,
}

impl From<&ObjectStorageTier> for ObjectStorageTierRow {
    fn from(t: &ObjectStorageTier) -> Self {
        Self {
            id: t.id.to_string(),
            name: t.sales_name.clone().unwrap_or_default(),
            price: t.price.map(|p| format!("${:.2}", p)).unwrap_or_default(),
            is_default: t.is_default.clone().unwrap_or_default(),
            slug: t.slug.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for ObjectStorageTier {
    fn print_table(&self) {
        let rows = vec![ObjectStorageTierRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(desc) = &self.sales_desc {
            println!("\n{}: {}", "Description".cyan(), desc);
        }
    }
}

impl TableDisplay for Vec<ObjectStorageTier> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No object storage tiers found.".yellow());
            return;
        }
        let rows: Vec<ObjectStorageTierRow> = self.iter().map(ObjectStorageTierRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying cluster-specific tiers in a table
#[derive(Tabled)]
struct ClusterTierRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "Default")]
    is_default: String,
    #[tabled(rename = "Slug")]
    slug: String,
}

impl From<&ClusterTier> for ClusterTierRow {
    fn from(t: &ClusterTier) -> Self {
        Self {
            id: t.id.to_string(),
            name: t.sales_name.clone().unwrap_or_default(),
            price: t.price.map(|p| format!("${:.2}", p)).unwrap_or_default(),
            is_default: t.is_default.clone().unwrap_or_default(),
            slug: t.slug.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for ClusterTier {
    fn print_table(&self) {
        let rows = vec![ClusterTierRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(desc) = &self.sales_desc {
            println!("\n{}: {}", "Description".cyan(), desc);
        }
    }
}

impl TableDisplay for Vec<ClusterTier> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No cluster tiers found.".yellow());
            return;
        }
        let rows: Vec<ClusterTierRow> = self.iter().map(ClusterTierRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying S3 credentials
#[derive(Tabled)]
struct S3CredentialsRow {
    #[tabled(rename = "Hostname")]
    hostname: String,
    #[tabled(rename = "Access Key")]
    access_key: String,
    #[tabled(rename = "Secret Key")]
    secret_key: String,
}

impl From<&S3Credentials> for S3CredentialsRow {
    fn from(c: &S3Credentials) -> Self {
        Self {
            hostname: c.s3_hostname.clone().unwrap_or_default(),
            access_key: c.s3_access_key.clone().unwrap_or_default(),
            secret_key: c.s3_secret_key.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for S3Credentials {
    fn print_table(&self) {
        let rows = vec![S3CredentialsRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying reserved IPs in a table
#[derive(Tabled)]
struct ReservedIpRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Type")]
    ip_type: String,
    #[tabled(rename = "Subnet")]
    subnet: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Instance")]
    instance_id: String,
}

impl From<&ReservedIp> for ReservedIpRow {
    fn from(r: &ReservedIp) -> Self {
        Self {
            id: r.id.clone(),
            region: r.region.clone().unwrap_or_default(),
            ip_type: r.ip_type.as_deref().map(format_ip_type).unwrap_or_default(),
            subnet: r.cidr().unwrap_or_default(),
            label: r.label.clone().unwrap_or_default(),
            instance_id: if r.is_attached() {
                r.instance_id.clone().unwrap_or_else(|| "-".to_string())
            } else {
                "-".to_string()
            },
        }
    }
}

impl TableDisplay for ReservedIp {
    fn print_table(&self) {
        let rows = vec![ReservedIpRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<ReservedIp> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No reserved IPs found.".yellow());
            return;
        }
        let rows: Vec<ReservedIpRow> = self.iter().map(ReservedIpRow::from).collect();
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

/// Wrapper for displaying VPC 2.0 networks in a table
#[derive(Tabled)]
struct Vpc2Row {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "IP Block")]
    ip_block: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Vpc2> for Vpc2Row {
    fn from(v: &Vpc2) -> Self {
        Self {
            id: v.id.clone(),
            description: v.description.clone().unwrap_or_default(),
            region: v.region.clone().unwrap_or_default(),
            ip_block: v.cidr().unwrap_or_default(),
            date_created: v.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vpc2 {
    fn print_table(&self) {
        let rows = vec![Vpc2Row::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Vpc2> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPC 2.0 networks found.".yellow());
            return;
        }
        let rows: Vec<Vpc2Row> = self.iter().map(Vpc2Row::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying VPC 2.0 nodes in a table
#[derive(Tabled)]
struct Vpc2NodeRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "IP Address")]
    ip_address: String,
    #[tabled(rename = "MAC Address")]
    mac_address: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Type")]
    node_type: String,
}

impl From<&Vpc2Node> for Vpc2NodeRow {
    fn from(n: &Vpc2Node) -> Self {
        Self {
            id: n.id.clone().unwrap_or_default(),
            ip_address: n.ip_address.clone().unwrap_or_default(),
            mac_address: n.mac_address.map(|m| m.to_string()).unwrap_or_default(),
            description: n.description.clone().unwrap_or_default(),
            node_type: n
                .node_type
                .as_deref()
                .map(format_vpc2_node_type)
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vpc2Node {
    fn print_table(&self) {
        let rows = vec![Vpc2NodeRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Vpc2Node> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPC 2.0 nodes found.".yellow());
            return;
        }
        let rows: Vec<Vpc2NodeRow> = self.iter().map(Vpc2NodeRow::from).collect();
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
    #[tabled(rename = "RAM (GB)")]
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
            ram: format_ram_gb(p.ram),
            disk: p.disk.map(|d| d.to_string()).unwrap_or_default(),
            monthly: p
                .monthly_cost
                .map(|c| format!("{:.2}", c))
                .unwrap_or_default(),
            plan_type: p
                .plan_type
                .as_deref()
                .map(format_plan_type)
                .unwrap_or_default(),
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

fn format_ram_gb(mb: Option<i32>) -> String {
    let mb = match mb {
        Some(value) => value as f64,
        None => return String::new(),
    };
    let gb = mb / 1024.0;
    if (gb.fract() - 0.0).abs() < f64::EPSILON {
        format!("{:.0}", gb)
    } else {
        format!("{:.2}", gb)
    }
}

fn format_plan_type(plan_type: &str) -> String {
    match plan_type.trim().to_lowercase().as_str() {
        "vc2" => "Cloud Compute".to_string(),
        "vhf" => "High Frequency Compute".to_string(),
        "vdc" => "Dedicated Cloud".to_string(),
        "vhp" => "High Performance".to_string(),
        "voc" => "Optimized Cloud".to_string(),
        "voc-g" => "Optimized Cloud (General Purpose)".to_string(),
        "voc-c" => "Optimized Cloud (CPU)".to_string(),
        "voc-m" => "Optimized Cloud (Memory)".to_string(),
        "voc-s" => "Optimized Cloud (Storage)".to_string(),
        "vcg" => "Cloud GPU".to_string(),
        "vbm" => "Bare Metal".to_string(),
        other => other.to_string(),
    }
}

fn format_ip_type(ip_type: &str) -> String {
    match ip_type.trim().to_lowercase().as_str() {
        "v4" | "ipv4" => "IPv4".to_string(),
        "v6" | "ipv6" => "IPv6".to_string(),
        other => other.to_string(),
    }
}

fn format_script_type(script_type: &str) -> String {
    match script_type.trim().to_lowercase().as_str() {
        "boot" => "Boot".to_string(),
        "pxe" => "PXE".to_string(),
        other => other.to_string(),
    }
}

fn format_vpc2_node_type(node_type: &str) -> String {
    match node_type.trim().to_lowercase().as_str() {
        "instance" => "Instance".to_string(),
        "baremetal" | "bare-metal" | "bare_metal" => "Bare Metal".to_string(),
        other => other.to_string(),
    }
}

/// Wrapper for displaying bare metal plans in a table (hourly pricing)
#[derive(Tabled)]
struct BareMetalPlanHourlyRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "CPUs")]
    cpu_count: String,
    #[tabled(rename = "Threads")]
    cpu_threads: String,
    #[tabled(rename = "CPU Model")]
    cpu_model: String,
    #[tabled(rename = "RAM (GB)")]
    ram: String,
    #[tabled(rename = "Disk (GB)")]
    disk: String,
    #[tabled(rename = "Disks")]
    disk_count: String,
    #[tabled(rename = "$/hour")]
    price: String,
    #[tabled(rename = "Type")]
    plan_type: String,
}

impl From<&BareMetalPlan> for BareMetalPlanHourlyRow {
    fn from(p: &BareMetalPlan) -> Self {
        Self {
            id: p.id.clone(),
            cpu_count: p.cpu_count.map(|v| v.to_string()).unwrap_or_default(),
            cpu_threads: p.cpu_threads.map(|v| v.to_string()).unwrap_or_default(),
            cpu_model: p.cpu_model.clone().unwrap_or_default(),
            ram: format_ram_gb(p.ram),
            disk: p.disk.map(|d| d.to_string()).unwrap_or_default(),
            disk_count: p.disk_count.map(|d| d.to_string()).unwrap_or_default(),
            price: p
                .hourly_cost
                .map(|c| format!("{:.2}", c))
                .unwrap_or_default(),
            plan_type: p.plan_type.clone().unwrap_or_default(),
        }
    }
}

/// Wrapper for displaying bare metal plans in a table (monthly pricing)
#[derive(Tabled)]
struct BareMetalPlanMonthlyRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "CPUs")]
    cpu_count: String,
    #[tabled(rename = "Threads")]
    cpu_threads: String,
    #[tabled(rename = "CPU Model")]
    cpu_model: String,
    #[tabled(rename = "RAM (GB)")]
    ram: String,
    #[tabled(rename = "Disk (GB)")]
    disk: String,
    #[tabled(rename = "Disks")]
    disk_count: String,
    #[tabled(rename = "$/month")]
    price: String,
    #[tabled(rename = "Type")]
    plan_type: String,
}

impl From<&BareMetalPlan> for BareMetalPlanMonthlyRow {
    fn from(p: &BareMetalPlan) -> Self {
        Self {
            id: p.id.clone(),
            cpu_count: p.cpu_count.map(|v| v.to_string()).unwrap_or_default(),
            cpu_threads: p.cpu_threads.map(|v| v.to_string()).unwrap_or_default(),
            cpu_model: p.cpu_model.clone().unwrap_or_default(),
            ram: format_ram_gb(p.ram),
            disk: p.disk.map(|d| d.to_string()).unwrap_or_default(),
            disk_count: p.disk_count.map(|d| d.to_string()).unwrap_or_default(),
            price: p
                .monthly_cost
                .map(|c| format!("{:.2}", c))
                .unwrap_or_default(),
            plan_type: p.plan_type.clone().unwrap_or_default(),
        }
    }
}

fn print_bare_metal_plans_table(plans: &[BareMetalPlan], monthly: bool) {
    if plans.is_empty() {
        println!("{}", "No bare metal plans found.".yellow());
        return;
    }

    if monthly {
        let rows: Vec<BareMetalPlanMonthlyRow> =
            plans.iter().map(BareMetalPlanMonthlyRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    } else {
        let rows: Vec<BareMetalPlanHourlyRow> =
            plans.iter().map(BareMetalPlanHourlyRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

pub fn print_bare_metal_plans(plans: &[BareMetalPlan], format: OutputFormat, monthly: bool) {
    match format {
        OutputFormat::Json => print_json(plans),
        OutputFormat::Table => print_bare_metal_plans_table(plans, monthly),
    }
}

impl TableDisplay for Vec<BareMetalPlan> {
    fn print_table(&self) {
        print_bare_metal_plans_table(self, false);
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

#[derive(Tabled)]
struct OsRowNoArch {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
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

impl From<&Os> for OsRowNoArch {
    fn from(o: &Os) -> Self {
        Self {
            id: o.id.to_string(),
            name: o.name.clone().unwrap_or_default(),
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
        let mut sorted: Vec<&Os> = self.iter().collect();
        sorted.sort_by(|a, b| {
            let family_a = a.family.as_deref().unwrap_or("").to_ascii_lowercase();
            let family_b = b.family.as_deref().unwrap_or("").to_ascii_lowercase();
            let name_a = a.name.as_deref().unwrap_or("").to_ascii_lowercase();
            let name_b = b.name.as_deref().unwrap_or("").to_ascii_lowercase();
            family_a
                .cmp(&family_b)
                .then_with(|| name_a.cmp(&name_b))
                .then_with(|| a.id.cmp(&b.id))
        });
        let mut uniform_arch: Option<String> = None;
        let mut all_same = true;
        for os in &sorted {
            let value = os.arch.as_deref().unwrap_or("").trim().to_ascii_lowercase();
            match &uniform_arch {
                None => uniform_arch = Some(value),
                Some(existing) if existing == &value => {}
                Some(_) => {
                    all_same = false;
                    break;
                }
            }
        }

        if all_same {
            let rows: Vec<OsRowNoArch> = sorted.iter().map(|os| OsRowNoArch::from(*os)).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{}", table);
        } else {
            let rows: Vec<OsRow> = sorted.iter().map(|os| OsRow::from(*os)).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{}", table);
        }
    }
}

/// Wrapper for displaying applications in a table
#[derive(Tabled)]
struct ApplicationRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Short Name")]
    short_name: String,
    #[tabled(rename = "Type")]
    app_type: String,
    #[tabled(rename = "Vendor")]
    vendor: String,
}

impl From<&Application> for ApplicationRow {
    fn from(a: &Application) -> Self {
        Self {
            id: a.id.to_string(),
            name: a.name.clone(),
            short_name: a.short_name.clone(),
            app_type: a.app_type.clone(),
            vendor: a.vendor.clone(),
        }
    }
}

impl TableDisplay for Vec<Application> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No applications found.".yellow());
            return;
        }
        let rows: Vec<ApplicationRow> = self.iter().map(ApplicationRow::from).collect();
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
            ip_type: i.ip_type.as_deref().map(format_ip_type).unwrap_or_default(),
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
            ip_type: i.ip_type.as_deref().map(format_ip_type).unwrap_or_default(),
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
    eprintln!("{} {}", "✓".green(), message);
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    eprintln!("{} {}", "⚠".yellow(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    eprintln!("{} {}", "ℹ".cyan(), message);
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

// NodePoolLabel display

#[derive(Tabled)]
struct NodePoolLabelRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Value")]
    value: String,
}

impl From<&NodePoolLabel> for NodePoolLabelRow {
    fn from(label: &NodePoolLabel) -> Self {
        Self {
            id: label.id.clone().unwrap_or_default(),
            key: label.key.clone().unwrap_or_default(),
            value: label.value.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<NodePoolLabel> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No labels found.".yellow());
            return;
        }
        let rows: Vec<NodePoolLabelRow> = self.iter().map(NodePoolLabelRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for NodePoolLabel {
    fn print_table(&self) {
        println!("{}", "Label:".cyan());
        if let Some(id) = &self.id {
            println!("  {}: {}", "ID".green(), id);
        }
        if let Some(key) = &self.key {
            println!("  {}: {}", "Key".green(), key);
        }
        if let Some(value) = &self.value {
            println!("  {}: {}", "Value".green(), value);
        }
    }
}

// NodePoolTaint display

#[derive(Tabled)]
struct NodePoolTaintRow {
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Value")]
    value: String,
    #[tabled(rename = "Effect")]
    effect: String,
}

impl From<&NodePoolTaint> for NodePoolTaintRow {
    fn from(taint: &NodePoolTaint) -> Self {
        Self {
            key: taint.key.clone().unwrap_or_default(),
            value: taint.value.clone().unwrap_or_default(),
            effect: taint.effect.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<NodePoolTaint> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No taints found.".yellow());
            return;
        }
        let rows: Vec<NodePoolTaintRow> = self.iter().map(NodePoolTaintRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
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

impl TableDisplay for ConnectionPoolsResponse {
    fn print_table(&self) {
        if let Some(connections) = &self.connections {
            println!("{}", "Connection Pool Summary:".cyan());
            if let Some(used) = connections.used {
                println!("  {}: {}", "Used".green(), used);
            }
            if let Some(available) = connections.available {
                println!("  {}: {}", "Available".green(), available);
            }
            if let Some(max) = connections.max {
                println!("  {}: {}", "Max".green(), max);
            }
            println!();
        }
        self.connection_pools.print_table();
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

#[derive(Tabled)]
struct DatabaseQuotaRow {
    #[tabled(rename = "Client ID")]
    client_id: String,
    #[tabled(rename = "User")]
    user: String,
    #[tabled(rename = "Consumer Rate")]
    consumer_byte_rate: String,
    #[tabled(rename = "Producer Rate")]
    producer_byte_rate: String,
    #[tabled(rename = "Request %")]
    request_percentage: String,
}

impl From<&DatabaseQuota> for DatabaseQuotaRow {
    fn from(q: &DatabaseQuota) -> Self {
        Self {
            client_id: q.client_id.clone().unwrap_or_else(|| "-".to_string()),
            user: q.user.clone().unwrap_or_else(|| "-".to_string()),
            consumer_byte_rate: q
                .consumer_byte_rate
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| "-".to_string()),
            producer_byte_rate: q
                .producer_byte_rate
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| "-".to_string()),
            request_percentage: q
                .request_percentage
                .map(|v| format!("{}%", v))
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

impl TableDisplay for Vec<DatabaseQuota> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No quotas found.".yellow());
            return;
        }
        let rows: Vec<DatabaseQuotaRow> = self.iter().map(DatabaseQuotaRow::from).collect();
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

// =====================
// DNS Display Types
// =====================

/// Table row for DNS domains
#[derive(Tabled)]
pub struct DnsDomainRow {
    #[tabled(rename = "DOMAIN")]
    pub domain: String,
    #[tabled(rename = "DNSSEC")]
    pub dns_sec: String,
    #[tabled(rename = "CREATED")]
    pub date_created: String,
}

impl From<&DnsDomain> for DnsDomainRow {
    fn from(d: &DnsDomain) -> Self {
        Self {
            domain: d.domain.clone(),
            dns_sec: d.dns_sec.clone().unwrap_or_else(|| "-".to_string()),
            date_created: d.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<DnsDomain> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No DNS domains found.".yellow());
            return;
        }
        let rows: Vec<DnsDomainRow> = self.iter().map(DnsDomainRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for DnsDomain {
    fn print_table(&self) {
        println!("{}", "DNS Domain:".cyan());
        println!("  {}: {}", "Domain".green(), self.domain);
        if let Some(dns_sec) = &self.dns_sec {
            println!("  {}: {}", "DNSSEC".green(), dns_sec);
        }
        if let Some(created) = &self.date_created {
            println!("  {}: {}", "Created".green(), created);
        }
    }
}

/// Table row for DNS records
#[derive(Tabled)]
pub struct DnsRecordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "TYPE")]
    pub record_type: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "DATA")]
    pub data: String,
    #[tabled(rename = "TTL")]
    pub ttl: String,
    #[tabled(rename = "PRIORITY")]
    pub priority: String,
}

impl From<&DnsRecord> for DnsRecordRow {
    fn from(r: &DnsRecord) -> Self {
        Self {
            id: r.id.clone(),
            record_type: r.record_type.clone().unwrap_or_default(),
            name: r.name.clone().unwrap_or_default(),
            data: r.data.clone().unwrap_or_default(),
            ttl: r
                .ttl
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".to_string()),
            priority: r
                .priority
                .map(|p| p.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

impl TableDisplay for Vec<DnsRecord> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No DNS records found.".yellow());
            return;
        }
        let rows: Vec<DnsRecordRow> = self.iter().map(DnsRecordRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for DnsRecord {
    fn print_table(&self) {
        println!("{}", "DNS Record:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(record_type) = &self.record_type {
            println!("  {}: {}", "Type".green(), record_type);
        }
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
        if let Some(data) = &self.data {
            println!("  {}: {}", "Data".green(), data);
        }
        if let Some(ttl) = &self.ttl {
            println!("  {}: {}", "TTL".green(), ttl);
        }
        if let Some(priority) = &self.priority {
            println!("  {}: {}", "Priority".green(), priority);
        }
    }
}

impl TableDisplay for DnsSoa {
    fn print_table(&self) {
        println!("{}", "SOA Information:".cyan());
        if let Some(nsprimary) = &self.nsprimary {
            println!("  {}: {}", "Primary Nameserver".green(), nsprimary);
        }
        if let Some(email) = &self.email {
            println!("  {}: {}", "Contact Email".green(), email);
        }
    }
}

// =====================
// Load Balancer Display
// =====================

/// Wrapper for displaying load balancers in a table
#[derive(Tabled)]
struct LoadBalancerRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "IPv4")]
    ipv4: String,
    #[tabled(rename = "Nodes")]
    nodes: String,
}

impl From<&LoadBalancer> for LoadBalancerRow {
    fn from(lb: &LoadBalancer) -> Self {
        Self {
            id: lb.id.clone(),
            label: lb.label.clone().unwrap_or_default(),
            region: lb.region.clone().unwrap_or_default(),
            status: lb.status.clone().unwrap_or_default(),
            ipv4: lb.ipv4.clone().unwrap_or_default(),
            nodes: lb.nodes.map(|n| n.to_string()).unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<LoadBalancer> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No load balancers found.".yellow());
            return;
        }
        let rows: Vec<LoadBalancerRow> = self.iter().map(LoadBalancerRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for LoadBalancer {
    fn print_table(&self) {
        println!("{}", "Load Balancer:".cyan());
        println!("  {}: {}", "ID".green(), self.id);
        if let Some(label) = &self.label {
            println!("  {}: {}", "Label".green(), label);
        }
        if let Some(region) = &self.region {
            println!("  {}: {}", "Region".green(), region);
        }
        if let Some(status) = &self.status {
            println!("  {}: {}", "Status".green(), status);
        }
        if let Some(ipv4) = &self.ipv4 {
            println!("  {}: {}", "IPv4".green(), ipv4);
        }
        if let Some(ipv6) = &self.ipv6 {
            println!("  {}: {}", "IPv6".green(), ipv6);
        }
        if let Some(nodes) = &self.nodes {
            println!("  {}: {}", "Nodes".green(), nodes);
        }
        if let Some(has_ssl) = &self.has_ssl {
            println!("  {}: {}", "Has SSL".green(), has_ssl);
        }
        if let Some(http2) = &self.http2 {
            println!("  {}: {}", "HTTP2".green(), http2);
        }
        if let Some(http3) = &self.http3 {
            println!("  {}: {}", "HTTP3".green(), http3);
        }
        if let Some(date_created) = &self.date_created {
            println!("  {}: {}", "Created".green(), date_created);
        }
        if let Some(generic_info) = &self.generic_info {
            println!("  {}:", "Generic Info".cyan());
            if let Some(algo) = &generic_info.balancing_algorithm {
                println!("    {}: {}", "Algorithm".green(), algo);
            }
            if let Some(ssl_redirect) = &generic_info.ssl_redirect {
                println!("    {}: {}", "SSL Redirect".green(), ssl_redirect);
            }
            if let Some(proxy) = &generic_info.proxy_protocol {
                println!("    {}: {}", "Proxy Protocol".green(), proxy);
            }
            if let Some(timeout) = &generic_info.timeout {
                println!("    {}: {}s", "Timeout".green(), timeout);
            }
            if let Some(vpc) = &generic_info.vpc {
                println!("    {}: {}", "VPC".green(), vpc);
            }
        }
        if let Some(health_check) = &self.health_check {
            println!("  {}:", "Health Check".cyan());
            if let Some(protocol) = &health_check.protocol {
                println!("    {}: {}", "Protocol".green(), protocol);
            }
            if let Some(port) = &health_check.port {
                println!("    {}: {}", "Port".green(), port);
            }
            if let Some(path) = &health_check.path {
                println!("    {}: {}", "Path".green(), path);
            }
            if let Some(interval) = &health_check.check_interval {
                println!("    {}: {}s", "Interval".green(), interval);
            }
            if let Some(timeout) = &health_check.response_timeout {
                println!("    {}: {}s", "Response Timeout".green(), timeout);
            }
        }
        if !self.forwarding_rules.is_empty() {
            println!(
                "  {}: {}",
                "Forwarding Rules".green(),
                self.forwarding_rules.len()
            );
        }
        if !self.firewall_rules.is_empty() {
            println!(
                "  {}: {}",
                "Firewall Rules".green(),
                self.firewall_rules.len()
            );
        }
        if !self.instances.is_empty() {
            println!("  {}: {}", "Instances".green(), self.instances.join(", "));
        }
    }
}

/// Wrapper for displaying forwarding rules in a table
#[derive(Tabled)]
struct ForwardingRuleRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Frontend Protocol")]
    frontend_protocol: String,
    #[tabled(rename = "Frontend Port")]
    frontend_port: String,
    #[tabled(rename = "Backend Protocol")]
    backend_protocol: String,
    #[tabled(rename = "Backend Port")]
    backend_port: String,
}

impl From<&ForwardingRule> for ForwardingRuleRow {
    fn from(r: &ForwardingRule) -> Self {
        Self {
            id: r.id.clone().unwrap_or_default(),
            frontend_protocol: r.frontend_protocol.clone().unwrap_or_default(),
            frontend_port: r.frontend_port.map(|p| p.to_string()).unwrap_or_default(),
            backend_protocol: r.backend_protocol.clone().unwrap_or_default(),
            backend_port: r.backend_port.map(|p| p.to_string()).unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<ForwardingRule> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No forwarding rules found.".yellow());
            return;
        }
        let rows: Vec<ForwardingRuleRow> = self.iter().map(ForwardingRuleRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for ForwardingRule {
    fn print_table(&self) {
        println!("{}", "Forwarding Rule:".cyan());
        if let Some(id) = &self.id {
            println!("  {}: {}", "ID".green(), id);
        }
        if let Some(protocol) = &self.frontend_protocol {
            println!("  {}: {}", "Frontend Protocol".green(), protocol);
        }
        if let Some(port) = &self.frontend_port {
            println!("  {}: {}", "Frontend Port".green(), port);
        }
        if let Some(protocol) = &self.backend_protocol {
            println!("  {}: {}", "Backend Protocol".green(), protocol);
        }
        if let Some(port) = &self.backend_port {
            println!("  {}: {}", "Backend Port".green(), port);
        }
    }
}

/// Wrapper for displaying LB firewall rules in a table
#[derive(Tabled)]
struct LBFirewallRuleRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "Source")]
    source: String,
    #[tabled(rename = "IP Type")]
    ip_type: String,
}

impl From<&LBFirewallRule> for LBFirewallRuleRow {
    fn from(r: &LBFirewallRule) -> Self {
        Self {
            id: r.id.clone().unwrap_or_default(),
            port: r.port.map(|p| p.to_string()).unwrap_or_default(),
            source: r.source.clone().unwrap_or_default(),
            ip_type: r.ip_type.as_deref().map(format_ip_type).unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<LBFirewallRule> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No firewall rules found.".yellow());
            return;
        }
        let rows: Vec<LBFirewallRuleRow> = self.iter().map(LBFirewallRuleRow::from).collect();
        let table = Table::new(rows).with(Style::sharp()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for LBFirewallRule {
    fn print_table(&self) {
        println!("{}", "Firewall Rule:".cyan());
        if let Some(id) = &self.id {
            println!("  {}: {}", "ID".green(), id);
        }
        if let Some(port) = &self.port {
            println!("  {}: {}", "Port".green(), port);
        }
        if let Some(source) = &self.source {
            println!("  {}: {}", "Source".green(), source);
        }
        if let Some(ip_type) = &self.ip_type {
            println!("  {}: {}", "IP Type".green(), format_ip_type(ip_type));
        }
    }
}

impl TableDisplay for ReverseDNS {
    fn print_table(&self) {
        println!("{}", "Reverse DNS:".cyan());
        if let Some(ipv4) = &self.ipv4 {
            println!("  {}: {}", "IPv4".green(), ipv4);
        }
        if !self.ipv6.is_empty() {
            println!("  {}:", "IPv6".green());
            for entry in &self.ipv6 {
                println!("    - {}", entry);
            }
        }
    }
}

// ==================
// CDN Output
// ==================

/// Wrapper for displaying CDN Pull Zones in a table
#[derive(Tabled)]
struct CdnPullZoneRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Origin")]
    origin: String,
    #[tabled(rename = "CDN URL")]
    cdn_url: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Active")]
    active: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&CdnPullZone> for CdnPullZoneRow {
    fn from(z: &CdnPullZone) -> Self {
        let origin = format!(
            "{}://{}",
            z.origin_scheme
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            z.origin_domain.clone().unwrap_or_default()
        );
        Self {
            id: z.id.clone(),
            label: z.label.clone().unwrap_or_default(),
            origin,
            cdn_url: z.cdn_url.clone().unwrap_or_default(),
            status: z
                .status
                .as_ref()
                .map(|st| st.to_string())
                .unwrap_or_default(),
            active: if z.is_active() {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
            date_created: z.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for CdnPullZone {
    fn print_table(&self) {
        let rows = vec![CdnPullZoneRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<CdnPullZone> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No CDN Pull Zones found.".yellow());
            return;
        }
        let rows: Vec<CdnPullZoneRow> = self.iter().map(CdnPullZoneRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying CDN Push Zones in a table
#[derive(Tabled)]
struct CdnPushZoneRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "CDN URL")]
    cdn_url: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Active")]
    active: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&CdnPushZone> for CdnPushZoneRow {
    fn from(z: &CdnPushZone) -> Self {
        Self {
            id: z.id.clone(),
            label: z.label.clone().unwrap_or_default(),
            cdn_url: z.cdn_url.clone().unwrap_or_default(),
            status: z
                .status
                .as_ref()
                .map(|st| st.to_string())
                .unwrap_or_default(),
            active: if z.is_active() {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
            date_created: z.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for CdnPushZone {
    fn print_table(&self) {
        let rows = vec![CdnPushZoneRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<CdnPushZone> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No CDN Push Zones found.".yellow());
            return;
        }
        let rows: Vec<CdnPushZoneRow> = self.iter().map(CdnPushZoneRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying CDN Push Zone Files in a table
#[derive(Tabled)]
struct CdnPushZoneFileRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Last Modified")]
    last_modified: String,
}

impl From<&CdnPushZoneFileMeta> for CdnPushZoneFileRow {
    fn from(f: &CdnPushZoneFileMeta) -> Self {
        Self {
            name: f.name.clone().unwrap_or_default(),
            size: f.size.clone().unwrap_or_default(),
            last_modified: f.last_modified.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for CdnPushZoneFileMeta {
    fn print_table(&self) {
        let rows = vec![CdnPushZoneFileRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<CdnPushZoneFileMeta> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No files found.".yellow());
            return;
        }
        let rows: Vec<CdnPushZoneFileRow> = self.iter().map(CdnPushZoneFileRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for CdnPushZoneFile {
    fn print_table(&self) {
        println!("{}", "File Details:".cyan());
        if let Some(name) = &self.name {
            println!("  {}: {}", "Name".green(), name);
        }
        if let Some(mime) = &self.mime {
            println!("  {}: {}", "MIME Type".green(), mime);
        }
        if let Some(size) = &self.size {
            println!("  {}: {}", "Size".green(), size);
        }
        if let Some(last_modified) = &self.last_modified {
            println!("  {}: {}", "Last Modified".green(), last_modified);
        }
    }
}

impl TableDisplay for CdnUploadEndpoint {
    fn print_table(&self) {
        println!("{}", "Upload Endpoint:".cyan());
        if let Some(url) = &self.url {
            println!("  {}: {}", "URL".green(), url);
        }
        if let Some(inputs) = &self.inputs {
            println!("{}", "  Inputs:".cyan());
            if let Some(acl) = &inputs.acl {
                println!("    {}: {}", "ACL".green(), acl);
            }
            if let Some(key) = &inputs.key {
                println!("    {}: {}", "Key".green(), key);
            }
            if let Some(algo) = &inputs.x_amz_algorithm {
                println!("    {}: {}", "Algorithm".green(), algo);
            }
        }
    }
}

// ===========================
// Container Registry Types
// ===========================

/// Wrapper for displaying registries in a table
#[derive(Tabled)]
struct RegistryRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "URN")]
    urn: String,
    #[tabled(rename = "Public")]
    public: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Registry> for RegistryRow {
    fn from(r: &Registry) -> Self {
        Self {
            id: r.id.clone(),
            name: r.name.clone().unwrap_or_default(),
            urn: r.urn.clone().unwrap_or_default(),
            public: if r.public { "Yes" } else { "No" }.to_string(),
            date_created: r.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Registry {
    fn print_table(&self) {
        let rows = vec![RegistryRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);

        // Print additional details
        if let Some(storage) = &self.storage {
            println!("\n{}", "Storage:".cyan());
            if let Some(used) = &storage.used {
                if let Some(mb) = used.mb {
                    println!("  {}: {:.2} MB", "Used".green(), mb);
                }
            }
            if let Some(allowed) = &storage.allowed {
                if let Some(mb) = allowed.mb {
                    println!("  {}: {:.2} MB", "Allowed".green(), mb);
                }
            }
        }

        if let Some(root_user) = &self.root_user {
            println!("\n{}", "Root User:".cyan());
            if let Some(username) = &root_user.username {
                println!("  {}: {}", "Username".green(), username);
            }
        }
    }
}

impl TableDisplay for Vec<Registry> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No registries found.".yellow());
            return;
        }
        let rows: Vec<RegistryRow> = self.iter().map(RegistryRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry repositories in a table
#[derive(Tabled)]
struct RegistryRepositoryRow {
    #[tabled(rename = "Image")]
    image: String,
    #[tabled(rename = "Artifacts")]
    artifact_count: String,
    #[tabled(rename = "Pulls")]
    pull_count: String,
    #[tabled(rename = "Updated")]
    updated_at: String,
}

impl From<&RegistryRepository> for RegistryRepositoryRow {
    fn from(r: &RegistryRepository) -> Self {
        Self {
            image: r.image.clone().unwrap_or_default(),
            artifact_count: r.artifact_count.map(|c| c.to_string()).unwrap_or_default(),
            pull_count: r.pull_count.map(|c| c.to_string()).unwrap_or_default(),
            updated_at: r.updated_at.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryRepository {
    fn print_table(&self) {
        let rows = vec![RegistryRepositoryRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                println!("\n{}: {}", "Description".cyan(), desc);
            }
        }
    }
}

impl TableDisplay for Vec<RegistryRepository> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No repositories found.".yellow());
            return;
        }
        let rows: Vec<RegistryRepositoryRow> =
            self.iter().map(RegistryRepositoryRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry artifacts in a table
#[derive(Tabled)]
struct RegistryArtifactRow {
    #[tabled(rename = "Digest")]
    digest: String,
    #[tabled(rename = "Type")]
    artifact_type: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Pushed")]
    push_time: String,
}

impl From<&RegistryArtifact> for RegistryArtifactRow {
    fn from(a: &RegistryArtifact) -> Self {
        let size = a
            .size
            .map(|s| {
                if s > 1_000_000 {
                    format!("{:.2} MB", s as f64 / 1_000_000.0)
                } else if s > 1_000 {
                    format!("{:.2} KB", s as f64 / 1_000.0)
                } else {
                    format!("{} B", s)
                }
            })
            .unwrap_or_default();

        Self {
            digest: a
                .digest
                .clone()
                .unwrap_or_default()
                .chars()
                .take(20)
                .collect::<String>()
                + "...",
            artifact_type: a.artifact_kind.clone().unwrap_or_default(),
            size,
            push_time: a.push_time.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryArtifact {
    fn print_table(&self) {
        println!("{}", "Artifact Details:".cyan());
        if let Some(digest) = &self.digest {
            println!("  {}: {}", "Digest".green(), digest);
        }
        if let Some(kind) = &self.artifact_kind {
            println!("  {}: {}", "Type".green(), kind);
        }
        if let Some(size) = &self.size {
            println!("  {}: {} bytes", "Size".green(), size);
        }
        if let Some(push) = &self.push_time {
            println!("  {}: {}", "Push Time".green(), push);
        }
        if let Some(pull) = &self.pull_time {
            println!("  {}: {}", "Pull Time".green(), pull);
        }
        if !self.tags.is_empty() {
            println!("  {}: {:?}", "Tags".green(), self.tags);
        }
    }
}

impl TableDisplay for Vec<RegistryArtifact> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No artifacts found.".yellow());
            return;
        }
        let rows: Vec<RegistryArtifactRow> = self.iter().map(RegistryArtifactRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry robots in a table
#[derive(Tabled)]
struct RegistryRobotRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Disabled")]
    disabled: String,
    #[tabled(rename = "Duration")]
    duration: String,
}

impl From<&RegistryRobot> for RegistryRobotRow {
    fn from(r: &RegistryRobot) -> Self {
        Self {
            name: r.name.clone().unwrap_or_default(),
            description: r.description.clone().unwrap_or_default(),
            disabled: if r.disable { "Yes" } else { "No" }.to_string(),
            duration: r
                .duration
                .map(|d| {
                    if d == -1 {
                        "Never expires".to_string()
                    } else {
                        format!("{} seconds", d)
                    }
                })
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryRobot {
    fn print_table(&self) {
        let rows = vec![RegistryRobotRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);

        if let Some(secret) = &self.secret {
            println!("\n{}: {}", "Secret".cyan(), secret);
        }
    }
}

impl TableDisplay for Vec<RegistryRobot> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No robot accounts found.".yellow());
            return;
        }
        let rows: Vec<RegistryRobotRow> = self.iter().map(RegistryRobotRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry replications in a table
#[derive(Tabled)]
struct RegistryReplicationRow {
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Namespace")]
    namespace: String,
    #[tabled(rename = "URN")]
    urn: String,
}

impl From<&RegistryReplication> for RegistryReplicationRow {
    fn from(r: &RegistryReplication) -> Self {
        Self {
            region: r.region.clone().unwrap_or_default(),
            namespace: r.namespace.clone().unwrap_or_default(),
            urn: r.urn.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryReplication {
    fn print_table(&self) {
        let rows = vec![RegistryReplicationRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<RegistryReplication> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No replications found.".yellow());
            return;
        }
        let rows: Vec<RegistryReplicationRow> =
            self.iter().map(RegistryReplicationRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry retention rules in a table
#[derive(Tabled)]
struct RegistryRetentionRuleRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Template")]
    template: String,
    #[tabled(rename = "Action")]
    action: String,
    #[tabled(rename = "Disabled")]
    disabled: String,
}

impl From<&RegistryRetentionRule> for RegistryRetentionRuleRow {
    fn from(r: &RegistryRetentionRule) -> Self {
        Self {
            id: r.id.map(|i| i.to_string()).unwrap_or_default(),
            template: r.template.clone().unwrap_or_default(),
            action: r.action.clone().unwrap_or_default(),
            disabled: if r.disabled { "Yes" } else { "No" }.to_string(),
        }
    }
}

impl TableDisplay for RegistryRetentionRule {
    fn print_table(&self) {
        let rows = vec![RegistryRetentionRuleRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);

        if let Some(params) = &self.params {
            println!("\n{}", "Parameters:".cyan());
            for (k, v) in params {
                println!("  {}: {}", k.green(), v);
            }
        }
    }
}

impl TableDisplay for Vec<RegistryRetentionRule> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No retention rules found.".yellow());
            return;
        }
        let rows: Vec<RegistryRetentionRuleRow> =
            self.iter().map(RegistryRetentionRuleRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry retention executions in a table
#[derive(Tabled)]
struct RegistryRetentionExecutionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Trigger")]
    trigger: String,
    #[tabled(rename = "Start Time")]
    start_time: String,
}

impl From<&RegistryRetentionExecution> for RegistryRetentionExecutionRow {
    fn from(e: &RegistryRetentionExecution) -> Self {
        Self {
            id: e.id.map(|i| i.to_string()).unwrap_or_default(),
            status: e.status.clone().unwrap_or_default(),
            trigger: e.trigger.clone().unwrap_or_default(),
            start_time: e.start_time.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryRetentionExecution {
    fn print_table(&self) {
        let rows = vec![RegistryRetentionExecutionRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<RegistryRetentionExecution> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No retention executions found.".yellow());
            return;
        }
        let rows: Vec<RegistryRetentionExecutionRow> = self
            .iter()
            .map(RegistryRetentionExecutionRow::from)
            .collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for RegistryRetentionSchedule {
    fn print_table(&self) {
        println!("{}", "Retention Schedule:".cyan());
        if let Some(stype) = &self.schedule_type {
            println!("  {}: {}", "Type".green(), stype);
        }
        if let Some(cron) = &self.cron {
            println!("  {}: {}", "Cron".green(), cron);
        }
    }
}

/// Wrapper for displaying registry regions in a table
#[derive(Tabled)]
struct RegistryRegionRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "URN")]
    urn: String,
    #[tabled(rename = "Public")]
    public: String,
}

impl From<&RegistryRegion> for RegistryRegionRow {
    fn from(r: &RegistryRegion) -> Self {
        Self {
            name: r.name.clone().unwrap_or_default(),
            urn: r.urn.clone().unwrap_or_default(),
            public: if r.public { "Yes" } else { "No" }.to_string(),
        }
    }
}

impl TableDisplay for RegistryRegion {
    fn print_table(&self) {
        let rows = vec![RegistryRegionRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<RegistryRegion> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No registry regions found.".yellow());
            return;
        }
        let rows: Vec<RegistryRegionRow> = self.iter().map(RegistryRegionRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

/// Wrapper for displaying registry plans in a table
#[derive(Tabled)]
struct RegistryPlanRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    vanity_name: String,
    #[tabled(rename = "Max Storage (MB)")]
    max_storage_mb: String,
    #[tabled(rename = "Monthly Price")]
    monthly_price: String,
}

impl From<&RegistryPlan> for RegistryPlanRow {
    fn from(p: &RegistryPlan) -> Self {
        Self {
            id: p.id.clone(),
            vanity_name: p.vanity_name.clone().unwrap_or_default(),
            max_storage_mb: p.max_storage_mb.map(|s| s.to_string()).unwrap_or_default(),
            monthly_price: p
                .monthly_price
                .map(|p| format!("${}", p))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for RegistryPlan {
    fn print_table(&self) {
        let rows = vec![RegistryPlanRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<RegistryPlan> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No registry plans found.".yellow());
            return;
        }
        let rows: Vec<RegistryPlanRow> = self.iter().map(RegistryPlanRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for RegistryDockerCredentials {
    fn print_table(&self) {
        println!("{}", "Docker Credentials:".cyan());
        if let Some(auths) = &self.auths {
            for (registry, auth) in auths {
                println!("  {}: {}", "Registry".green(), registry);
                if let Some(auth_str) = &auth.auth {
                    println!("    {}: {}", "Auth".green(), auth_str);
                }
            }
        }
    }
}

impl TableDisplay for RegistryKubernetesCredentials {
    fn print_table(&self) {
        println!("{}", "Kubernetes Docker Credentials:".cyan());
        if let Some(api_version) = &self.api_version {
            println!("  {}: {}", "API Version".green(), api_version);
        }
        if let Some(kind) = &self.kind {
            println!("  {}: {}", "Kind".green(), kind);
        }
        if let Some(cred_type) = &self.cred_type {
            println!("  {}: {}", "Type".green(), cred_type);
        }
        if let Some(metadata) = &self.metadata {
            if let Some(name) = &metadata.name {
                println!("  {}: {}", "Secret Name".green(), name);
            }
        }
        if let Some(data) = &self.data {
            if let Some(dockerconfig) = &data.dockerconfigjson {
                println!("  {}: {}", ".dockerconfigjson".green(), dockerconfig);
            }
        }
    }
}

// ==================
// Account TableDisplay
// ==================

impl TableDisplay for Account {
    fn print_table(&self) {
        println!("{}", "Account Information".cyan().bold());
        println!(
            "  {}: {}",
            "Name".green(),
            self.name.as_deref().unwrap_or("-")
        );
        println!(
            "  {}: {}",
            "Email".green(),
            self.email.as_deref().unwrap_or("-")
        );
        println!(
            "  {}: ${:.2}",
            "Balance".green(),
            self.balance.unwrap_or(0.0)
        );
        println!(
            "  {}: ${:.2}",
            "Pending Charges".green(),
            self.pending_charges.unwrap_or(0.0)
        );
        if let Some(date) = &self.last_payment_date {
            println!("  {}: {}", "Last Payment Date".green(), date);
        }
        if let Some(amount) = self.last_payment_amount {
            println!("  {}: ${:.2}", "Last Payment Amount".green(), amount);
        }
        if !self.acls.is_empty() {
            println!("  {}: {}", "ACLs".green(), self.acls.join(", "));
        }
    }
}

impl TableDisplay for BgpInfo {
    fn print_table(&self) {
        println!("{}", "BGP Information".cyan().bold());
        println!(
            "  {}: {}",
            "Enabled".green(),
            self.enabled
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("-")
        );
        if let Some(asn) = self.asn {
            println!("  {}: {}", "ASN".green(), asn);
        }
        if !self.allowed_prefix_ipv4.is_empty() {
            println!("  {}:", "IPv4 Prefixes".green());
            for prefix in &self.allowed_prefix_ipv4 {
                println!(
                    "    - {} ({})",
                    prefix.prefix.as_deref().unwrap_or("-"),
                    prefix.description.as_deref().unwrap_or("no description")
                );
            }
        }
        if !self.allowed_prefix_ipv6.is_empty() {
            println!("  {}:", "IPv6 Prefixes".green());
            for prefix in &self.allowed_prefix_ipv6 {
                println!(
                    "    - {} ({})",
                    prefix.prefix.as_deref().unwrap_or("-"),
                    prefix.description.as_deref().unwrap_or("no description")
                );
            }
        }
    }
}

impl TableDisplay for AccountBandwidth {
    fn print_table(&self) {
        println!("{}", "Account Bandwidth".cyan().bold());
        if let Some(prev) = &self.previous_month {
            println!("  {}:", "Previous Month".green());
            println!(
                "    {}: {} bytes",
                "Incoming".cyan(),
                prev.incoming_bytes.unwrap_or(0)
            );
            println!(
                "    {}: {} bytes",
                "Outgoing".cyan(),
                prev.outgoing_bytes.unwrap_or(0)
            );
            if let Some(total) = prev.gb_total {
                println!("    {}: {:.2} GB", "Total".cyan(), total);
            }
        }
        if let Some(curr) = &self.current_month_to_date {
            println!("  {}:", "Current Month".green());
            println!(
                "    {}: {} bytes",
                "Incoming".cyan(),
                curr.incoming_bytes.unwrap_or(0)
            );
            println!(
                "    {}: {} bytes",
                "Outgoing".cyan(),
                curr.outgoing_bytes.unwrap_or(0)
            );
            if let Some(total) = curr.gb_total {
                println!("    {}: {:.2} GB", "Total".cyan(), total);
            }
        }
    }
}

// ==================
// Billing TableDisplay
// ==================

#[derive(Tabled)]
struct BillingHistoryRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Type")]
    entry_type: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Balance")]
    balance: String,
}

impl From<&BillingHistory> for BillingHistoryRow {
    fn from(b: &BillingHistory) -> Self {
        Self {
            id: b.id.map(|i| i.to_string()).unwrap_or_default(),
            date: b.date.clone().unwrap_or_default(),
            entry_type: b.entry_type.clone().unwrap_or_default(),
            description: b.description.clone().unwrap_or_default(),
            amount: format!("${:.2}", b.amount.unwrap_or(0.0)),
            balance: format!("${:.2}", b.balance.unwrap_or(0.0)),
        }
    }
}

impl TableDisplay for BillingHistory {
    fn print_table(&self) {
        let rows = vec![BillingHistoryRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<BillingHistory> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No billing history found.".yellow());
            return;
        }
        let rows: Vec<BillingHistoryRow> = self.iter().map(BillingHistoryRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct InvoiceRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Balance")]
    balance: String,
}

impl From<&Invoice> for InvoiceRow {
    fn from(i: &Invoice) -> Self {
        Self {
            id: i.id.map(|id| id.to_string()).unwrap_or_default(),
            date: i.date.clone().unwrap_or_default(),
            description: i.description.clone().unwrap_or_default(),
            amount: format!("${:.2}", i.amount.unwrap_or(0.0)),
            balance: format!("${:.2}", i.balance.unwrap_or(0.0)),
        }
    }
}

impl TableDisplay for Invoice {
    fn print_table(&self) {
        let rows = vec![InvoiceRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<Invoice> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No invoices found.".yellow());
            return;
        }
        let rows: Vec<InvoiceRow> = self.iter().map(InvoiceRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct InvoiceItemRow {
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Product")]
    product: String,
    #[tabled(rename = "Start")]
    start_date: String,
    #[tabled(rename = "End")]
    end_date: String,
    #[tabled(rename = "Units")]
    units: String,
    #[tabled(rename = "Unit Type")]
    unit_type: String,
    #[tabled(rename = "Unit Price")]
    unit_price: String,
    #[tabled(rename = "Total")]
    total: String,
}

impl From<&InvoiceItem> for InvoiceItemRow {
    fn from(i: &InvoiceItem) -> Self {
        Self {
            description: i.description.clone().unwrap_or_default(),
            product: i.product.clone().unwrap_or_default(),
            start_date: i.start_date.clone().unwrap_or_default(),
            end_date: i.end_date.clone().unwrap_or_default(),
            units: i.units.map(|u| u.to_string()).unwrap_or_default(),
            unit_type: i.unit_type.clone().unwrap_or_default(),
            unit_price: format!("${:.4}", i.unit_price.unwrap_or(0.0)),
            total: format!("${:.2}", i.total.unwrap_or(0.0)),
        }
    }
}

impl TableDisplay for InvoiceItem {
    fn print_table(&self) {
        let rows = vec![InvoiceItemRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<InvoiceItem> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No invoice items found.".yellow());
            return;
        }
        let rows: Vec<InvoiceItemRow> = self.iter().map(InvoiceItemRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct PendingChargeRow {
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Amount")]
    amount: String,
}

impl From<&PendingCharge> for PendingChargeRow {
    fn from(p: &PendingCharge) -> Self {
        Self {
            description: p.description.clone().unwrap_or_default(),
            date: p.date.clone().unwrap_or_default(),
            amount: format!("${:.2}", p.amount.unwrap_or(0.0)),
        }
    }
}

impl TableDisplay for PendingCharge {
    fn print_table(&self) {
        let rows = vec![PendingChargeRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<PendingCharge> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No pending charges found.".yellow());
            return;
        }
        let rows: Vec<PendingChargeRow> = self.iter().map(PendingChargeRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// ==================
// User TableDisplay
// ==================

#[derive(Tabled)]
struct UserRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Email")]
    email: String,
    #[tabled(rename = "API Enabled")]
    api_enabled: String,
}

impl From<&User> for UserRow {
    fn from(u: &User) -> Self {
        Self {
            id: u.id.clone(),
            name: u.name.clone().unwrap_or_default(),
            email: u.email.clone().unwrap_or_default(),
            api_enabled: u
                .api_enabled
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("-")
                .to_string(),
        }
    }
}

impl TableDisplay for User {
    fn print_table(&self) {
        let rows = vec![UserRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if !self.acls.is_empty() {
            println!("\n{}: {}", "ACLs".cyan(), self.acls.join(", "));
        }
    }
}

impl TableDisplay for Vec<User> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No users found.".yellow());
            return;
        }
        let rows: Vec<UserRow> = self.iter().map(UserRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct ApiKeyRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&ApiKey> for ApiKeyRow {
    fn from(k: &ApiKey) -> Self {
        Self {
            id: k.id.clone().unwrap_or_default(),
            name: k.name.clone().unwrap_or_default(),
            date_created: k.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for ApiKey {
    fn print_table(&self) {
        let rows = vec![ApiKeyRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
        if let Some(key) = &self.api_key {
            println!("\n{}: {}", "API Key".cyan(), key);
        }
    }
}

impl TableDisplay for Vec<ApiKey> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No API keys found.".yellow());
            return;
        }
        let rows: Vec<ApiKeyRow> = self.iter().map(ApiKeyRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct IpWhitelistEntryRow {
    #[tabled(rename = "Subnet")]
    subnet: String,
    #[tabled(rename = "Size")]
    subnet_size: String,
    #[tabled(rename = "Type")]
    ip_type: String,
    #[tabled(rename = "Date Added")]
    date_added: String,
}

impl From<&IpWhitelistEntry> for IpWhitelistEntryRow {
    fn from(e: &IpWhitelistEntry) -> Self {
        Self {
            subnet: e.subnet.clone().unwrap_or_default(),
            subnet_size: e.subnet_size.map(|s| s.to_string()).unwrap_or_default(),
            ip_type: e.ip_type.as_deref().map(format_ip_type).unwrap_or_default(),
            date_added: e.date_added.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for IpWhitelistEntry {
    fn print_table(&self) {
        let rows = vec![IpWhitelistEntryRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<IpWhitelistEntry> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No IP whitelist entries found.".yellow());
            return;
        }
        let rows: Vec<IpWhitelistEntryRow> = self.iter().map(IpWhitelistEntryRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Marketplace App Variables
// =====================

#[derive(Tabled)]
struct AppVariableRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Required")]
    required: String,
    #[tabled(rename = "Description")]
    description: String,
}

impl From<&AppVariable> for AppVariableRow {
    fn from(v: &AppVariable) -> Self {
        Self {
            name: v.name.clone().unwrap_or_default(),
            required: v
                .required
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".to_string()),
            description: v.description.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<AppVariable> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No app variables found.".yellow());
            return;
        }
        let rows: Vec<AppVariableRow> = self.iter().map(AppVariableRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Inference
// =====================

#[derive(Tabled)]
struct InferenceSubscriptionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Created")]
    date_created: String,
    #[tabled(rename = "API Key")]
    api_key: String,
}

impl From<&InferenceSubscription> for InferenceSubscriptionRow {
    fn from(s: &InferenceSubscription) -> Self {
        Self {
            id: s.id.clone().unwrap_or_default(),
            label: s.label.clone().unwrap_or_default(),
            date_created: s.date_created.clone().unwrap_or_default(),
            api_key: s.api_key.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for InferenceSubscription {
    fn print_table(&self) {
        let rows = vec![InferenceSubscriptionRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vec<InferenceSubscription> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No inference subscriptions found.".yellow());
            return;
        }
        let rows: Vec<InferenceSubscriptionRow> =
            self.iter().map(InferenceSubscriptionRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for InferenceUsage {
    fn print_table(&self) {
        println!("{}", "Inference Usage:".cyan());
        if let Some(chat) = &self.chat {
            println!("  {}:", "Chat".green());
            if let Some(tokens) = &chat.current_tokens {
                println!("    {}: {}", "Current Tokens".cyan(), tokens);
            }
            if let Some(allotment) = &chat.monthly_allotment {
                println!("    {}: {}", "Monthly Allotment".cyan(), allotment);
            }
            if let Some(overage) = &chat.overage {
                println!("    {}: {}", "Overage".cyan(), overage);
            }
        }
        if let Some(audio) = &self.audio {
            println!("  {}:", "Audio".green());
            if let Some(chars) = &audio.tts_characters {
                println!("    {}: {}", "TTS Characters".cyan(), chars);
            }
            if let Some(chars) = &audio.tts_sm_characters {
                println!("    {}: {}", "TTS SM Characters".cyan(), chars);
            }
        }
    }
}

// =====================
// Logs
// =====================

#[derive(Tabled)]
struct LogRow {
    #[tabled(rename = "Timestamp")]
    timestamp: String,
    #[tabled(rename = "Level")]
    level: String,
    #[tabled(rename = "Resource Type")]
    resource_type: String,
    #[tabled(rename = "Resource ID")]
    resource_id: String,
    #[tabled(rename = "Message")]
    message: String,
}

impl From<&Log> for LogRow {
    fn from(l: &Log) -> Self {
        Self {
            timestamp: l.timestamp.clone().unwrap_or_default(),
            level: l.log_level.clone().unwrap_or_default(),
            resource_type: l.resource_type.clone().unwrap_or_default(),
            resource_id: l.resource_id.clone().unwrap_or_default(),
            message: l.message.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Log> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No logs found.".yellow());
            return;
        }
        let rows: Vec<LogRow> = self.iter().map(LogRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for LogsResponse {
    fn print_table(&self) {
        println!("{}", "Log Summary:".cyan());
        println!(
            "  {}: {} / {}",
            "Returned".green(),
            self.meta.returned_count,
            self.meta.total_count
        );
        if self.meta.unreturned_count > 0 {
            println!("  {}: {}", "Remaining".green(), self.meta.unreturned_count);
        }
        if !self.meta.next_page_url.is_empty() {
            println!("  {}: {}", "Next Page".green(), self.meta.next_page_url);
        }
        if !self.meta.continue_time.is_empty() {
            println!("  {}: {}", "Continue Time".green(), self.meta.continue_time);
        }
        println!();
        self.logs.print_table();
    }
}

// =====================
// Subaccounts
// =====================

#[derive(Tabled)]
struct SubaccountRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Email")]
    email: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Subaccount ID")]
    subaccount_id: String,
    #[tabled(rename = "Activated")]
    activated: String,
    #[tabled(rename = "Balance")]
    balance: String,
    #[tabled(rename = "Pending Charges")]
    pending_charges: String,
}

impl From<&Subaccount> for SubaccountRow {
    fn from(s: &Subaccount) -> Self {
        Self {
            id: s.id.clone().unwrap_or_default(),
            email: s.email.clone().unwrap_or_default(),
            name: s.subaccount_name.clone().unwrap_or_default(),
            subaccount_id: s.subaccount_id.clone().unwrap_or_default(),
            activated: s
                .activated
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            balance: s.balance.map(|v| format!("{:.2}", v)).unwrap_or_default(),
            pending_charges: s
                .pending_charges
                .map(|v| format!("{:.2}", v))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Subaccount> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No subaccounts found.".yellow());
            return;
        }
        let rows: Vec<SubaccountRow> = self.iter().map(SubaccountRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Subaccount {
    fn print_table(&self) {
        let rows = vec![SubaccountRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Private Networks
// =====================

#[derive(Tabled)]
struct NetworkRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "CIDR")]
    cidr: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Created")]
    date_created: String,
}

impl From<&Network> for NetworkRow {
    fn from(n: &Network) -> Self {
        Self {
            id: n.id.clone(),
            region: n.region.clone().unwrap_or_default(),
            cidr: n.cidr().unwrap_or_default(),
            description: n.description.clone().unwrap_or_default(),
            date_created: n.date_created.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Network> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No private networks found.".yellow());
            return;
        }
        let rows: Vec<NetworkRow> = self.iter().map(NetworkRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Network {
    fn print_table(&self) {
        let rows = vec![NetworkRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// VPC Attachments
// =====================

#[derive(Tabled)]
struct VpcAttachmentRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Type")]
    attachment_type: String,
    #[tabled(rename = "MAC")]
    mac_address: String,
    #[tabled(rename = "IPv4")]
    ipv4: String,
    #[tabled(rename = "Date Added")]
    date_added: String,
}

impl From<&VpcAttachment> for VpcAttachmentRow {
    fn from(a: &VpcAttachment) -> Self {
        Self {
            id: a.id.clone().unwrap_or_default(),
            attachment_type: a.attachment_type.clone().unwrap_or_default(),
            mac_address: a.mac_address.clone().unwrap_or_default(),
            ipv4: a
                .ip
                .as_ref()
                .and_then(|ip| ip.v4.clone())
                .unwrap_or_default(),
            date_added: a.date_added.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<VpcAttachment> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VPC attachments found.".yellow());
            return;
        }
        let rows: Vec<VpcAttachmentRow> = self.iter().map(VpcAttachmentRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Storage Gateways
// =====================

#[derive(Tabled)]
struct StorageGatewayRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Type")]
    gateway_type: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Health")]
    health: String,
    #[tabled(rename = "Pending")]
    pending_charges: String,
}

impl From<&StorageGateway> for StorageGatewayRow {
    fn from(g: &StorageGateway) -> Self {
        Self {
            id: g.id.clone().unwrap_or_default(),
            label: g.label.clone().unwrap_or_default(),
            gateway_type: g.gateway_type.clone().unwrap_or_default(),
            status: g.status.clone().unwrap_or_default(),
            health: g.health.clone().unwrap_or_default(),
            pending_charges: g
                .pending_charges
                .map(|v| format!("{:.2}", v))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<StorageGateway> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No storage gateways found.".yellow());
            return;
        }
        let rows: Vec<StorageGatewayRow> = self.iter().map(StorageGatewayRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for StorageGateway {
    fn print_table(&self) {
        let rows = vec![StorageGatewayRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct StorageGatewayExportRow {
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "VFS UUID")]
    vfs_uuid: String,
    #[tabled(rename = "Pseudo Root")]
    pseudo_root_path: String,
    #[tabled(rename = "Allowed IPs")]
    allowed_ips: String,
}

impl From<&StorageGatewayExport> for StorageGatewayExportRow {
    fn from(e: &StorageGatewayExport) -> Self {
        Self {
            label: e.label.clone().unwrap_or_default(),
            vfs_uuid: e.vfs_uuid.clone().unwrap_or_default(),
            pseudo_root_path: e.pseudo_root_path.clone().unwrap_or_default(),
            allowed_ips: e
                .allowed_ips
                .as_ref()
                .map(|ips| ips.join(", "))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<StorageGatewayExport> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No storage gateway exports found.".yellow());
            return;
        }
        let rows: Vec<StorageGatewayExportRow> =
            self.iter().map(StorageGatewayExportRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for StorageGatewayExport {
    fn print_table(&self) {
        let rows = vec![StorageGatewayExportRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// VFS
// =====================

#[derive(Tabled)]
struct VfsRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Label")]
    label: String,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Size (GB)")]
    size_gb: String,
    #[tabled(rename = "Used (GB)")]
    used_gb: String,
    #[tabled(rename = "Disk")]
    disk_type: String,
}

impl From<&Vfs> for VfsRow {
    fn from(v: &Vfs) -> Self {
        Self {
            id: v.id.clone().unwrap_or_default(),
            label: v.label.clone().unwrap_or_default(),
            region: v.region.clone().unwrap_or_default(),
            status: v.status.clone().unwrap_or_default(),
            size_gb: v
                .storage_size
                .as_ref()
                .and_then(|s| s.gb)
                .map(|g| g.to_string())
                .unwrap_or_default(),
            used_gb: v
                .storage_used
                .as_ref()
                .and_then(|s| s.gb)
                .map(|g| g.to_string())
                .unwrap_or_default(),
            disk_type: v.disk_type.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<Vfs> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VFS subscriptions found.".yellow());
            return;
        }
        let rows: Vec<VfsRow> = self.iter().map(VfsRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for Vfs {
    fn print_table(&self) {
        let rows = vec![VfsRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct VfsRegionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Country")]
    country: String,
    #[tabled(rename = "NVMe $/GB")]
    nvme_price: String,
    #[tabled(rename = "HDD $/GB")]
    hdd_price: String,
    #[tabled(rename = "Min NVMe (GB)")]
    min_nvme: String,
    #[tabled(rename = "Min HDD (GB)")]
    min_hdd: String,
}

impl From<&VfsRegion> for VfsRegionRow {
    fn from(r: &VfsRegion) -> Self {
        Self {
            id: r.id.clone().unwrap_or_default(),
            description: r.description.clone().unwrap_or_default(),
            country: r.country.clone().unwrap_or_default(),
            nvme_price: r
                .price_per_gb
                .as_ref()
                .and_then(|p| p.nvme)
                .map(|v| format!("{:.2}", v))
                .unwrap_or_default(),
            hdd_price: r
                .price_per_gb
                .as_ref()
                .and_then(|p| p.hdd)
                .map(|v| format!("{:.2}", v))
                .unwrap_or_default(),
            min_nvme: r
                .min_size_gb
                .as_ref()
                .and_then(|m| m.nvme)
                .map(|v| v.to_string())
                .unwrap_or_default(),
            min_hdd: r
                .min_size_gb
                .as_ref()
                .and_then(|m| m.hdd)
                .map(|v| v.to_string())
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<VfsRegion> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VFS regions found.".yellow());
            return;
        }
        let rows: Vec<VfsRegionRow> = self.iter().map(VfsRegionRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

#[derive(Tabled)]
struct VfsAttachmentRow {
    #[tabled(rename = "Target ID")]
    target_id: String,
    #[tabled(rename = "State")]
    state: String,
    #[tabled(rename = "Mount Tag")]
    mount_tag: String,
}

impl From<&VfsAttachment> for VfsAttachmentRow {
    fn from(a: &VfsAttachment) -> Self {
        Self {
            target_id: a.target_id.clone().unwrap_or_default(),
            state: a.state.clone().unwrap_or_default(),
            mount_tag: a.mount_tag.map(|v| v.to_string()).unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<VfsAttachment> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No VFS attachments found.".yellow());
            return;
        }
        let rows: Vec<VfsAttachmentRow> = self.iter().map(VfsAttachmentRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

impl TableDisplay for VfsAttachment {
    fn print_table(&self) {
        let rows = vec![VfsAttachmentRow::from(self)];
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Database Connector Configuration
// =====================

#[derive(Tabled)]
struct ConnectorConfigRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    option_type: String,
    #[tabled(rename = "Required")]
    required: String,
    #[tabled(rename = "Default")]
    default_value: String,
    #[tabled(rename = "Description")]
    description: String,
}

impl From<&DatabaseConnectorConfigurationSchema> for ConnectorConfigRow {
    fn from(c: &DatabaseConnectorConfigurationSchema) -> Self {
        Self {
            name: c.name.clone().unwrap_or_default(),
            option_type: c.option_type.clone().unwrap_or_default(),
            required: c
                .required
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            default_value: c.default_value.clone().unwrap_or_default(),
            description: c.description.clone().unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<DatabaseConnectorConfigurationSchema> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No connector configuration options found.".yellow());
            return;
        }
        let rows: Vec<ConnectorConfigRow> = self.iter().map(ConnectorConfigRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
    }
}

// =====================
// Database Available Options
// =====================

#[derive(Tabled)]
struct DatabaseAvailableOptionRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    option_type: String,
    #[tabled(rename = "Min")]
    min: String,
    #[tabled(rename = "Max")]
    max: String,
    #[tabled(rename = "Units")]
    units: String,
    #[tabled(rename = "Enumerals")]
    enumerals: String,
}

impl From<&DatabaseAvailableOption> for DatabaseAvailableOptionRow {
    fn from(o: &DatabaseAvailableOption) -> Self {
        Self {
            name: o.name.clone().unwrap_or_default(),
            option_type: o.option_type.clone().unwrap_or_default(),
            min: o.min_value.map(|v| v.to_string()).unwrap_or_default(),
            max: o.max_value.map(|v| v.to_string()).unwrap_or_default(),
            units: o.units.clone().unwrap_or_default(),
            enumerals: o
                .enumerals
                .as_ref()
                .map(|vals| vals.join(", "))
                .unwrap_or_default(),
        }
    }
}

impl TableDisplay for Vec<DatabaseAvailableOption> {
    fn print_table(&self) {
        if self.is_empty() {
            println!("{}", "No available options found.".yellow());
            return;
        }
        let rows: Vec<DatabaseAvailableOptionRow> =
            self.iter().map(DatabaseAvailableOptionRow::from).collect();
        let table = Table::new(rows).with(Style::rounded()).to_string();
        println!("{}", table);
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
            internet: None,
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
        assert_eq!(row.ram, "1");
        assert_eq!(row.monthly, "5.00");
        assert_eq!(row.plan_type, "Cloud Compute");
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
        assert_eq!(row.script_type, "Boot");
    }
}
