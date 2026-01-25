# Vultr CLI

A production-ready command-line interface for managing Vultr cloud resources, built in Rust.

## Features

- **Full Resource Management**: SSH keys, startup scripts, instances (VMs), snapshots, block storage, firewalls, and VPCs
- **Multiple Output Formats**: Table (default) and JSON
- **Secure Authentication**: API keys stored in system keyring (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **Wait Flag**: `--wait` to poll until async operations complete
- **Pagination**: `--all` to fetch all pages for list commands
- **Deletion Verification**: Confirms resources are actually deleted
- **Retries/Backoff**: Automatic retry with bounded exponential backoff for transient API/network errors (429/5xx/timeouts)
- **Shell Completions**: Bash, Zsh, Fish, PowerShell, Elvish
- **Cross-Platform**: Linux, macOS, Windows

## Installation

```bash
# From source
cargo install --path .

# Or build directly
cargo build --release
```

## Authentication

```bash
# Interactive login (stores key securely in system keyring)
vultr auth login

# Or use environment variable
export VULTR_API_KEY="your-api-key"

# Or pass directly (not recommended for scripts)
vultr --api-key "your-key" instance list

# Check auth status
vultr auth status

# Logout (remove stored key)
vultr auth logout
```

## Usage Examples

### Instances (VMs)

```bash
# List all instances
vultr instance list

# Create an instance
vultr instance create \
  --region ewr \
  --plan vc2-1c-1gb \
  --os-id 387 \
  --label "my-server" \
  --ssh-keys key-id-1,key-id-2 \
  --wait

# Other create options
# - Use one of: --os-id, --iso-id, --snapshot-id, --app-id, --image-id
# - Optional: --disable-public-ipv4, --activation-email, --reserved-ipv4, --user-scheme

# Get instance details
vultr instance get <instance-id>

# Start/stop/reboot
vultr instance start <instance-id> --wait
vultr instance stop <instance-id> --wait
vultr instance reboot <instance-id>

# Delete (with confirmation)
vultr instance delete <instance-id>

# Delete without confirmation, wait for deletion
vultr instance delete <instance-id> -y --wait
```

### SSH Keys

```bash
# List SSH keys
vultr ssh-key list

# Create from string
vultr ssh-key create --name "my-key" --key "ssh-rsa AAAA..."

# Create from file
vultr ssh-key create --name "my-key" --key @~/.ssh/id_rsa.pub

# Delete
vultr ssh-key delete <key-id>
```

### Startup Scripts

```bash
# List scripts
vultr startup-script list

# Create from file
vultr startup-script create --name "setup" --script @./setup.sh --script-type boot

# Get script (shows decoded content)
vultr startup-script get <script-id>
```

### Snapshots

```bash
# List snapshots
vultr snapshot list

# Create from instance
vultr snapshot create --instance-id <id> --description "Before upgrade" --wait

# Create from URL
vultr snapshot create-from-url --url "https://..." --description "Custom image"

# Delete
vultr snapshot delete <snapshot-id> --wait
```

### Block Storage

```bash
# List volumes
vultr block-storage list

# Create
vultr block-storage create --region ewr --size 100 --label "data-vol" --wait

# Attach to instance (live)
vultr block-storage attach <block-id> --instance-id <instance-id> --live

# Detach
vultr block-storage detach <block-id> --live

# Delete
vultr block-storage delete <block-id> --wait
```

### Firewalls

```bash
# List firewall groups
vultr firewall group list

# Create group
vultr firewall group create --description "Web servers"

# Add rule (allow SSH from anywhere)
vultr firewall rule create \
  --group-id <group-id> \
  --ip-type v4 \
  --protocol TCP \
  --subnet 0.0.0.0 \
  --subnet-size 0 \
  --port 22 \
  --notes "SSH access"

# Add rule (allow HTTP)
vultr firewall rule create \
  --group-id <group-id> \
  --ip-type v4 \
  --protocol TCP \
  --subnet 0.0.0.0 \
  --subnet-size 0 \
  --port 80 \
  --notes "HTTP"

# List rules
vultr firewall rule list --group-id <group-id>
```

### VPCs

```bash
# List VPCs
vultr vpc list

# Create
vultr vpc create --region ewr --description "Production" --subnet 10.0.0.0 --subnet-mask 24

# Delete
vultr vpc delete <vpc-id>

# List VPC attachments
vultr vpc attachments <vpc-id>
```

### Private Networks (Legacy)

```bash
# List private networks
vultr private-network list
vultr pnet list
vultr privnet list

# Create
vultr private-network create --region ewr --description "Legacy net" --subnet 10.10.0.0 --subnet-mask 24
```

### Applications (Marketplace Variables)

```bash
# List applications
vultr applications list

# Show required variables for a marketplace app
vultr applications variables --image-id <image-id>
```

### Inference

```bash
# List subscriptions
vultr inference list

# Create a subscription
vultr inference create --label "prod"

# Usage
vultr inference usage <subscription-id>
```

### Logs

```bash
# Filter logs by time window and resource
vultr logs --start-time 2024-01-01T00:00:00Z --end-time 2024-01-01T01:00:00Z --resource-type instance

# Continue from a prior response
vultr logs --continue-time 2024-01-01T01:00:00Z
```

### Subaccounts

```bash
# List subaccounts
vultr subaccount list

# Create a subaccount
vultr subaccount create --email "sub@acme.co" --name "Acme Widgets LLC"
```

### Storage Gateways

```bash
# List storage gateways
vultr storage-gateway list

# Create a storage gateway
vultr storage-gateway create \
  --label "my_storage_gateway" \
  --type nfs4 \
  --region ewr \
  --export-label "my_export_1" \
  --export-vfs-uuid <vfs-uuid> \
  --export-pseudo-root-path "/" \
  --export-allowed-ips 192.0.2.123 \
  --ipv4-public-enabled true

# Add an export
vultr storage-gateway export add --gateway-id <id> --label "export2" --vfs-uuid <vfs-uuid>
```

### VFS

```bash
# List VFS
vultr vfs list

# Create VFS
vultr vfs create --region ewr --label "prod-vfs" --size-gb 100 --disk-type nvme

# Attach a VPS to VFS
vultr vfs attachment attach --vfs-id <vfs-id> --vps-id <instance-id>
```

### Databases (Advanced Options / Kafka)

```bash
# Get advanced options
vultr database advanced-options <db-id>

# Set Kafka REST advanced options
vultr database set-advanced-options-kafka-rest --database-id <db-id> --options '{"producer_acks":"1"}'

# Kafka user permissions
vultr database user permissions --database-id <db-id> --username <user> --permission read

# Connector configuration schema
vultr database connector config-schema --database-id <db-id> --connector-class <class>
```

### Reference Data

```bash
# List regions
vultr regions

# List plans (optionally filter by type)
vultr plans
vultr plans --plan-type vc2
vultr plans --region ewr
vultr plans --bare-metal
vultr plans --bare-metal --price monthly
vultr plans --bare-metal --region ewr

# List operating systems
vultr os
vultr os --family ubuntu
vultr os --name "Ubuntu 22.04"
```

## Global Options

```
--api-key <KEY>      API key (or use VULTR_API_KEY env var)
--profile <NAME>     Config profile to use (default: "default")
--output, -o <FMT>   Output format: table, json (default: table)
--yes, -y            Skip confirmation prompts
--wait, -w           Wait for async operations to complete
--wait-timeout <SEC> Timeout for wait operations (default: 600)
```

## Shell Completions

```bash
# Bash
vultr completions bash > /etc/bash_completion.d/vultr

# Zsh
vultr completions zsh > ~/.zfunc/_vultr

# Fish
vultr completions fish > ~/.config/fish/completions/vultr.fish

# PowerShell
vultr completions powershell > vultr.ps1
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error |
| 3 | Resource not found |
| 4 | Rate limited |
| 5 | Timeout |
| 6 | Cancelled by user |
| 7 | Invalid input |
| 10 | Server error (5xx) |
| 11 | Client error (4xx) |

## Configuration

Config file location:
- Linux: `~/.config/vultr-cli/vultr/config.json`
- macOS: `~/Library/Application Support/com.vultr-cli.vultr/config.json`
- Windows: `%APPDATA%\vultr-cli\vultr\config.json`

```json
{
  "default_profile": "default",
  "profiles": {
    "default": {
      "output_format": "table"
    },
    "production": {
      "output_format": "json"
    }
  },
  "settings": {
    "output_format": "table",
    "confirm_destructive": true,
    "wait_timeout": 600,
    "poll_interval": 5
  }
}
```

## Support

Please use GitHub Issues for bug reports and feature requests.

## Releasing

This project follows Keep a Changelog and Semantic Versioning.

1. Update `CHANGELOG.md` (move items from Unreleased to a new version).
2. Bump the version in `Cargo.toml`.
3. Commit, tag `vX.Y.Z`, and push.

## License

MIT. See `LICENSE`.


## Secrets storage

By default, `vultr` stores your API token in the operating system keyring (macOS Keychain, Windows Credential Manager, Linux Secret Service).

If you are in an environment where the keyring is unavailable (some CI runners), you can enable an explicit fallback to a local `credentials.json` file by setting:

- `VULTR_CLI_INSECURE_FILE_SECRETS=1`

This fallback is less secure and should be used only when necessary.
