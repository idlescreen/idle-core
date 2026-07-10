# Trance

<img src="assets/icon.svg" width="48" height="48" alt="trance logo" align="right">

Wayland-native screensaver for Linux. A small background daemon watches for idle time and shows modular effects (beams, storm, radar, and more).

Works on any Wayland desktop. Optional **COSMIC** panel applet; everyone else can use the **TUI** or **CLI**.

---

## Install

Packages are published from [UberMetroid/packages](https://github.com/UberMetroid/packages) (GitHub Pages).

### Debian / Ubuntu / Pop!_OS

```bash
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://ubermetroid.github.io/packages/apt/ubermetroid-keyring.gpg \
  -o /etc/apt/keyrings/ubermetroid.gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/ubermetroid.gpg] https://ubermetroid.github.io/packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/ubermetroid.list

sudo apt update
sudo apt install trance
```

`trance` pulls in the daemon plus recommended **CLI**, **TUI**, and **plugins**.  
Core only (no recommends): `sudo apt install --no-install-recommends trance`

### Fedora

```bash
sudo curl -fsSL https://ubermetroid.github.io/packages/rpm/ubermetroid.repo \
  -o /etc/yum.repos.d/ubermetroid.repo

sudo dnf install trance
```

### First-time setup (every user)

The daemon is a **user** systemd service (not system-wide):

```bash
systemctl --user enable --now trance-daemon
trance status          # should show running
```

After package upgrades, if the screensaver stops working:

```bash
trance doctor --fix
```

### COSMIC panel applet (optional)

Not installed by default (GNOME/KDE/Hyprland don’t need it).

```bash
sudo apt install trance-applet    # or: sudo dnf install trance-applet
```

On Fedora, if `cosmic-panel` is already installed, dnf may offer the applet via a weak dependency.

---

## How to use

Pick one control surface — they all talk to the same daemon.

| Interface | Best for | Package |
|-----------|----------|---------|
| **CLI** `trance` | Scripts, quick commands | `trance-cli` (recommended with `trance`) |
| **TUI** `trance-tui` | Any desktop, keyboard UI | `trance-tui` (recommended) |
| **Applet** | COSMIC panel | `trance-applet` (optional) |

### CLI

```bash
trance status                 # live state
trance enable                 # allow idle screensaver
trance disable                # stop idle activation
trance timeout 10             # idle minutes (1–240)
trance list                   # installed savers
trance saver set beams        # or: trance saver set random
trance preview storm          # try a saver now
trance stop                   # end preview / presentation
trance doctor                 # diagnostics
trance doctor --fix          # reload/enable/restart user service
```

More: `trance help` — includes `config`, `inhibitors`, `fps-overlay`, `render-scale`, `completion`, `bug-report`.

### TUI

```bash
trance-tui
```

| Key | Action |
|-----|--------|
| `Tab` | Switch Settings / Screensavers |
| `↑` `↓` | Navigate |
| `Space` / `Enter` | Toggle option or set active saver |
| `←` `→` | Adjust timeout or render scale |
| `p` | Preview selected saver |
| `q` / `Esc` | Quit |

Turning **Daemon Service** on uses `systemctl --user enable --now` so it starts again at login.

### COSMIC applet

1. Install `trance-applet` (see above).
2. Panel → **Add Applet** → search **Trance**.
3. Click the icon for settings; middle-click for a quick preview.

You can start/stop the daemon, set idle timeout, pick a saver, adjust render scale, toggle FPS overlay, and preview. Battery mode shows a short “30 FPS” badge when unplugged.

---

## Screensavers

Default install includes **beams** (hard dependency). The recommended `trance-plugins-all` meta package pulls the rest:

| Name | Effect |
|------|--------|
| beams | Spotlight cones over a starfield |
| bursts | City skyline fireworks |
| chaos | Logo glitch / chromatic aberration |
| cosmos | Accretion / singularity cycle |
| glyphs | Matrix-style rain + system info |
| gnats | Firefly predator/prey swarm |
| radar | Retro sweeping radar |
| storm | Rain, lightning, wildlife |

Install a single plugin if you prefer, e.g. `sudo apt install trance-plugin-storm`.

---

## Upgrades

```bash
# APT
sudo apt update && sudo apt upgrade trance
trance doctor --fix

# DNF
sudo dnf upgrade trance
trance doctor --fix
```

Package scripts try to restart the user service; they cannot always reach your session bus. `trance doctor --fix` is the reliable post-upgrade step.

---

## Configuration file

`~/.config/trance/config.yaml` (created automatically). The CLI, TUI, and applet edit the same settings over D-Bus when the daemon is running.

Useful environment overrides:

| Variable | Meaning |
|----------|---------|
| `TRANCE_RENDER_SCALE` | Simulation scale `0.25`–`1.0` (lower = cheaper) |
| `TRANCE_MAX_FPS` | Cap FPS; `0` = match display refresh |

---

## Links

* Packages / repo setup notes: [UberMetroid/packages](https://github.com/UberMetroid/packages)
* Plugins source: [UberMetroid/trance-plugins](https://github.com/UberMetroid/trance-plugins)
* Security policy: [SECURITY.md](SECURITY.md)

---

## License

[Apache License 2.0](LICENSE) · Copyright 2026 UberMetroid
