# p9s

A k9s-like terminal UI for Proxmox VE clusters.

## Features

- Real-time cluster resource view (nodes, VMs, LXC, storage)
- Start, stop, and reboot actions with confirmation prompts
- CPU/memory sparkline history in detail view
- Color-coded status indicators (with `--no-color` support)
- Keyboard-driven navigation with filter search
- Async task tracking for lifecycle operations

## Install

```bash
cargo install p9s
```

## Usage

```bash
p9s --host https://pve.local --token-id root@pam!p9s --token abc123 --insecure
```

## Configuration

`~/.config/p9s/config.yaml` — CLI flags override file values:

```yaml
host: https://pve.local
token_id: root@pam!p9s
token: abc123
insecure: true
refresh_interval: 5
no_color: false
```

### CLI Flags

```
Usage: p9s [OPTIONS]

Options:
      --host <HOST>                          Proxmox host URL
      --token-id <TOKEN_ID>                  API token ID (e.g. root@pam!p9s)
      --token <TOKEN>                        API token secret
      --insecure                             Allow insecure HTTPS (self-signed certs)
      --refresh-interval <REFRESH_INTERVAL>  Data refresh interval in seconds [default: 5]
      --filter <FILTER>                      Initial resource filter
      --no-color                             Disable colors
      --config <CONFIG>                      Path to config file
  -h, --help                                 Print help


```

### Environment

`P9S_TOKEN` — API token secret fallback
