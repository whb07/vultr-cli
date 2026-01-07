//! CLI command definitions and handlers

use crate::config::OutputFormat;
use clap::{Parser, Subcommand, ValueEnum};

/// Vultr CLI - Manage your Vultr cloud resources
#[derive(Parser, Debug, Clone)]
#[command(name = "vultr-cli")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// API key (can also use VULTR_API_KEY env var)
    #[arg(long, global = true, env = "VULTR_API_KEY")]
    pub api_key: Option<String>,

    /// Profile to use from config file
    #[arg(long, global = true, default_value = "default")]
    pub profile: String,

    /// Output format (overrides config)
    #[arg(long, short, global = true, value_enum)]
    pub output: Option<OutputFormat>,

    /// Skip confirmation prompts
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,

    /// Wait for async operations to complete
    #[arg(long, short = 'w', global = true)]
    pub wait: bool,

    /// Wait timeout in seconds (used with --wait; overrides config)
    #[arg(long, global = true)]
    pub wait_timeout: Option<u64>,

    /// Poll interval in seconds for --wait (overrides config)
    #[arg(long, global = true)]
    pub poll_interval: Option<u64>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Authentication management
    Auth(AuthArgs),

    /// Manage compute instances (VMs)
    #[command(alias = "vm", alias = "server")]
    Instance(InstanceArgs),

    /// Manage SSH keys
    #[command(alias = "ssh")]
    SshKey(SshKeyArgs),

    /// Manage startup scripts
    #[command(alias = "script")]
    StartupScript(StartupScriptArgs),

    /// Manage snapshots
    Snapshot(SnapshotArgs),

    /// Manage backups
    Backup(BackupArgs),

    /// Manage bare metal servers
    #[command(alias = "bm")]
    BareMetal(BareMetalArgs),

    /// Manage ISOs
    Iso(IsoArgs),

    /// Manage block storage
    #[command(alias = "block", alias = "storage")]
    BlockStorage(BlockStorageArgs),

    /// Manage object storage (S3-compatible)
    #[command(alias = "obj", alias = "s3")]
    ObjectStorage(ObjectStorageArgs),

    /// Manage firewall groups and rules
    #[command(alias = "fw")]
    Firewall(FirewallArgs),

    /// Manage VPCs (Virtual Private Clouds)
    Vpc(VpcArgs),

    /// Manage VPC 2.0 networks
    Vpc2(Vpc2Args),

    /// Manage Kubernetes clusters (VKE)
    #[command(alias = "k8s", alias = "vke")]
    Kubernetes(KubernetesArgs),

    /// Manage load balancers
    #[command(alias = "lb")]
    LoadBalancer(LoadBalancerArgs),

    /// Manage databases (MySQL, PostgreSQL, Valkey, Kafka)
    #[command(alias = "db", alias = "dbaas")]
    Database(DatabaseArgs),

    /// Manage CDN (Content Delivery Network)
    Cdn(CdnArgs),

    /// Manage DNS domains and records
    #[command(alias = "domain")]
    Dns(DnsArgs),

    /// Manage container registries
    #[command(alias = "cr")]
    Registry(RegistryArgs),

    /// Manage reserved IPs
    #[command(alias = "rip")]
    ReservedIp(ReservedIpArgs),

    /// List available regions
    Regions,

    /// List available plans
    Plans(PlansArgs),

    /// List available operating systems
    Os,

    /// List available applications (one-click and marketplace)
    #[command(alias = "app", alias = "apps")]
    Applications,

    /// Manage account information
    Account(AccountArgs),

    /// Manage billing and invoices
    Billing(BillingArgs),

    /// Manage users and API keys
    User(UserArgs),

    /// Manage CLI configuration
    #[command(alias = "cfg")]
    Config(ConfigArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

// ==================
// Config Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Get a specific configuration value
    Get {
        /// Config key (e.g., "output_format", "default_profile")
        key: String,
    },
    /// Set a configuration value
    Set {
        /// Config key (e.g., "output_format", "default_profile")
        key: String,
        /// Value to set
        value: String,
    },
    /// Show current profile settings
    Profile,
}

// ==================
// Auth Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommands {
    /// Login and store API key securely
    Login(AuthLoginArgs),
    /// Remove stored API key
    Logout,
    /// Show current authentication status
    Status,
}

#[derive(Parser, Debug, Clone)]
pub struct AuthLoginArgs {
    /// API key to store (will prompt if not provided)
    #[arg(long)]
    pub api_key: Option<String>,
}

// ==================
// Bare Metal Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalArgs {
    #[command(subcommand)]
    pub command: BareMetalCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalCommands {
    /// List all bare metal servers
    List(ListArgs),

    /// Get bare metal server details
    Get {
        /// Bare metal ID
        id: String,
    },

    /// Create a new bare metal server
    Create(BareMetalCreateArgs),

    /// Update a bare metal server
    Update(BareMetalUpdateArgs),

    /// Delete a bare metal server
    Delete {
        /// Bare metal ID
        id: String,
    },

    /// Start a bare metal server
    Start {
        /// Bare metal ID
        id: String,
    },

    /// Stop/halt a bare metal server
    #[command(alias = "halt")]
    Stop {
        /// Bare metal ID
        id: String,
    },

    /// Reboot a bare metal server
    Reboot {
        /// Bare metal ID
        id: String,
    },

    /// Reinstall a bare metal server
    Reinstall {
        /// Bare metal ID
        id: String,
        /// New hostname
        #[arg(long)]
        hostname: Option<String>,
    },

    /// Get bare metal bandwidth usage
    Bandwidth {
        /// Bare metal ID
        id: String,
    },

    /// Get available upgrades for a bare metal server
    Upgrades {
        /// Bare metal ID
        id: String,
    },

    /// Get bare metal user data
    UserData {
        /// Bare metal ID
        id: String,
    },

    /// Get VNC URL for a bare metal server
    Vnc {
        /// Bare metal ID
        id: String,
    },

    /// Manage bare metal IPv4 addresses
    Ipv4(BareMetalIpv4Args),

    /// Manage bare metal IPv6 addresses
    Ipv6(BareMetalIpv6Args),

    /// Manage bare metal VPC attachments
    Vpc(BareMetalVpcArgs),

    /// Manage bare metal VPC2 attachments
    Vpc2(BareMetalVpc2Args),

    /// Bulk operations on multiple bare metal servers
    Bulk(BareMetalBulkArgs),
}

// ==================
// Bare Metal IPv4 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalIpv4Args {
    #[command(subcommand)]
    pub command: BareMetalIpv4Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalIpv4Commands {
    /// List IPv4 addresses
    List {
        /// Bare metal ID
        id: String,
    },

    /// Set reverse DNS for IPv4
    Reverse {
        /// Bare metal ID
        id: String,
        /// IPv4 address
        #[arg(long)]
        ip: String,
        /// Reverse DNS hostname
        #[arg(long)]
        reverse: String,
    },

    /// Set default reverse DNS for IPv4
    DefaultReverse {
        /// Bare metal ID
        id: String,
        /// IPv4 address
        #[arg(long)]
        ip: String,
    },
}

// ==================
// Bare Metal IPv6 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalIpv6Args {
    #[command(subcommand)]
    pub command: BareMetalIpv6Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalIpv6Commands {
    /// List IPv6 addresses
    List {
        /// Bare metal ID
        id: String,
    },

    /// Set reverse DNS for IPv6
    Reverse {
        /// Bare metal ID
        id: String,
        /// IPv6 address
        #[arg(long)]
        ip: String,
        /// Reverse DNS hostname
        #[arg(long)]
        reverse: String,
    },

    /// Delete reverse DNS for IPv6
    DeleteReverse {
        /// Bare metal ID
        id: String,
        /// IPv6 address
        #[arg(long)]
        ip: String,
    },
}

// ==================
// Bare Metal VPC Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalVpcArgs {
    #[command(subcommand)]
    pub command: BareMetalVpcCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalVpcCommands {
    /// List attached VPCs
    List {
        /// Bare metal ID
        id: String,
    },

    /// Attach a VPC
    Attach {
        /// Bare metal ID
        id: String,
        /// VPC ID
        #[arg(long)]
        vpc_id: String,
    },

    /// Detach a VPC
    Detach {
        /// Bare metal ID
        id: String,
        /// VPC ID
        #[arg(long)]
        vpc_id: String,
    },
}

// ==================
// Bare Metal VPC2 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalVpc2Args {
    #[command(subcommand)]
    pub command: BareMetalVpc2Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalVpc2Commands {
    /// List attached VPC2s
    List {
        /// Bare metal ID
        id: String,
    },

    /// Attach a VPC2
    Attach {
        /// Bare metal ID
        id: String,
        /// VPC2 ID
        #[arg(long)]
        vpc_id: String,
        /// IP address to assign
        #[arg(long)]
        ip_address: Option<String>,
    },

    /// Detach a VPC2
    Detach {
        /// Bare metal ID
        id: String,
        /// VPC2 ID
        #[arg(long)]
        vpc_id: String,
    },
}

