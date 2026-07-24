# IdleScreen ship names

## Installable packages

| Role | Package | Command / unit |
|------|---------|----------------|
| **Daemon + host** | **`idle`** | `idle-daemon`, unit `idle-daemon.service` |
| **CLI** | **`idle-cli`** | **`idle`** status / doctor / preview |
| All savers meta | `idle-savers` | hard-depends every `idle-saver-*` |
| One effect | `idle-saver-<name>` | `.so` under `/usr/libexec/idle/screensavers/` |
| COSMIC product | **`idle-cosmic`** | applet + requires `idle` + `idle-savers` |
| Live TUI | **`idle-tui`** | `idle-tui` |
| Studio | **`idle-studio`** | offline director |

## One-command COSMIC

```bash
sudo dnf install idle-cosmic
# pulls: idle + idle-savers (all idle-saver-*) + COSMIC applet
idle status
```

## Paths

| | Canonical | Legacy still read |
|--|-----------|-------------------|
| Plugins | `/usr/libexec/idle/screensavers` | idlescreen/, trance/ |
| Config | `~/.config/idle/` | idlescreen/, trance/ |

## Frozen ABI

- D-Bus: `io.github.ubermetroid.trance`
- Plugin stem: `libscreensaver_<name>.so`
- Internal crates: `trance-*`
