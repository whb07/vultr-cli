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
vultr-cli auth login

# Or use environment variable
export VULTR_API_KEY="your-api-key"

# Or pass directly (not recommended for scripts)
vultr-cli --api-key "your-key" instance list

# Check auth status
vultr-cli auth status

# Logout (remove stored key)
vultr-cli auth logout
```

## Usage Examples

### Instances (VMs)

```bash
# List all instances
vultr-cli instance list

# Create an instance
vultr-cli instance create \
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
vultr-cli instance get <instance-id>

# Start/stop/reboot
vultr-cli instance start <instance-id> --wait
vultr-cli instance stop <instance-id> --wait
vultr-cli instance reboot <instance-id>

# Delete (with confirmation)
vultr-cli instance delete <instance-id>

# Delete without confirmation, wait for deletion
vultr-cli instance delete <instance-id> -y --wait
```

### SSH Keys

```bash
# List SSH keys
vultr-cli ssh-key list

# Create from string
vultr-cli ssh-key create --name "my-key" --key "ssh-rsa AAAA..."

# Create from file
vultr-cli ssh-key create --name "my-key" --key @~/.ssh/id_rsa.pub

# Delete
vultr-cli ssh-key delete <key-id>
```

### Startup Scripts

```bash
# List scripts
vultr-cli startup-script list

# Create from file
vultr-cli startup-script create --name "setup" --script @./setup.sh --script-type boot

# Get script (shows decoded content)
vultr-cli startup-script get <script-id>
```

### Snapshots

```bash
# List snapshots
vultr-cli snapshot list

# Create from instance
vultr-cli snapshot create --instance-id <id> --description "Before upgrade" --wait

# Create from URL
vultr-cli snapshot create-from-url --url "https://..." --description "Custom image"

# Delete
vultr-cli snapshot delete <snapshot-id> --wait
```

### Block Storage

```bash
# List volumes
vultr-cli block-storage list

# Create
vultr-cli block-storage create --region ewr --size 100 --label "data-vol" --wait

# Attach to instance (live)
vultr-cli block-storage attach <block-id> --instance-id <instance-id> --live

# Detach
vultr-cli block-storage detach <block-id> --live

# Delete
vultr-cli block-storage delete <block-id> --wait
```

### Firewalls

```bash
# List firewall groups
vultr-cli firewall group list

# Create group
vultr-cli firewall group create --description "Web servers"

# Add rule (allow SSH from anywhere)
vultr-cli firewall rule create \
  --group-id <group-id> \
  --ip-type v4 \
  --protocol TCP \
  --subnet 0.0.0.0 \
  --subnet-size 0 \
  --port 22 \
  --notes "SSH access"

# Add rule (allow HTTP)
vultr-cli firewall rule create \
  --group-id <group-id> \
  --ip-type v4 \
  --protocol TCP \
  --subnet 0.0.0.0 \
  --subnet-size 0 \
  --port 80 \
  --notes "HTTP"

# List rules
vultr-cli firewall rule list --group-id <group-id>
```

### VPCs

```bash
# List VPCs
vultr-cli vpc list

# Create
vultr-cli vpc create --region ewr --description "Production" --subnet 10.0.0.0 --subnet-mask 24

# Delete
vultr-cli vpc delete <vpc-id>
```

### Reference Data

```bash
# List regions
vultr-cli regions

# List plans (optionally filter by type)
vultr-cli plans
vultr-cli plans --plan-type vc2
vultr-cli plans --bare-metal

# List operating systems
vultr-cli os
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
vultr-cli completions bash > /etc/bash_completion.d/vultr-cli

# Zsh
vultr-cli completions zsh > ~/.zfunc/_vultr-cli

# Fish
vultr-cli completions fish > ~/.config/fish/completions/vultr-cli.fish

# PowerShell
vultr-cli completions powershell > vultr-cli.ps1
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
- Linux: `~/.config/vultr-cli/config.json`
- macOS: `~/Library/Application Support/com.vultr.vultr-cli/config.json`
- Windows: `%APPDATA%\vultr\vultr-cli\config.json`

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

## License

MIT


## Secrets storage

By default, `vultr-cli` stores your API token in the operating system keyring (macOS Keychain, Windows Credential Manager, Linux Secret Service).

If you are in an environment where the keyring is unavailable (some CI runners), you can enable an explicit fallback to a local `credentials.json` file by setting:

- `VULTR_CLI_INSECURE_FILE_SECRETS=1`

This fallback is less secure and should be used only when necessary.
