<p align="center">
  <a href="https://crateria.github.io/">
    <img src="assets/crateria-header.jpg" alt="Crateria" width="100%">
  </a>
</p>

# Trance

[![CI](https://github.com/crateria/trance/actions/workflows/ci.yml/badge.svg)](https://github.com/crateria/trance/actions/workflows/ci.yml) [![APT Repository](https://img.shields.io/badge/apt-repo-blue.svg)](https://crateria.github.io/packages/) [![DNF Repository](https://img.shields.io/badge/dnf-repo-blue.svg)](https://crateria.github.io/packages/)

Wayland-native modular screensaver daemon and desktop suite for Linux.

---

## Installation

Configure the repository and install the application packages.

### Fedora / RHEL (DNF)
```bash
# Add repository configuration
sudo curl -o /etc/yum.repos.d/crateria.repo https://crateria.github.io/packages/rpm/crateria.repo

# Install all suite components
sudo dnf install trance trance-cli trance-tui trance-applet
```

### Ubuntu / Debian (APT)
```bash
# Add keyring
sudo curl -sSLo /etc/apt/keyrings/crateria.gpg https://crateria.github.io/packages/apt/crateria-keyring.gpg

# Add sources list
echo "deb [signed-by=/etc/apt/keyrings/crateria.gpg] https://crateria.github.io/packages/apt stable main" | sudo tee /etc/apt/sources.list.d/crateria.list

# Install all suite components
sudo apt update && sudo apt install trance trance-cli trance-tui trance-applet
```

---

## CUI (Command Line Interface)

Manage and interact with the screensaver daemon session via the terminal.

### Active Session Controls:
* **`trance status`** (or `st`)  
  Query session status (lock status, timeouts, inhibitors). Use `--json` for scripting.
* **`trance enable`** / **`disable`** (or `on` / `off`)  
  Toggle screensaver idle activation.
* **`trance timeout <mins>`** (or `t`)  
  Query or set the idle activation timeout length.
* **`trance saver <plugin>`**  
  Query or set the active default screensaver plugin.
* **`trance preview <plugin>`** (or `p`)  
  Instantly trigger a preview of the specified screensaver.
* **`trance stop`**  
  Dismiss any active screensaver presentation or running preview.
* **`trance list`** (or `ls`)  
  List all available screensaver plugin library names.
* **`trance inhibitors`**  
  List active D-Bus session inhibitors blocking screensaver activation.
* **`trance render-scale <value>`** (or `scale`)  
  Set pixel grid scale multiplier (e.g. `0.5`, `1.0`, `2.0`).
* **`trance fps-overlay <on/off>`** (or `fps`)  
  Toggle or query the live frame-rate counter overlay.
* **`trance interactive`** (or `i`)  
  Launch a lightweight interactive control prompt in your terminal.

### System Utilities:
* **`trance doctor`** (or `doc`)  
  Perform system health diagnostics (PAM config, DBus socket, group membership). Pass `--fix` to repair issues.
* **`trance clean`**  
  Purge orphaned system sockets, locks, and temporary log directories.
* **`trance completion <shell>`**  
  Generate tab-completion scripts (supports `bash`, `zsh`, `fish`).
* **`trance bug-report`**  
  Bundle system specifications, configs, and logs for debugging.
* **`trance update`** (or `self-update`)  
  Check for and apply client upgrades.

---

## TUI (Terminal User Interface)

Launch the interactive terminal dashboard to monitor daemon health, check logs, swap screensaver libraries, and configure simulation frame-rates dynamically:

```bash
trance-tui
```

---

## COSMIC Panel Applet

An integrated applet for the COSMIC Desktop bar (`cosmic-panel`). Toggle screensaver activation, lock the screen, or switch visual designs directly from your desktop interface.

### Adding to your panel:
1. **Right-click** on your desktop panel/bar.
2. Select **Add Applet**.
3. Search for **Trance** and add it to your panel.

*(Alternatively, run `trance-applet` in a terminal or startup script to launch the background panel process directly.)*

---

## License

[Apache-2.0](LICENSE) · Copyright 2026 Crateria
