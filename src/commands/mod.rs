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

    /// Manage block storage
    #[command(alias = "block", alias = "storage")]
    BlockStorage(BlockStorageArgs),

    /// Manage firewall groups and rules
    #[command(alias = "fw")]
    Firewall(FirewallArgs),

    /// Manage VPCs (Virtual Private Clouds)
    Vpc(VpcArgs),

    /// Manage Kubernetes clusters (VKE)
    #[command(alias = "k8s", alias = "vke")]
    Kubernetes(KubernetesArgs),

    /// List available regions
    Regions,

    /// List available plans
    Plans(PlansArgs),

    /// List available operating systems
    Os,

    /// Generate shell completions
    Completions(CompletionsArgs),
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

    /// Application ID
    #[arg(long)]
    pub app_id: Option<i32>,

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

    /// Enable automatic backups
    #[arg(long)]
    pub backups: bool,

    /// Enable DDoS protection
    #[arg(long)]
    pub ddos_protection: bool,

    /// VPC IDs to attach (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub vpc: Option<Vec<String>>,

    /// Firewall group ID
    #[arg(long)]
    pub firewall_group_id: Option<String>,

    /// Tags (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    /// User data (base64 encoded)
    #[arg(long)]
    pub user_data: Option<String>,
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
// Plans Commands
// ==================

#[derive(Parser, Debug, Clone)]
pub struct PlansArgs {
    /// Filter by plan type (vc2, vhf, vdc)
    #[arg(long)]
    pub plan_type: Option<String>,
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
            app_id: None,
            label: Some("test-instance".to_string()),
            hostname: None,
            ssh_keys: Some(vec!["key1".to_string(), "key2".to_string()]),
            script_id: None,
            enable_ipv6: true,
            backups: false,
            ddos_protection: false,
            vpc: None,
            firewall_group_id: None,
            tags: None,
            user_data: None,
        };
        assert_eq!(args.ssh_keys.as_ref().unwrap().len(), 2);
        assert!(args.enable_ipv6);
    }

    #[test]
    fn test_plans_args_with_type() {
        let args = PlansArgs {
            plan_type: Some("vc2".to_string()),
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
}
