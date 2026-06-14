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

## Key Bindings

| Key            | Action                 |
| -------------- | ---------------------- |
| `q` / `Ctrl+C` | Quit                   |
| `?`            | Help                   |
| `/`            | Filter                 |
| `↑`/`↓`        | Navigate               |
| `Enter`        | View details           |
| `s`            | Start VM/CT            |
| `S`            | Stop VM/CT (confirm)   |
| `r`            | Reboot VM/CT (confirm) |
| `Esc`          | Close modal            |

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
--host          Proxmox host URL
--token-id      API token ID (e.g. root@pam!p9s)
--token          API token secret
--config         Path to config file
--filter         Initial resource filter
--insecure       Accept self-signed certs
--refresh-interval  Seconds between polls (default: 5)
--no-color       Disable colors
```

### Environment

`P9S_TOKEN` — API token secret fallback