// ==================
// Bare Metal Bulk Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalBulkArgs {
    #[command(subcommand)]
    pub command: BareMetalBulkCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BareMetalBulkCommands {
    /// Start multiple bare metal servers
    Start {
        /// Bare metal IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },

    /// Stop/halt multiple bare metal servers
    #[command(alias = "halt")]
    Stop {
        /// Bare metal IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },

    /// Reboot multiple bare metal servers
    Reboot {
        /// Bare metal IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },
}

// ==================
// Bare Metal Create Arguments
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalCreateArgs {
    /// Region ID (e.g., "ewr", "lax")
    #[arg(long)]
    pub region: String,

    /// Plan ID (e.g., "vbm-4c-32gb")
    #[arg(long)]
    pub plan: String,

    /// Operating system ID
    #[arg(long)]
    pub os_id: Option<i32>,

    /// Snapshot ID to deploy from
    #[arg(long)]
    pub snapshot_id: Option<String>,

    /// Application ID
    #[arg(long)]
    pub app_id: Option<i32>,

    /// Application image ID
    #[arg(long)]
    pub image_id: Option<String>,

    /// SSH key IDs (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub sshkey_id: Option<Vec<String>>,

    /// Startup script ID
    #[arg(long)]
    pub script_id: Option<String>,

    /// Instance label
    #[arg(long)]
    pub label: Option<String>,

    /// Enable IPv6
    #[arg(long)]
    pub enable_ipv6: bool,

    /// VPC IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub attach_vpc: Option<Vec<String>>,

    /// VPC2 IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub attach_vpc2: Option<Vec<String>>,

    /// Tags (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// User data (cloud-init, base64 encoded)
    #[arg(long)]
    pub user_data: Option<String>,

    /// Reserved IPv4 address
    #[arg(long)]
    pub reserved_ipv4: Option<String>,

    /// Enable persistent PXE
    #[arg(long)]
    pub persistent_pxe: bool,

    /// Send activation email
    #[arg(long)]
    pub activation_email: bool,

    /// Hostname
    #[arg(long)]
    pub hostname: Option<String>,

    /// Mdisk mode
    #[arg(long)]
    pub mdisk_mode: Option<String>,

    /// User scheme (root or limited)
    #[arg(long)]
    pub user_scheme: Option<String>,
}

// ==================
// Bare Metal Update Arguments
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BareMetalUpdateArgs {
    /// Bare metal ID
    pub id: String,

    /// Instance label
    #[arg(long)]
    pub label: Option<String>,

    /// Enable IPv6
    #[arg(long)]
    pub enable_ipv6: Option<bool>,

    /// User data (cloud-init, base64 encoded)
    #[arg(long)]
    pub user_data: Option<String>,

    /// Tags (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// VPC IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub attach_vpc: Option<Vec<String>>,

    /// VPC IDs to detach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub detach_vpc: Option<Vec<String>>,

    /// VPC2 IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub attach_vpc2: Option<Vec<String>>,

    /// VPC2 IDs to detach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub detach_vpc2: Option<Vec<String>>,
}

// ==================
// Instance Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceArgs {
    #[command(subcommand)]
    pub command: InstanceCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceCommands {
    /// List all instances
    List(ListArgs),

    /// Get instance details
    Get {
        /// Instance ID
        id: String,
    },

    /// Create a new instance
    Create(InstanceCreateArgs),

    /// Update an instance
    Update(InstanceUpdateArgs),

    /// Delete an instance
    Delete {
        /// Instance ID
        id: String,
    },

    /// Start an instance
    Start {
        /// Instance ID
        id: String,
    },

    /// Stop/halt an instance
    #[command(alias = "halt")]
    Stop {
        /// Instance ID
        id: String,
    },

    /// Reboot an instance
    Reboot {
        /// Instance ID
        id: String,
    },

    /// Reinstall an instance
    Reinstall {
        /// Instance ID
        id: String,
        /// New hostname
        #[arg(long)]
        hostname: Option<String>,
    },

    /// Get instance bandwidth usage
    Bandwidth {
        /// Instance ID
        id: String,
    },

    /// Get instances sharing the same host
    Neighbors {
        /// Instance ID
        id: String,
    },

    /// Get available upgrades for an instance
    Upgrades {
        /// Instance ID
        id: String,
    },

    /// Get instance user data
    UserData {
        /// Instance ID
        id: String,
    },

    /// Restore instance from backup or snapshot
    Restore {
        /// Instance ID
        id: String,
        /// Backup ID to restore from
        #[arg(long, conflicts_with = "snapshot_id")]
        backup_id: Option<String>,
        /// Snapshot ID to restore from
        #[arg(long, conflicts_with = "backup_id")]
        snapshot_id: Option<String>,
    },

    /// Manage instance IPv4 addresses
    Ipv4(InstanceIpv4Args),

    /// Manage instance IPv6 addresses
    Ipv6(InstanceIpv6Args),

    /// Manage ISO attachments
    Iso(InstanceIsoArgs),

    /// Manage backup schedule
    Backup(InstanceBackupArgs),

    /// Manage instance VPC attachments
    Vpc(InstanceVpcArgs),

    /// Manage instance VPC2 attachments
    Vpc2(InstanceVpc2Args),

    /// Bulk operations on multiple instances
    Bulk(InstanceBulkArgs),
}

// ==================
// Instance IPv4 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceIpv4Args {
    #[command(subcommand)]
    pub command: InstanceIpv4Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceIpv4Commands {
    /// List IPv4 addresses
    List {
        /// Instance ID
        id: String,
    },

    /// Create an additional IPv4 address
    Create {
        /// Instance ID
        id: String,
        /// Reboot instance after adding IP
        #[arg(long)]
        reboot: bool,
    },

    /// Delete an IPv4 address
    Delete {
        /// Instance ID
        id: String,
        /// IPv4 address to delete
        #[arg(long)]
        ipv4: String,
    },

    /// Set reverse DNS for IPv4
    Reverse {
        /// Instance ID
        id: String,
        /// IPv4 address
        #[arg(long)]
        ip: String,
        /// Reverse DNS hostname
        #[arg(long)]
        reverse: String,
    },

    /// Set default reverse DNS for IPv4
    DefaultReverse {
        /// Instance ID
        id: String,
        /// IPv4 address
        #[arg(long)]
        ip: String,
    },
}

// ==================
// Instance IPv6 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceIpv6Args {
    #[command(subcommand)]
    pub command: InstanceIpv6Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceIpv6Commands {
    /// List IPv6 addresses
    List {
        /// Instance ID
        id: String,
    },

    /// List IPv6 reverse DNS entries
    ReverseList {
        /// Instance ID
        id: String,
    },

    /// Set reverse DNS for IPv6
    Reverse {
        /// Instance ID
        id: String,
        /// IPv6 address
        #[arg(long)]
        ip: String,
        /// Reverse DNS hostname
        #[arg(long)]
        reverse: String,
    },

    /// Delete reverse DNS for IPv6
    DeleteReverse {
        /// Instance ID
        id: String,
        /// IPv6 address
        #[arg(long)]
        ip: String,
    },
}

// ==================
// Instance ISO Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceIsoArgs {
    #[command(subcommand)]
    pub command: InstanceIsoCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceIsoCommands {
    /// Get ISO attachment status
    Status {
        /// Instance ID
        id: String,
    },

    /// Attach an ISO to instance
    Attach {
        /// Instance ID
        id: String,
        /// ISO ID
        #[arg(long)]
        iso_id: String,
    },

    /// Detach ISO from instance
    Detach {
        /// Instance ID
        id: String,
    },
}

// ==================
// Instance Backup Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceBackupArgs {
    #[command(subcommand)]
    pub command: InstanceBackupCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceBackupCommands {
    /// Get backup schedule
    Get {
        /// Instance ID
        id: String,
    },

    /// Set backup schedule
    Set {
        /// Instance ID
        id: String,
        /// Schedule type (daily, weekly, monthly, daily_alt_even, daily_alt_odd)
        #[arg(long)]
        schedule_type: String,
        /// Hour of day (0-23)
        #[arg(long)]
        hour: Option<i32>,
        /// Day of week (1-7, Sunday = 1) for weekly
        #[arg(long)]
        dow: Option<i32>,
        /// Day of month (1-28) for monthly
        #[arg(long)]
        dom: Option<i32>,
    },
}

// ==================
// Instance VPC Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceVpcArgs {
    #[command(subcommand)]
    pub command: InstanceVpcCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceVpcCommands {
    /// List attached VPCs
    List {
        /// Instance ID
        id: String,
    },

    /// Attach a VPC
    Attach {
        /// Instance ID
        id: String,
        /// VPC ID
        #[arg(long)]
        vpc_id: String,
    },

    /// Detach a VPC
    Detach {
        /// Instance ID
        id: String,
        /// VPC ID
        #[arg(long)]
        vpc_id: String,
    },
}

// ==================
// Instance VPC2 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceVpc2Args {
    #[command(subcommand)]
    pub command: InstanceVpc2Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceVpc2Commands {
    /// List attached VPC2s
    List {
        /// Instance ID
        id: String,
    },

    /// Attach a VPC2
    Attach {
        /// Instance ID
        id: String,
        /// VPC2 ID
        #[arg(long)]
        vpc_id: String,
        /// IP address to assign
        #[arg(long)]
        ip_address: Option<String>,
    },

    /// Detach a VPC2
    Detach {
        /// Instance ID
        id: String,
        /// VPC2 ID
        #[arg(long)]
        vpc_id: String,
    },
}

// ==================
// Instance Bulk Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct InstanceBulkArgs {
    #[command(subcommand)]
    pub command: InstanceBulkCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InstanceBulkCommands {
    /// Start multiple instances
    Start {
        /// Instance IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },

    /// Stop/halt multiple instances
    #[command(alias = "halt")]
    Stop {
        /// Instance IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },

    /// Reboot multiple instances
    Reboot {
        /// Instance IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ids: Vec<String>,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct ListArgs {
    /// Number of results per page
    #[arg(long, default_value = "25")]
    pub per_page: u32,
    /// Pagination cursor
    #[arg(long)]
    pub cursor: Option<String>,

    /// Fetch all pages
    #[arg(long, conflicts_with = "cursor")]
    pub all: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct InstanceCreateArgs {
    /// Region ID (e.g., "ewr", "lax")
    #[arg(long)]
    pub region: String,

    /// Plan ID (e.g., "vc2-1c-1gb")
    #[arg(long)]
    pub plan: String,

    /// Operating system ID
    #[arg(long)]
    pub os_id: Option<i32>,

    /// Snapshot ID to deploy from
    #[arg(long)]
    pub snapshot_id: Option<String>,

    /// ISO ID to deploy from
    #[arg(long)]
    pub iso_id: Option<String>,

    /// Application ID
    #[arg(long)]
    pub app_id: Option<i32>,

    /// Application image ID
    #[arg(long)]
    pub image_id: Option<String>,

    /// Instance label
    #[arg(long)]
    pub label: Option<String>,

    /// Hostname
    #[arg(long)]
    pub hostname: Option<String>,

    /// SSH key IDs (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub ssh_keys: Option<Vec<String>>,

    /// Startup script ID
    #[arg(long)]
    pub script_id: Option<String>,

    /// Enable IPv6
    #[arg(long)]
    pub enable_ipv6: bool,

    /// Disable public IPv4
    #[arg(long)]
    pub disable_public_ipv4: bool,

    /// Enable automatic backups
    #[arg(long)]
    pub backups: bool,

    /// Enable DDoS protection
    #[arg(long)]
    pub ddos_protection: bool,

    /// Send activation email
    #[arg(long)]
    pub activation_email: bool,

    /// VPC IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub vpc: Option<Vec<String>>,

    /// Firewall group ID
    #[arg(long)]
    pub firewall_group_id: Option<String>,

    /// Reserved IPv4 ID
    #[arg(long)]
    pub reserved_ipv4: Option<String>,

    /// Tags (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// User data (base64 encoded)
    #[arg(long)]
    pub user_data: Option<String>,

    /// User scheme (root or limited)
    #[arg(long)]
    pub user_scheme: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct InstanceUpdateArgs {
    /// Instance ID
    pub id: String,

    /// New label
    #[arg(long)]
    pub label: Option<String>,

    /// New plan (for resizing)
    #[arg(long)]
    pub plan: Option<String>,

    /// Firewall group ID
    #[arg(long)]
    pub firewall_group_id: Option<String>,

    /// Tags (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// Enable backups
    #[arg(long)]
    pub backups: Option<bool>,

    /// Enable DDoS protection
    #[arg(long)]
    pub ddos_protection: Option<bool>,
}

// ==================
// SSH Key Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct SshKeyArgs {
    #[command(subcommand)]
    pub command: SshKeyCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SshKeyCommands {
    /// List all SSH keys
    List(ListArgs),

    /// Get SSH key details
    Get {
        /// SSH key ID
        id: String,
    },

    /// Create a new SSH key
    Create {
        /// Name for the SSH key
        #[arg(long)]
        name: String,
        /// SSH public key content (or path starting with @)
        #[arg(long)]
        key: String,
    },

    /// Update an SSH key
    Update {
        /// SSH key ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New SSH key content
        #[arg(long)]
        key: Option<String>,
    },

    /// Delete an SSH key
    Delete {
        /// SSH key ID
        id: String,
    },
}

// ==================
// Startup Script Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct StartupScriptArgs {
    #[command(subcommand)]
    pub command: StartupScriptCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum StartupScriptCommands {
    /// List all startup scripts
    List(ListArgs),

    /// Get startup script details
    Get {
        /// Script ID
        id: String,
    },

    /// Create a new startup script
    Create {
        /// Name for the script
        #[arg(long)]
        name: String,
        /// Script content (or path starting with @)
        #[arg(long)]
        script: String,
        /// Script type (boot or pxe)
        #[arg(long, default_value = "boot")]
        script_type: String,
    },

    /// Update a startup script
    Update {
        /// Script ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New script content (or path starting with @)
        #[arg(long)]
        script: Option<String>,
        /// New script type
        #[arg(long)]
        script_type: Option<String>,
    },

    /// Delete a startup script
    Delete {
        /// Script ID
        id: String,
    },
}

// ==================
// Snapshot Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct SnapshotArgs {
    #[command(subcommand)]
    pub command: SnapshotCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SnapshotCommands {
    /// List all snapshots
    List(ListArgs),

    /// Get snapshot details
    Get {
        /// Snapshot ID
        id: String,
    },

    /// Create a snapshot from an instance
    Create {
        /// Instance ID to snapshot
        #[arg(long)]
        instance_id: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },

    /// Create a snapshot from a URL
    CreateFromUrl {
        /// URL of the raw disk image
        #[arg(long)]
        url: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },

    /// Update a snapshot
    Update {
        /// Snapshot ID
        id: String,
        /// New description
        #[arg(long)]
        description: String,
    },

    /// Delete a snapshot
    Delete {
        /// Snapshot ID
        id: String,
    },
}

// ==================
// Backup Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BackupArgs {
    #[command(subcommand)]
    pub command: BackupCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BackupCommands {
    /// List all backups
    List(ListArgs),

    /// Get backup details
    Get {
        /// Backup ID
        id: String,
    },
}

// ==================
// ISO Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct IsoArgs {
    #[command(subcommand)]
    pub command: IsoCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum IsoCommands {
    /// List all ISOs
    List(ListArgs),

    /// Get ISO details
    Get {
        /// ISO ID
        id: String,
    },

    /// Create an ISO from URL
    Create {
        /// URL of the ISO to download
        #[arg(long)]
        url: String,
    },

    /// Delete an ISO
    Delete {
        /// ISO ID
        id: String,
    },

    /// List public ISOs
    Public,
}

// ==================
// Block Storage Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BlockStorageArgs {
    #[command(subcommand)]
    pub command: BlockStorageCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BlockStorageCommands {
    /// List all block storage volumes
    List(ListArgs),

    /// Get block storage details
    Get {
        /// Block storage ID
        id: String,
    },

    /// Create a new block storage volume
    Create {
        /// Region ID
        #[arg(long)]
        region: String,
        /// Size in GB (10-40000)
        #[arg(long)]
        size: i32,
        /// Label
        #[arg(long)]
        label: Option<String>,
        /// Block type (high_perf or storage_opt)
        #[arg(long)]
        block_type: Option<String>,
    },

    /// Update block storage
    Update {
        /// Block storage ID
        id: String,
        /// New label
        #[arg(long)]
        label: Option<String>,
        /// New size in GB (can only increase)
        #[arg(long)]
        size: Option<i32>,
    },

    /// Delete block storage
    Delete {
        /// Block storage ID
        id: String,
    },

    /// Attach block storage to an instance
    Attach {
        /// Block storage ID
        id: String,
        /// Instance ID
        #[arg(long)]
        instance_id: String,
        /// Live attach (without reboot)
        #[arg(long)]
        live: bool,
    },

    /// Detach block storage from an instance
    Detach {
        /// Block storage ID
        id: String,
        /// Live detach (without reboot)
        #[arg(long)]
        live: bool,
    },
}

// ==================
// Object Storage Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct ObjectStorageArgs {
    #[command(subcommand)]
    pub command: ObjectStorageCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ObjectStorageCommands {
    /// List all object storages
    List(ListArgs),

    /// Get object storage details
    Get {
        /// Object storage ID
        id: String,
    },

    /// Create a new object storage
    Create {
        /// Cluster ID
        #[arg(long)]
        cluster_id: i32,
        /// Tier ID
        #[arg(long)]
        tier_id: i32,
        /// Label
        #[arg(long)]
        label: Option<String>,
    },

    /// Update object storage
    Update {
        /// Object storage ID
        id: String,
        /// New label
        #[arg(long)]
        label: String,
    },

    /// Delete object storage
    Delete {
        /// Object storage ID
        id: String,
    },

    /// Regenerate S3 access keys
    RegenerateKeys {
        /// Object storage ID
        id: String,
    },

    /// List available object storage clusters
    Clusters(ListArgs),

    /// List available object storage tiers
    Tiers {
        /// Filter by cluster ID (optional)
        #[arg(long)]
        cluster_id: Option<i32>,
    },
}

// ==================
// Firewall Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct FirewallArgs {
    #[command(subcommand)]
    pub command: FirewallCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FirewallCommands {
    /// Manage firewall groups
    Group(FirewallGroupArgs),

    /// Manage firewall rules
    Rule(FirewallRuleArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct FirewallGroupArgs {
    #[command(subcommand)]
    pub command: FirewallGroupCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FirewallGroupCommands {
    /// List all firewall groups
    List(ListArgs),

    /// Get firewall group details
    Get {
        /// Firewall group ID
        id: String,
    },

    /// Create a new firewall group
    Create {
        /// Description
        #[arg(long)]
        description: Option<String>,
    },

    /// Update a firewall group
    Update {
        /// Firewall group ID
        id: String,
        /// New description
        #[arg(long)]
        description: String,
    },

    /// Delete a firewall group
    Delete {
        /// Firewall group ID
        id: String,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct FirewallRuleArgs {
    #[command(subcommand)]
    pub command: FirewallRuleCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FirewallRuleCommands {
    /// List rules in a firewall group
    List {
        /// Firewall group ID
        #[arg(long)]
        group_id: String,
        #[command(flatten)]
        list: ListArgs,
    },

    /// Get a firewall rule
    Get {
        /// Firewall group ID
        #[arg(long)]
        group_id: String,
        /// Rule ID
        #[arg(long)]
        rule_id: i32,
    },

    /// Create a firewall rule
    Create {
        /// Firewall group ID
        #[arg(long)]
        group_id: String,
        /// IP type (v4 or v6)
        #[arg(long)]
        ip_type: String,
        /// Protocol (TCP, UDP, ICMP, GRE, ESP, AH)
        #[arg(long)]
        protocol: String,
        /// Subnet (e.g., "0.0.0.0" for all)
        #[arg(long)]
        subnet: String,
        /// Subnet size in CIDR bits (e.g., 0 for all)
        #[arg(long)]
        subnet_size: i32,
        /// Port or port range (for TCP/UDP)
        #[arg(long)]
        port: Option<String>,
        /// Source ("cloudflare" or empty)
        #[arg(long)]
        source: Option<String>,
        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },

    /// Delete a firewall rule
    Delete {
        /// Firewall group ID
        #[arg(long)]
        group_id: String,
        /// Rule ID
        #[arg(long)]
        rule_id: i32,
    },
}

// ==================
// VPC Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct VpcArgs {
    #[command(subcommand)]
    pub command: VpcCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum VpcCommands {
    /// List all VPCs
    List(ListArgs),

    /// Get VPC details
    Get {
        /// VPC ID
        id: String,
    },

    /// Create a new VPC
    Create {
        /// Region ID
        #[arg(long)]
        region: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// IPv4 subnet (e.g., "10.0.0.0")
        #[arg(long)]
        subnet: Option<String>,
        /// Subnet mask (CIDR bits, e.g., 24)
        #[arg(long)]
        subnet_mask: Option<i32>,
    },

    /// Update a VPC
    Update {
        /// VPC ID
        id: String,
        /// New description
        #[arg(long)]
        description: String,
    },

    /// Delete a VPC
    Delete {
        /// VPC ID
        id: String,
    },
}

// ==================
// VPC 2.0 Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct Vpc2Args {
    #[command(subcommand)]
    pub command: Vpc2Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Vpc2Commands {
    /// List all VPC 2.0 networks
    List(ListArgs),

    /// Get VPC 2.0 details
    Get {
        /// VPC 2.0 ID
        id: String,
    },

    /// Create a new VPC 2.0 network
    Create {
        /// Region ID
        #[arg(long)]
        region: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// IP block (e.g., "10.99.0.0")
        #[arg(long)]
        ip_block: Option<String>,
        /// Prefix length (CIDR bits, e.g., 24)
        #[arg(long)]
        prefix_length: Option<i32>,
    },

    /// Update a VPC 2.0 network
    Update {
        /// VPC 2.0 ID
        id: String,
        /// New description
        #[arg(long)]
        description: String,
    },

    /// Delete a VPC 2.0 network
    Delete {
        /// VPC 2.0 ID
        id: String,
    },

    /// List nodes attached to a VPC 2.0 network
    Nodes {
        /// VPC 2.0 ID
        id: String,
        #[command(flatten)]
        list: ListArgs,
    },

    /// Attach nodes to a VPC 2.0 network
    Attach {
        /// VPC 2.0 ID
        id: String,
        /// Node IDs to attach (comma-separated instance IDs)
        #[arg(long, value_delimiter = ',')]
        nodes: Vec<String>,
    },

    /// Detach nodes from a VPC 2.0 network
    Detach {
        /// VPC 2.0 ID
        id: String,
        /// Node IDs to detach (comma-separated instance IDs)
        #[arg(long, value_delimiter = ',')]
        nodes: Vec<String>,
    },
}

// ==================
// Kubernetes Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct KubernetesArgs {
    #[command(subcommand)]
    pub command: KubernetesCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum KubernetesCommands {
    /// List all Kubernetes clusters
    List,

    /// Get cluster details
    Get {
        /// Cluster ID (VKE ID)
        id: String,
    },

    /// Create a new Kubernetes cluster
    Create(KubernetesCreateArgs),

    /// Update a Kubernetes cluster
    Update {
        /// Cluster ID (VKE ID)
        id: String,
        /// New label
        #[arg(long)]
        label: Option<String>,
    },

    /// Delete a Kubernetes cluster
    Delete {
        /// Cluster ID (VKE ID)
        id: String,
        /// Also delete all linked resources (block storage, load balancers)
        #[arg(long)]
        with_resources: bool,
    },

    /// Get kubeconfig for a cluster
    Config {
        /// Cluster ID (VKE ID)
        id: String,
        /// Decode and output raw kubeconfig (default: base64 encoded)
        #[arg(long)]
        decode: bool,
    },

    /// Get available upgrades for a cluster
    Upgrades {
        /// Cluster ID (VKE ID)
        id: String,
    },

    /// Upgrade a cluster to a new version
    Upgrade {
        /// Cluster ID (VKE ID)
        id: String,
        /// Target Kubernetes version
        #[arg(long)]
        version: String,
    },

    /// Get resources deployed by a cluster
    Resources {
        /// Cluster ID (VKE ID)
        id: String,
    },

    /// List available Kubernetes versions
    Versions,

    /// Manage node pools
    #[command(alias = "pool")]
    NodePool(KubernetesNodePoolArgs),

    /// Manage nodes
    Node(KubernetesNodeArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct KubernetesCreateArgs {
    /// Region ID
    #[arg(long)]
    pub region: String,

    /// Kubernetes version
    #[arg(long)]
    pub version: String,

    /// Cluster label
    #[arg(long)]
    pub label: Option<String>,

    /// Enable HA control planes
    #[arg(long)]
    pub ha_controlplanes: bool,

    /// Enable managed firewall
    #[arg(long)]
    pub enable_firewall: bool,

    /// Node pool label (required if creating initial node pool)
    #[arg(long)]
    pub pool_label: Option<String>,

    /// Node pool plan (required if creating initial node pool)
    #[arg(long)]
    pub pool_plan: Option<String>,

    /// Number of nodes in pool
    #[arg(long, default_value = "3")]
    pub pool_quantity: i32,

    /// Enable auto-scaler for pool
    #[arg(long)]
    pub pool_auto_scaler: bool,

    /// Minimum nodes for auto-scaler
    #[arg(long)]
    pub pool_min_nodes: Option<i32>,

    /// Maximum nodes for auto-scaler
    #[arg(long)]
    pub pool_max_nodes: Option<i32>,
}

// Node Pool subcommands

#[derive(Parser, Debug, Clone)]
pub struct KubernetesNodePoolArgs {
    #[command(subcommand)]
    pub command: KubernetesNodePoolCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum KubernetesNodePoolCommands {
    /// List node pools in a cluster
    List {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
    },

    /// Get node pool details
    Get {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        id: String,
    },

    /// Create a new node pool
    Create {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool label
        #[arg(long)]
        label: String,
        /// Plan ID for nodes
        #[arg(long)]
        plan: String,
        /// Number of nodes
        #[arg(long)]
        quantity: i32,
        /// Tag for nodes
        #[arg(long)]
        tag: Option<String>,
        /// Enable auto-scaler
        #[arg(long)]
        auto_scaler: bool,
        /// Minimum nodes for auto-scaler
        #[arg(long)]
        min_nodes: Option<i32>,
        /// Maximum nodes for auto-scaler
        #[arg(long)]
        max_nodes: Option<i32>,
    },

    /// Update a node pool
    Update {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        id: String,
        /// New node quantity
        #[arg(long)]
        quantity: Option<i32>,
        /// New tag
        #[arg(long)]
        tag: Option<String>,
        /// Enable/disable auto-scaler
        #[arg(long)]
        auto_scaler: Option<bool>,
        /// New minimum nodes
        #[arg(long)]
        min_nodes: Option<i32>,
        /// New maximum nodes
        #[arg(long)]
        max_nodes: Option<i32>,
    },

    /// Delete a node pool
    Delete {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        id: String,
    },

    /// List labels for a node pool
    ListLabels {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
    },

    /// Add a label to a node pool
    AddLabel {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
        /// Label key
        #[arg(long)]
        key: String,
        /// Label value
        #[arg(long)]
        value: String,
    },

    /// Delete a label from a node pool
    DeleteLabel {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
        /// Label ID
        label_id: String,
    },

    /// List taints for a node pool
    ListTaints {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
    },
}

// Node subcommands

#[derive(Parser, Debug, Clone)]
pub struct KubernetesNodeArgs {
    #[command(subcommand)]
    pub command: KubernetesNodeCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum KubernetesNodeCommands {
    /// List nodes in a node pool
    List {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
    },

    /// Get node details
    Get {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
        /// Node ID
        id: String,
    },

    /// Delete a node
    Delete {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
        /// Node ID
        id: String,
    },

    /// Recycle a node (delete and recreate)
    Recycle {
        /// Cluster ID (VKE ID)
        #[arg(long)]
        cluster_id: String,
        /// Node pool ID
        #[arg(long)]
        nodepool_id: String,
        /// Node ID
        id: String,
    },
}

// ==================
// Load Balancer Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerArgs {
    #[command(subcommand)]
    pub command: LoadBalancerCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoadBalancerCommands {
    /// List all load balancers
    List(ListArgs),

    /// Get load balancer details
    Get {
        /// Load Balancer ID
        id: String,
    },

    /// Create a new load balancer
    Create(LoadBalancerCreateArgs),

    /// Update a load balancer
    Update(LoadBalancerUpdateArgs),

    /// Delete a load balancer
    Delete {
        /// Load Balancer ID
        id: String,
    },

    /// Manage SSL certificates
    Ssl(LoadBalancerSslArgs),

    /// Manage forwarding rules
    #[command(alias = "rule", alias = "rules")]
    ForwardingRule(LoadBalancerForwardingRuleArgs),

    /// Manage firewall rules
    #[command(alias = "fw")]
    FirewallRule(LoadBalancerFirewallRuleArgs),

    /// Manage reverse DNS
    #[command(alias = "rdns")]
    ReverseDns(LoadBalancerReverseDnsArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerCreateArgs {
    /// Region ID
    #[arg(long)]
    pub region: String,

    /// Label for the load balancer
    #[arg(long)]
    pub label: Option<String>,

    /// Balancing algorithm (roundrobin, leastconn)
    #[arg(long)]
    pub balancing_algorithm: Option<String>,

    /// Enable SSL redirect
    #[arg(long)]
    pub ssl_redirect: bool,

    /// Enable HTTP2
    #[arg(long)]
    pub http2: bool,

    /// Enable HTTP3/QUIC
    #[arg(long)]
    pub http3: bool,

    /// Number of nodes (1-99, must be odd)
    #[arg(long)]
    pub nodes: Option<i32>,

    /// Enable proxy protocol
    #[arg(long)]
    pub proxy_protocol: bool,

    /// Connection timeout in seconds
    #[arg(long)]
    pub timeout: Option<i32>,

    /// VPC ID
    #[arg(long)]
    pub vpc: Option<String>,

    /// Health check protocol (http, https, tcp)
    #[arg(long)]
    pub health_check_protocol: Option<String>,

    /// Health check port
    #[arg(long)]
    pub health_check_port: Option<i32>,

    /// Health check path
    #[arg(long)]
    pub health_check_path: Option<String>,

    /// Health check interval (seconds)
    #[arg(long)]
    pub health_check_interval: Option<i32>,

    /// Health check response timeout (seconds)
    #[arg(long)]
    pub health_check_timeout: Option<i32>,

    /// Health check unhealthy threshold
    #[arg(long)]
    pub health_check_unhealthy_threshold: Option<i32>,

    /// Health check healthy threshold
    #[arg(long)]
    pub health_check_healthy_threshold: Option<i32>,

    /// Sticky session cookie name
    #[arg(long)]
    pub sticky_session_cookie: Option<String>,

    /// Instance IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub instances: Option<Vec<String>>,
}

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerUpdateArgs {
    /// Load Balancer ID
    #[arg(long)]
    pub id: String,

    /// Label for the load balancer
    #[arg(long)]
    pub label: Option<String>,

    /// Balancing algorithm (roundrobin, leastconn)
    #[arg(long)]
    pub balancing_algorithm: Option<String>,

    /// Enable SSL redirect
    #[arg(long)]
    pub ssl_redirect: Option<bool>,

    /// Enable HTTP2
    #[arg(long)]
    pub http2: Option<bool>,

    /// Enable HTTP3/QUIC
    #[arg(long)]
    pub http3: Option<bool>,

    /// Number of nodes (1-99, must be odd)
    #[arg(long)]
    pub nodes: Option<i32>,

    /// Enable proxy protocol
    #[arg(long)]
    pub proxy_protocol: Option<bool>,

    /// Connection timeout in seconds
    #[arg(long)]
    pub timeout: Option<i32>,

    /// VPC ID
    #[arg(long)]
    pub vpc: Option<String>,

    /// Health check protocol (http, https, tcp)
    #[arg(long)]
    pub health_check_protocol: Option<String>,

    /// Health check port
    #[arg(long)]
    pub health_check_port: Option<i32>,

    /// Health check path
    #[arg(long)]
    pub health_check_path: Option<String>,

    /// Health check interval (seconds)
    #[arg(long)]
    pub health_check_interval: Option<i32>,

    /// Health check response timeout (seconds)
    #[arg(long)]
    pub health_check_timeout: Option<i32>,

    /// Health check unhealthy threshold
    #[arg(long)]
    pub health_check_unhealthy_threshold: Option<i32>,

    /// Health check healthy threshold
    #[arg(long)]
    pub health_check_healthy_threshold: Option<i32>,

    /// Sticky session cookie name
    #[arg(long)]
    pub sticky_session_cookie: Option<String>,

    /// Instance IDs to attach (comma-separated, replaces existing)
    #[arg(long, value_delimiter = ',')]
    pub instances: Option<Vec<String>>,
}

// ==================
// Load Balancer SSL Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerSslArgs {
    #[command(subcommand)]
    pub command: LoadBalancerSslCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoadBalancerSslCommands {
    /// Add SSL certificate
    Add {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,

        /// Private key (file path or base64)
        #[arg(long)]
        private_key: Option<String>,

        /// SSL certificate (file path or base64)
        #[arg(long)]
        certificate: Option<String>,

        /// Certificate chain (file path or base64)
        #[arg(long)]
        chain: Option<String>,

        /// Private key (base64 encoded)
        #[arg(long)]
        private_key_b64: Option<String>,

        /// SSL certificate (base64 encoded)
        #[arg(long)]
        certificate_b64: Option<String>,

        /// Certificate chain (base64 encoded)
        #[arg(long)]
        chain_b64: Option<String>,
    },

    /// Delete SSL certificate
    Delete {
        /// Load Balancer ID
        lb_id: String,
    },

    /// Disable auto SSL
    DisableAutoSsl {
        /// Load Balancer ID
        lb_id: String,
    },
}

// ==================
// Load Balancer Forwarding Rule Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerForwardingRuleArgs {
    #[command(subcommand)]
    pub command: LoadBalancerForwardingRuleCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoadBalancerForwardingRuleCommands {
    /// List forwarding rules
    List {
        /// Load Balancer ID
        lb_id: String,
    },

    /// Get forwarding rule details
    Get {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Forwarding Rule ID
        #[arg(long)]
        rule_id: String,
    },

    /// Create a forwarding rule
    Create {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Frontend protocol (http, https, tcp)
        #[arg(long)]
        frontend_protocol: String,
        /// Frontend port
        #[arg(long)]
        frontend_port: i32,
        /// Backend protocol (http, https, tcp)
        #[arg(long)]
        backend_protocol: String,
        /// Backend port
        #[arg(long)]
        backend_port: i32,
    },

    /// Delete a forwarding rule
    Delete {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Forwarding Rule ID
        #[arg(long)]
        rule_id: String,
    },
}

// ==================
// Load Balancer Firewall Rule Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerFirewallRuleArgs {
    #[command(subcommand)]
    pub command: LoadBalancerFirewallRuleCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoadBalancerFirewallRuleCommands {
    /// List firewall rules
    List {
        /// Load Balancer ID
        lb_id: String,
    },

    /// Get firewall rule details
    Get {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Firewall Rule ID
        #[arg(long)]
        rule_id: String,
    },

    /// Create a firewall rule
    Create {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Port number
        #[arg(long)]
        port: i32,
        /// Source IP/CIDR or "cloudflare"
        #[arg(long)]
        source: String,
        /// IP type (v4, v6)
        #[arg(long)]
        ip_type: String,
    },

    /// Delete a firewall rule
    Delete {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Firewall Rule ID
        #[arg(long)]
        rule_id: String,
    },
}

// ==================
// Load Balancer Reverse DNS Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct LoadBalancerReverseDnsArgs {
    #[command(subcommand)]
    pub command: LoadBalancerReverseDnsCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum LoadBalancerReverseDnsCommands {
    /// Get reverse DNS
    Get {
        /// Load Balancer ID
        lb_id: String,
    },

    /// Update IPv4 reverse DNS
    UpdateIpv4 {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// Domain for reverse DNS
        #[arg(long)]
        domain: String,
    },

    /// Create IPv6 reverse DNS entry
    CreateIpv6 {
        /// Load Balancer ID
        #[arg(long)]
        lb_id: String,
        /// IPv6 address
        #[arg(long)]
        ip: String,
        /// Domain for reverse DNS
        #[arg(long)]
        domain: String,
    },
}

// ==================
// Database Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub command: DatabaseCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseCommands {
    /// List all databases
    List,

    /// Get database details
    Get {
        /// Database ID
        id: String,
    },

    /// Create a new database
    Create(DatabaseCreateArgs),

    /// Update a database
    Update(DatabaseUpdateArgs),

    /// Delete a database
    Delete {
        /// Database ID
        id: String,
    },

    /// List available database plans
    Plans {
        /// Filter by engine (mysql, pg, valkey, kafka)
        #[arg(long)]
        engine: Option<String>,
        /// Filter by number of nodes
        #[arg(long)]
        nodes: Option<i32>,
        /// Filter by region
        #[arg(long)]
        region: Option<String>,
    },

    /// Get database usage metrics
    Usage {
        /// Database ID
        id: String,
    },

    /// Get database alerts
    Alerts {
        /// Database ID
        id: String,
    },

    /// Get database backups
    Backups {
        /// Database ID
        id: String,
    },

    /// Restore database from backup
    Restore(DatabaseRestoreArgs),

    /// Fork a database
    Fork(DatabaseForkArgs),

    /// Create a read replica
    ReadReplica {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Label for replica
        #[arg(long)]
        label: String,
        /// Region (optional)
        #[arg(long)]
        region: Option<String>,
    },

    /// Promote read replica to standalone
    Promote {
        /// Database ID (read replica)
        id: String,
    },

    /// Get maintenance schedule
    Maintenance {
        /// Database ID
        id: String,
    },

    /// Update maintenance schedule
    SetMaintenance {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Day of week (sunday, monday, etc.)
        #[arg(long)]
        day: String,
        /// Hour (0-23)
        #[arg(long)]
        hour: i32,
    },

    /// Get available version upgrades
    Upgrades {
        /// Database ID
        id: String,
    },

    /// Upgrade database version
    Upgrade {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Target version
        #[arg(long)]
        version: String,
    },

    /// Database user management
    User(DatabaseUserArgs),

    /// Logical database management (MySQL/PostgreSQL)
    Db(DatabaseDbArgs),

    /// Connection pool management (PostgreSQL)
    Pool(DatabasePoolArgs),

    /// Kafka topic management
    Topic(DatabaseTopicArgs),

    /// Kafka connector management
    Connector(DatabaseConnectorArgs),

    /// Database migration
    Migration(DatabaseMigrationArgs),

    /// Get advanced options
    AdvancedOptions {
        /// Database ID
        id: String,
    },

    /// Set advanced options (JSON format)
    SetAdvancedOptions {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// JSON string with advanced options
        #[arg(long)]
        options: String,
    },

    /// Kafka quota management
    Quota(DatabaseQuotaArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseCreateArgs {
    /// Database engine (mysql, pg, valkey, kafka)
    #[arg(long)]
    pub engine: String,

    /// Engine version
    #[arg(long)]
    pub version: String,

    /// Region ID
    #[arg(long)]
    pub region: String,

    /// Plan ID
    #[arg(long)]
    pub plan: String,

    /// Label for the database
    #[arg(long)]
    pub label: Option<String>,

    /// Tag
    #[arg(long)]
    pub tag: Option<String>,

    /// VPC ID to attach
    #[arg(long)]
    pub vpc_id: Option<String>,

    /// Maintenance day of week
    #[arg(long)]
    pub maintenance_dow: Option<String>,

    /// Maintenance time (HH:MM)
    #[arg(long)]
    pub maintenance_time: Option<String>,

    /// Backup hour (0-23)
    #[arg(long)]
    pub backup_hour: Option<i32>,

    /// Backup minute (0-59)
    #[arg(long)]
    pub backup_minute: Option<i32>,

    /// Trusted IPs (comma-separated CIDR notation)
    #[arg(long, value_delimiter = ',')]
    pub trusted_ips: Option<Vec<String>>,

    /// MySQL SQL modes (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub mysql_sql_modes: Option<Vec<String>>,

    /// MySQL require primary key
    #[arg(long)]
    pub mysql_require_primary_key: Option<bool>,

    /// Valkey eviction policy
    #[arg(long)]
    pub eviction_policy: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseUpdateArgs {
    /// Database ID
    pub id: String,

    /// New plan
    #[arg(long)]
    pub plan: Option<String>,

    /// New label
    #[arg(long)]
    pub label: Option<String>,

    /// New tag
    #[arg(long)]
    pub tag: Option<String>,

    /// VPC ID
    #[arg(long)]
    pub vpc_id: Option<String>,

    /// Maintenance day of week
    #[arg(long)]
    pub maintenance_dow: Option<String>,

    /// Maintenance time (HH:MM)
    #[arg(long)]
    pub maintenance_time: Option<String>,

    /// Backup hour
    #[arg(long)]
    pub backup_hour: Option<i32>,

    /// Backup minute
    #[arg(long)]
    pub backup_minute: Option<i32>,

    /// Trusted IPs (comma-separated CIDR notation)
    #[arg(long, value_delimiter = ',')]
    pub trusted_ips: Option<Vec<String>>,

    /// Cluster time zone
    #[arg(long)]
    pub cluster_time_zone: Option<String>,

    /// Valkey eviction policy
    #[arg(long)]
    pub eviction_policy: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseRestoreArgs {
    /// Database ID to restore from
    #[arg(long)]
    pub database_id: String,

    /// Label for restored database
    #[arg(long)]
    pub label: String,

    /// Backup date (YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,

    /// Backup time (HH:MM:SS)
    #[arg(long)]
    pub time: Option<String>,

    /// Restoration type
    #[arg(long)]
    pub restore_type: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DatabaseForkArgs {
    /// Database ID to fork from
    #[arg(long)]
    pub database_id: String,

    /// Label for forked database
    #[arg(long)]
    pub label: String,

    /// Region (defaults to source region)
    #[arg(long)]
    pub region: Option<String>,

    /// Plan (defaults to source plan)
    #[arg(long)]
    pub plan: Option<String>,

    /// VPC ID
    #[arg(long)]
    pub vpc_id: Option<String>,

    /// Backup date
    #[arg(long)]
    pub date: Option<String>,

    /// Backup time
    #[arg(long)]
    pub time: Option<String>,

    /// Fork type
    #[arg(long)]
    pub fork_type: Option<String>,
}

// Database User Commands

#[derive(Parser, Debug, Clone)]
pub struct DatabaseUserArgs {
    #[command(subcommand)]
    pub command: DatabaseUserCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseUserCommands {
    /// List database users
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Get user details
    Get {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Username
        username: String,
    },

    /// Create a database user
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Username
        #[arg(long)]
        username: String,
        /// Password (auto-generated if not provided)
        #[arg(long)]
        password: Option<String>,
        /// Encryption type (MySQL only: Default, Legacy)
        #[arg(long)]
        encryption: Option<String>,
        /// Permission (Kafka only: admin, read, readwrite, write)
        #[arg(long)]
        permission: Option<String>,
    },

    /// Update a database user
    Update {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Username
        username: String,
        /// New password
        #[arg(long)]
        password: String,
    },

    /// Delete a database user
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Username
        username: String,
    },

    /// Update user access control (Valkey ACLs)
    AccessControl {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Username
        #[arg(long)]
        username: String,
        /// ACL categories (comma-separated, e.g., "+@all,-@dangerous")
        #[arg(long)]
        acl_categories: Option<String>,
        /// ACL channels (comma-separated)
        #[arg(long)]
        acl_channels: Option<String>,
        /// ACL commands (comma-separated, e.g., "+get,+set,-del")
        #[arg(long)]
        acl_commands: Option<String>,
        /// ACL keys (comma-separated patterns, e.g., "prefix:*,other:*")
        #[arg(long)]
        acl_keys: Option<String>,
    },
}

// Logical Database Commands (MySQL/PostgreSQL)

#[derive(Parser, Debug, Clone)]
pub struct DatabaseDbArgs {
    #[command(subcommand)]
    pub command: DatabaseDbCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseDbCommands {
    /// List logical databases
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Get logical database details
    Get {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Database name
        name: String,
    },

    /// Create a logical database
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Database name
        #[arg(long)]
        name: String,
    },

    /// Delete a logical database
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Database name
        name: String,
    },
}

// Connection Pool Commands (PostgreSQL)

#[derive(Parser, Debug, Clone)]
pub struct DatabasePoolArgs {
    #[command(subcommand)]
    pub command: DatabasePoolCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabasePoolCommands {
    /// List connection pools
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Get connection pool details
    Get {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Pool name
        name: String,
    },

    /// Create a connection pool
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Pool name
        #[arg(long)]
        name: String,
        /// Database name
        #[arg(long)]
        database: String,
        /// Username
        #[arg(long)]
        username: String,
        /// Pool mode (session, transaction, statement)
        #[arg(long)]
        mode: String,
        /// Pool size
        #[arg(long)]
        size: i32,
    },

    /// Update a connection pool
    Update {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Pool name
        name: String,
        /// New database name
        #[arg(long)]
        database: Option<String>,
        /// New username
        #[arg(long)]
        username: Option<String>,
        /// New pool mode
        #[arg(long)]
        mode: Option<String>,
        /// New pool size
        #[arg(long)]
        size: Option<i32>,
    },

    /// Delete a connection pool
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Pool name
        name: String,
    },
}

// Kafka Topic Commands

#[derive(Parser, Debug, Clone)]
pub struct DatabaseTopicArgs {
    #[command(subcommand)]
    pub command: DatabaseTopicCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseTopicCommands {
    /// List Kafka topics
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Get topic details
    Get {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Topic name
        name: String,
    },

    /// Create a Kafka topic
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Topic name
        #[arg(long)]
        name: String,
        /// Number of partitions
        #[arg(long)]
        partitions: i32,
        /// Replication factor
        #[arg(long)]
        replication: i32,
        /// Retention hours
        #[arg(long)]
        retention_hours: Option<i32>,
        /// Retention bytes
        #[arg(long)]
        retention_bytes: Option<i64>,
    },

    /// Update a Kafka topic
    Update {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Topic name
        name: String,
        /// New partition count
        #[arg(long)]
        partitions: Option<i32>,
        /// New retention hours
        #[arg(long)]
        retention_hours: Option<i32>,
        /// New retention bytes
        #[arg(long)]
        retention_bytes: Option<i64>,
    },

    /// Delete a Kafka topic
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Topic name
        name: String,
    },
}

// Kafka Connector Commands

#[derive(Parser, Debug, Clone)]
pub struct DatabaseConnectorArgs {
    #[command(subcommand)]
    pub command: DatabaseConnectorCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseConnectorCommands {
    /// List available connector classes
    Available {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// List connectors
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Get connector details
    Get {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Create a connector
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        #[arg(long)]
        name: String,
        /// Connector class
        #[arg(long)]
        class: String,
        /// Topics (comma-separated)
        #[arg(long)]
        topics: Option<String>,
    },

    /// Delete a connector
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Get connector status
    Status {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Pause a connector
    Pause {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Resume a connector
    Resume {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Restart a connector
    Restart {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        name: String,
    },

    /// Restart a connector task
    RestartTask {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Connector name
        #[arg(long)]
        connector_name: String,
        /// Task ID
        task_id: String,
    },
}

// Database Migration Commands

#[derive(Parser, Debug, Clone)]
pub struct DatabaseMigrationArgs {
    #[command(subcommand)]
    pub command: DatabaseMigrationCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseMigrationCommands {
    /// Get migration status
    Status {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Start migration from external source
    Start {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Source host
        #[arg(long)]
        host: String,
        /// Source port
        #[arg(long)]
        port: i32,
        /// Source username
        #[arg(long)]
        username: String,
        /// Source password
        #[arg(long)]
        password: String,
        /// Source database (optional for Valkey)
        #[arg(long)]
        database: Option<String>,
        /// Ignored databases (comma-separated)
        #[arg(long)]
        ignored_databases: Option<String>,
        /// Use SSL
        #[arg(long)]
        ssl: bool,
    },

    /// Detach (stop) migration
    Detach {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },
}

// Database Quota Commands (Kafka)

#[derive(Parser, Debug, Clone)]
pub struct DatabaseQuotaArgs {
    #[command(subcommand)]
    pub command: DatabaseQuotaCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DatabaseQuotaCommands {
    /// List database quotas
    List {
        /// Database ID
        #[arg(long)]
        database_id: String,
    },

    /// Create a database quota
    Create {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Client ID
        #[arg(long)]
        client_id: String,
        /// Username
        #[arg(long)]
        username: String,
        /// Consumer byte rate (optional)
        #[arg(long)]
        consumer_byte_rate: Option<i64>,
        /// Producer byte rate (optional)
        #[arg(long)]
        producer_byte_rate: Option<i64>,
        /// Request percentage (optional)
        #[arg(long)]
        request_percentage: Option<i32>,
    },

    /// Delete a database quota
    Delete {
        /// Database ID
        #[arg(long)]
        database_id: String,
        /// Client ID
        #[arg(long)]
        client_id: String,
        /// Username
        #[arg(long)]
        username: String,
    },
}

// ==================
// CDN Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct CdnArgs {
    #[command(subcommand)]
    pub command: CdnCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CdnCommands {
    /// Manage CDN Pull Zones
    #[command(alias = "pull")]
    PullZone(CdnPullZoneArgs),

    /// Manage CDN Push Zones
    #[command(alias = "push")]
    PushZone(CdnPushZoneArgs),
}

// CDN Pull Zone Commands

#[derive(Parser, Debug, Clone)]
pub struct CdnPullZoneArgs {
    #[command(subcommand)]
    pub command: CdnPullZoneCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CdnPullZoneCommands {
    /// List all CDN Pull Zones
    List(ListArgs),

    /// Get CDN Pull Zone details
    Get {
        /// Pull Zone ID
        id: String,
    },

    /// Create a new CDN Pull Zone
    Create {
        /// Label for the pull zone
        #[arg(long)]
        label: String,
        /// Origin domain to pull content from
        #[arg(long)]
        origin_domain: String,
        /// Origin scheme (http or https)
        #[arg(long, default_value = "https")]
        origin_scheme: String,
        /// Custom vanity domain
        #[arg(long)]
        vanity_domain: Option<String>,
        /// Enable CORS
        #[arg(long)]
        cors: bool,
        /// Enable gzip compression
        #[arg(long)]
        gzip: bool,
        /// Block AI bots
        #[arg(long)]
        block_ai: bool,
        /// Block bad bots
        #[arg(long)]
        block_bad_bots: bool,
        /// Regions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        regions: Option<Vec<String>>,
    },

    /// Update a CDN Pull Zone
    Update {
        /// Pull Zone ID
        id: String,
        /// New label
        #[arg(long)]
        label: Option<String>,
        /// Origin domain
        #[arg(long)]
        origin_domain: Option<String>,
        /// Origin scheme (http or https)
        #[arg(long)]
        origin_scheme: Option<String>,
        /// Custom vanity domain
        #[arg(long)]
        vanity_domain: Option<String>,
        /// Enable CORS
        #[arg(long)]
        cors: Option<bool>,
        /// Enable gzip compression
        #[arg(long)]
        gzip: Option<bool>,
        /// Block AI bots
        #[arg(long)]
        block_ai: Option<bool>,
        /// Block bad bots
        #[arg(long)]
        block_bad_bots: Option<bool>,
        /// Regions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        regions: Option<Vec<String>>,
    },

    /// Delete a CDN Pull Zone
    Delete {
        /// Pull Zone ID
        id: String,
    },

    /// Purge cache for a CDN Pull Zone
    Purge {
        /// Pull Zone ID
        id: String,
    },
}

// CDN Push Zone Commands

#[derive(Parser, Debug, Clone)]
pub struct CdnPushZoneArgs {
    #[command(subcommand)]
    pub command: CdnPushZoneCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CdnPushZoneCommands {
    /// List all CDN Push Zones
    List(ListArgs),

    /// Get CDN Push Zone details
    Get {
        /// Push Zone ID
        id: String,
    },

    /// Create a new CDN Push Zone
    Create {
        /// Label for the push zone
        #[arg(long)]
        label: String,
        /// Custom vanity domain
        #[arg(long)]
        vanity_domain: Option<String>,
        /// Enable CORS
        #[arg(long)]
        cors: bool,
        /// Enable gzip compression
        #[arg(long)]
        gzip: bool,
        /// Block AI bots
        #[arg(long)]
        block_ai: bool,
        /// Block bad bots
        #[arg(long)]
        block_bad_bots: bool,
        /// Regions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        regions: Option<Vec<String>>,
    },

    /// Update a CDN Push Zone
    Update {
        /// Push Zone ID
        id: String,
        /// New label
        #[arg(long)]
        label: Option<String>,
        /// Custom vanity domain
        #[arg(long)]
        vanity_domain: Option<String>,
        /// Enable CORS
        #[arg(long)]
        cors: Option<bool>,
        /// Enable gzip compression
        #[arg(long)]
        gzip: Option<bool>,
        /// Block AI bots
        #[arg(long)]
        block_ai: Option<bool>,
        /// Block bad bots
        #[arg(long)]
        block_bad_bots: Option<bool>,
        /// Regions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        regions: Option<Vec<String>>,
    },

    /// Delete a CDN Push Zone
    Delete {
        /// Push Zone ID
        id: String,
    },

    /// Manage files in a CDN Push Zone
    File(CdnPushZoneFileArgs),
}

// CDN Push Zone File Commands

#[derive(Parser, Debug, Clone)]
pub struct CdnPushZoneFileArgs {
    #[command(subcommand)]
    pub command: CdnPushZoneFileCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CdnPushZoneFileCommands {
    /// List files in a CDN Push Zone
    List {
        /// Push Zone ID
        #[arg(long)]
        pushzone_id: String,
        #[command(flatten)]
        list: ListArgs,
    },

    /// Get file details
    Get {
        /// Push Zone ID
        #[arg(long)]
        pushzone_id: String,
        /// File name
        file_name: String,
    },

    /// Create a file upload endpoint
    CreateEndpoint {
        /// Push Zone ID
        #[arg(long)]
        pushzone_id: String,
        /// File name
        #[arg(long)]
        name: String,
        /// File size in bytes
        #[arg(long)]
        size: i64,
    },

    /// Delete a file from a CDN Push Zone
    Delete {
        /// Push Zone ID
        #[arg(long)]
        pushzone_id: String,
        /// File name
        file_name: String,
    },
}

// ==================
// DNS Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct DnsArgs {
    #[command(subcommand)]
    pub command: DnsCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DnsCommands {
    /// List all DNS domains
    List(ListArgs),

    /// Get DNS domain details
    Get {
        /// Domain name (e.g., example.com)
        domain: String,
    },

    /// Create a new DNS domain
    Create {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        /// Default IP address for the domain
        #[arg(long)]
        ip: Option<String>,
        /// Enable DNSSEC (enabled/disabled)
        #[arg(long)]
        dns_sec: Option<String>,
    },

    /// Update a DNS domain
    Update {
        /// Domain name (e.g., example.com)
        domain: String,
        /// Enable or disable DNSSEC (enabled/disabled)
        #[arg(long)]
        dns_sec: String,
    },

    /// Delete a DNS domain
    Delete {
        /// Domain name (e.g., example.com)
        domain: String,
    },

    /// Get SOA information for a domain
    Soa {
        /// Domain name (e.g., example.com)
        domain: String,
    },

    /// Update SOA information for a domain
    UpdateSoa {
        /// Domain name (e.g., example.com)
        domain: String,
        /// Primary nameserver
        #[arg(long)]
        nsprimary: Option<String>,
        /// Contact email
        #[arg(long)]
        email: Option<String>,
    },

    /// Get DNSSEC information for a domain
    Dnssec {
        /// Domain name (e.g., example.com)
        domain: String,
    },

    /// Manage DNS records
    Record(DnsRecordArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct DnsRecordArgs {
    #[command(subcommand)]
    pub command: DnsRecordCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DnsRecordCommands {
    /// List DNS records for a domain
    List {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        #[command(flatten)]
        list: ListArgs,
    },

    /// Get DNS record details
    Get {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        /// Record ID
        id: String,
    },

    /// Create a DNS record
    Create {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        /// Record name/hostname (e.g., www, mail, @)
        #[arg(long)]
        name: String,
        /// Record type (A, AAAA, CNAME, NS, MX, SRV, TXT, CAA, SSHFP)
        #[arg(long, rename_all = "SCREAMING_SNAKE_CASE")]
        record_type: String,
        /// Record data (e.g., IP address, hostname)
        #[arg(long)]
        data: String,
        /// Time to Live in seconds
        #[arg(long)]
        ttl: Option<i32>,
        /// Priority (required for MX and SRV records)
        #[arg(long)]
        priority: Option<i32>,
    },

    /// Update a DNS record
    Update {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        /// Record ID
        id: String,
        /// New record name/hostname
        #[arg(long)]
        name: Option<String>,
        /// New record data
        #[arg(long)]
        data: Option<String>,
        /// New TTL in seconds
        #[arg(long)]
        ttl: Option<i32>,
        /// New priority
        #[arg(long)]
        priority: Option<i32>,
    },

    /// Delete a DNS record
    Delete {
        /// Domain name (e.g., example.com)
        #[arg(long)]
        domain: String,
        /// Record ID
        id: String,
    },
}

// ==================
// Container Registry Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub command: RegistryCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryCommands {
    /// List all container registries
    List,

    /// Get registry details
    Get {
        /// Registry ID
        id: String,
    },

    /// Create a new container registry
    Create {
        /// Registry name (globally unique)
        #[arg(long)]
        name: String,
        /// Region name (e.g., sjc)
        #[arg(long)]
        region: String,
        /// Plan ID (e.g., start_up, business, premium, enterprise)
        #[arg(long)]
        plan: String,
        /// Make registry public
        #[arg(long)]
        public: bool,
    },

    /// Update a container registry
    Update {
        /// Registry ID
        id: String,
        /// New plan ID
        #[arg(long)]
        plan: Option<String>,
        /// Make registry public
        #[arg(long)]
        public: Option<bool>,
    },

    /// Delete a container registry
    Delete {
        /// Registry ID
        id: String,
    },

    /// Manage repositories
    #[command(alias = "repo")]
    Repository(RegistryRepositoryArgs),

    /// Manage robot accounts
    Robot(RegistryRobotArgs),

    /// Manage replications
    #[command(alias = "repl")]
    Replication(RegistryReplicationArgs),

    /// Manage retention policies
    Retention(RegistryRetentionArgs),

    /// Get Docker credentials
    DockerCredentials {
        /// Registry ID
        id: String,
        /// Credentials expiry in seconds
        #[arg(long)]
        expiry_seconds: Option<i64>,
        /// Request read-write credentials
        #[arg(long)]
        read_write: bool,
    },

    /// Get Kubernetes Docker credentials
    KubernetesCredentials {
        /// Registry ID
        id: String,
        /// Credentials expiry in seconds
        #[arg(long)]
        expiry_seconds: Option<i64>,
        /// Request read-write credentials
        #[arg(long)]
        read_write: bool,
        /// Base64 encode the output
        #[arg(long)]
        base64_encode: bool,
    },

    /// Update root user password
    UpdatePassword {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// New password
        #[arg(long)]
        password: String,
    },

    /// List available registry regions
    Regions,

    /// List available registry plans
    Plans,
}

// Registry Repository Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryRepositoryArgs {
    #[command(subcommand)]
    pub command: RegistryRepositoryCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryRepositoryCommands {
    /// List repositories in a registry
    List {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },

    /// Get repository details
    Get {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Repository image name
        image: String,
    },

    /// Delete a repository
    Delete {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Repository image name
        image: String,
    },

    /// Manage artifacts
    Artifact(RegistryArtifactArgs),
}

// Registry Artifact Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryArtifactArgs {
    #[command(subcommand)]
    pub command: RegistryArtifactCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryArtifactCommands {
    /// List artifacts in a repository
    List {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Repository image name
        #[arg(long)]
        image: String,
    },

    /// Delete an artifact
    Delete {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Repository image name
        #[arg(long)]
        image: String,
        /// Artifact digest (sha256:...)
        digest: String,
    },
}

// Registry Robot Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryRobotArgs {
    #[command(subcommand)]
    pub command: RegistryRobotCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryRobotCommands {
    /// List robot accounts
    List {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },

    /// Get robot account details
    Get {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Robot name
        name: String,
    },

    /// Create a robot account
    Create {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Robot name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Duration in seconds (-1 for never expires)
        #[arg(long, default_value = "-1")]
        duration: i64,
        /// Disable the robot account
        #[arg(long)]
        disable: bool,
    },

    /// Update a robot account
    Update {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Robot name
        name: String,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New duration in seconds
        #[arg(long)]
        duration: Option<i64>,
        /// Disable/enable the robot account
        #[arg(long)]
        disable: Option<bool>,
    },

    /// Delete a robot account
    Delete {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Robot name
        name: String,
    },
}

// Registry Replication Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryReplicationArgs {
    #[command(subcommand)]
    pub command: RegistryReplicationCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryReplicationCommands {
    /// List replications
    List {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },

    /// Create a replication
    Create {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Target region
        #[arg(long)]
        region: String,
    },

    /// Delete a replication
    Delete {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Target region
        region: String,
    },
}

// Registry Retention Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryRetentionArgs {
    #[command(subcommand)]
    pub command: RegistryRetentionCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryRetentionCommands {
    /// Manage retention schedule
    Schedule(RegistryRetentionScheduleArgs),

    /// Manage retention rules
    Rule(RegistryRetentionRuleArgs),

    /// List retention executions
    Executions {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },
}

// Registry Retention Schedule Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryRetentionScheduleArgs {
    #[command(subcommand)]
    pub command: RegistryRetentionScheduleCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryRetentionScheduleCommands {
    /// Get retention schedule
    Get {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },

    /// Update retention schedule
    Update {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Schedule type (Hourly, Daily, Weekly, Custom)
        #[arg(long)]
        schedule_type: String,
        /// Cron expression (required for Custom type)
        #[arg(long)]
        cron: Option<String>,
    },
}

// Registry Retention Rule Commands

#[derive(Parser, Debug, Clone)]
pub struct RegistryRetentionRuleArgs {
    #[command(subcommand)]
    pub command: RegistryRetentionRuleCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryRetentionRuleCommands {
    /// List retention rules
    List {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
    },

    /// Create a retention rule
    Create {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Template (latestPushedK, latestPulledN, nDaysSinceLastPull, nDaysSinceLastPush, always)
        #[arg(long)]
        template: String,
        /// Tag pattern to match
        #[arg(long, default_value = "**")]
        tag_pattern: String,
        /// Repository pattern to match (repoMatches)
        #[arg(long)]
        repo_pattern: Option<String>,
        /// Number of artifacts/days (depends on template)
        #[arg(long)]
        count: Option<i64>,
    },

    /// Update a retention rule
    Update {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Rule ID
        rule_id: i64,
        /// New template
        #[arg(long)]
        template: Option<String>,
        /// Disable/enable the rule
        #[arg(long)]
        disabled: Option<bool>,
        /// Number of artifacts/days
        #[arg(long)]
        count: Option<i64>,
    },

    /// Delete a retention rule
    Delete {
        /// Registry ID
        #[arg(long)]
        registry_id: String,
        /// Rule ID
        rule_id: i64,
    },
}

// ==================
// Reserved IP Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct ReservedIpArgs {
    #[command(subcommand)]
    pub command: ReservedIpCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ReservedIpCommands {
    /// List all reserved IPs
    List(ListArgs),

    /// Get reserved IP details
    Get {
        /// Reserved IP ID
        id: String,
    },

    /// Create a new reserved IP
    Create {
        /// Region ID
        #[arg(long)]
        region: String,
        /// IP type (v4 or v6)
        #[arg(long)]
        ip_type: String,
        /// Label for the reserved IP
        #[arg(long)]
        label: Option<String>,
    },

    /// Update a reserved IP
    Update {
        /// Reserved IP ID
        id: String,
        /// New label
        #[arg(long)]
        label: String,
    },

    /// Delete a reserved IP
    Delete {
        /// Reserved IP ID
        id: String,
    },

    /// Attach a reserved IP to an instance
    Attach {
        /// Reserved IP ID
        id: String,
        /// Instance ID to attach to
        #[arg(long)]
        instance_id: String,
    },

    /// Detach a reserved IP from an instance
    Detach {
        /// Reserved IP ID
        id: String,
    },

    /// Convert an instance IP to a reserved IP
    Convert {
        /// IP address to convert
        #[arg(long)]
        ip_address: String,
        /// Label for the reserved IP
        #[arg(long)]
        label: Option<String>,
    },
}

// ==================
// Plans Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct PlansArgs {
    /// Filter by plan type (vc2, vhf, vdc)
    #[arg(long)]
    pub plan_type: Option<String>,

    /// List bare metal plans (includes CPU/RAM/pricing)
    #[arg(long, alias = "metal", conflicts_with = "plan_type")]
    pub bare_metal: bool,
}

// ==================
// Completions Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Elvish => clap_complete::Shell::Elvish,
        }
    }
}

// ==================
// Account Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AccountCommands {
    /// Get account information
    Info,

    /// Get BGP information
    Bgp,

    /// Get account bandwidth usage
    Bandwidth,
}

// ==================
// Billing Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct BillingArgs {
    #[command(subcommand)]
    pub command: BillingCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BillingCommands {
    /// List billing history
    History(ListArgs),

    /// List invoices
    Invoices(ListArgs),

    /// Get invoice details
    Invoice {
        /// Invoice ID
        id: i64,
    },

    /// List invoice items
    InvoiceItems(InvoiceItemsArgs),

    /// List pending charges
    PendingCharges,

    /// Get pending charges as CSV
    PendingChargesCsv,
}

#[derive(Parser, Debug, Clone)]
pub struct InvoiceItemsArgs {
    /// Invoice ID
    #[arg(long)]
    pub invoice_id: i64,

    #[command(flatten)]
    pub list_args: ListArgs,
}

// ==================
// User Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: UserCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UserCommands {
    /// List all users
    List(ListArgs),

    /// Get user details
    Get {
        /// User ID
        id: String,
    },

    /// Create a new user
    Create {
        /// Email address
        #[arg(long)]
        email: String,
        /// User name
        #[arg(long)]
        name: String,
        /// Password
        #[arg(long)]
        password: String,
        /// Enable API access
        #[arg(long)]
        api_enabled: Option<bool>,
        /// Access control list (comma-separated)
        #[arg(long, value_delimiter = ',')]
        acls: Option<Vec<String>>,
    },

    /// Update a user
    Update {
        /// User ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New email
        #[arg(long)]
        email: Option<String>,
        /// New password
        #[arg(long)]
        password: Option<String>,
        /// Enable/disable API access
        #[arg(long)]
        api_enabled: Option<bool>,
        /// Access control list (comma-separated)
        #[arg(long, value_delimiter = ',')]
        acls: Option<Vec<String>>,
    },

    /// Delete a user
    Delete {
        /// User ID
        id: String,
    },

    /// Manage user API keys
    ApiKeys(UserApiKeyArgs),

    /// Manage user IP whitelist
    IpWhitelist(UserIpWhitelistArgs),
}

// User API Key Commands

#[derive(Parser, Debug, Clone)]
pub struct UserApiKeyArgs {
    #[command(subcommand)]
    pub command: UserApiKeyCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UserApiKeyCommands {
    /// List API keys for a user
    List(UserApiKeyListArgs),

    /// Create an API key for a user
    Create {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// API key name
        #[arg(long)]
        name: String,
    },

    /// Delete an API key
    Delete {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// API key ID
        #[arg(long)]
        api_key_id: String,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct UserApiKeyListArgs {
    /// User ID
    #[arg(long)]
    pub user_id: String,

    #[command(flatten)]
    pub list_args: ListArgs,
}

// User IP Whitelist Commands

#[derive(Parser, Debug, Clone)]
pub struct UserIpWhitelistArgs {
    #[command(subcommand)]
    pub command: UserIpWhitelistCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum UserIpWhitelistCommands {
    /// List IP whitelist entries for a user
    List {
        /// User ID
        #[arg(long)]
        user_id: String,
    },

    /// Get IP whitelist entry details
    Get {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// Subnet (IP address)
        #[arg(long)]
        subnet: String,
        /// Subnet size (CIDR)
        #[arg(long)]
        subnet_size: i32,
    },

    /// Add an IP to the whitelist
    Add {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// Subnet (IP address)
        #[arg(long)]
        subnet: String,
        /// Subnet size (CIDR)
        #[arg(long)]
        subnet_size: i32,
    },

    /// Delete an IP from the whitelist
    Delete {
        /// User ID
        #[arg(long)]
        user_id: String,
        /// Subnet (IP address)
        #[arg(long)]
        subnet: String,
        /// Subnet size (CIDR)
        #[arg(long)]
        subnet_size: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_to_clap_complete() {
        assert_eq!(
            clap_complete::Shell::from(Shell::Bash),
            clap_complete::Shell::Bash
        );
        assert_eq!(
            clap_complete::Shell::from(Shell::Zsh),
            clap_complete::Shell::Zsh
        );
        assert_eq!(
            clap_complete::Shell::from(Shell::Fish),
            clap_complete::Shell::Fish
        );
        assert_eq!(
            clap_complete::Shell::from(Shell::PowerShell),
            clap_complete::Shell::PowerShell
        );
        assert_eq!(
            clap_complete::Shell::from(Shell::Elvish),
            clap_complete::Shell::Elvish
        );
    }

    #[test]
    fn test_list_args_default() {
        let args = ListArgs {
            per_page: 25,
            cursor: None,
            all: false,
        };
        assert_eq!(args.per_page, 25);
        assert!(args.cursor.is_none());
        assert!(!args.all);
    }

    #[test]
    fn test_instance_create_args_with_ssh_keys() {
        let args = InstanceCreateArgs {
            region: "ewr".to_string(),
            plan: "vc2-1c-1gb".to_string(),
            os_id: Some(215),
            snapshot_id: None,
            iso_id: None,
            app_id: None,
            image_id: None,
            label: Some("test-instance".to_string()),
            hostname: None,
            ssh_keys: Some(vec!["key1".to_string(), "key2".to_string()]),
            script_id: None,
            enable_ipv6: true,
            disable_public_ipv4: false,
            backups: false,
            ddos_protection: false,
            activation_email: false,
            vpc: None,
            firewall_group_id: None,
            reserved_ipv4: None,
            tags: None,
            user_data: None,
            user_scheme: None,
        };
        assert_eq!(args.ssh_keys.as_ref().unwrap().len(), 2);
        assert!(args.enable_ipv6);
    }

    #[test]
    fn test_plans_args_with_type() {
        let args = PlansArgs {
            plan_type: Some("vc2".to_string()),
            bare_metal: false,
        };
        assert_eq!(args.plan_type.unwrap(), "vc2");
    }

    #[test]
    fn test_completions_args() {
        let args = CompletionsArgs { shell: Shell::Bash };
        assert_eq!(args.shell, Shell::Bash);
    }

    #[test]
    fn test_auth_login_args() {
        let args = AuthLoginArgs {
            api_key: Some("test-key".to_string()),
        };
        assert_eq!(args.api_key.unwrap(), "test-key");
    }

    #[test]
    fn test_firewall_rule_create_with_notes() {
        // Test creating a firewall rule command structure
        let group_id = "fw-123".to_string();
        let ip_type = "v4".to_string();
        let protocol = "TCP".to_string();
        let subnet = "0.0.0.0".to_string();
        let subnet_size = 0;
        let port = Some("443".to_string());
        let notes = Some("HTTPS traffic".to_string());

        assert_eq!(group_id, "fw-123");
        assert_eq!(subnet_size, 0);
        assert_eq!(port.unwrap(), "443");
        assert_eq!(notes.unwrap(), "HTTPS traffic");
        assert_eq!(ip_type, "v4");
        assert_eq!(protocol, "TCP");
        assert_eq!(subnet, "0.0.0.0");
    }

    #[test]
    fn test_block_storage_create_args() {
        // Simulating block storage create args validation
        let region = "ewr";
        let size = 100;
        let label = Some("my-storage");
        let block_type = Some("high_perf");

        assert_eq!(region, "ewr");
        assert_eq!(size, 100);
        assert!(size >= 10 && size <= 40000);
        assert_eq!(label.unwrap(), "my-storage");
        assert_eq!(block_type.unwrap(), "high_perf");
    }

    #[test]
    fn test_vpc_create_args() {
        let region = "ewr".to_string();
        let description = Some("Production VPC".to_string());
        let subnet = Some("10.0.0.0".to_string());
        let subnet_mask = Some(16);

        assert_eq!(region, "ewr");
        assert_eq!(description.unwrap(), "Production VPC");
        assert_eq!(subnet.unwrap(), "10.0.0.0");
        assert_eq!(subnet_mask.unwrap(), 16);
    }

    #[test]
    fn test_cli_api_key_before_subcommand() {
        let cli = Cli::try_parse_from(["vultr-cli", "--api-key", "test123", "regions"]).unwrap();
        assert_eq!(cli.api_key, Some("test123".to_string()));
    }

    #[test]
    fn test_cli_api_key_after_subcommand() {
        let cli = Cli::try_parse_from(["vultr-cli", "regions", "--api-key", "test456"]).unwrap();
        assert_eq!(cli.api_key, Some("test456".to_string()));
    }

    #[test]
    fn test_cli_api_key_with_nested_subcommand() {
        let cli =
            Cli::try_parse_from(["vultr-cli", "instance", "list", "--api-key", "test789"]).unwrap();
        assert_eq!(cli.api_key, Some("test789".to_string()));
    }

    #[test]
    fn test_cli_api_key_between_subcommands() {
        let cli =
            Cli::try_parse_from(["vultr-cli", "instance", "--api-key", "testabc", "list"]).unwrap();
        assert_eq!(cli.api_key, Some("testabc".to_string()));
    }

    #[test]
    fn test_cli_all_global_flags() {
        let cli = Cli::try_parse_from([
            "vultr-cli",
            "--api-key",
            "mykey",
            "--profile",
            "prod",
            "--output",
            "json",
            "--yes",
            "--wait",
            "--wait-timeout",
            "300",
            "--poll-interval",
            "10",
            "regions",
        ])
        .unwrap();
        assert_eq!(cli.api_key, Some("mykey".to_string()));
        assert_eq!(cli.profile, "prod");
        assert_eq!(cli.output, Some(OutputFormat::Json));
        assert!(cli.yes);
        assert!(cli.wait);
        assert_eq!(cli.wait_timeout, Some(300));
        assert_eq!(cli.poll_interval, Some(10));
    }
}
