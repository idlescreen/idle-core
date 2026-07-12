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

Manage and interact with the screensaver daemon session via the terminal:

```bash
# Query the current session status
trance status

# Lock the screensaver session immediately
trance lock

# Reload plug-in configurations
trance reload
```

---

## TUI (Terminal User Interface)

Launch the interactive terminal dashboard to monitor daemon health, check logs, swap screensaver libraries, and configure simulation frame-rates dynamically:

```bash
trance-tui
```

---

## COSMIC Panel Applet

An integrated applet for the COSMIC Desktop bar (`cosmic-panel`). Toggle screensaver activation, lock the screen, or switch visual designs directly from your desktop interface:

```bash
trance-applet
```

---

## License

[Apache-2.0](LICENSE) · Copyright 2026 Crateria
