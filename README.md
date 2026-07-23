<h1 align="center">
  <img src="assets/icon.png" width="48" height="48" valign="middle"> Trance
</h1>

<p align="center">
  <b>Modular Wayland-native screensaver and ambient display daemon for Linux, written in Rust.</b>
</p>

<p align="center">
  Part of <a href="https://github.com/crateria">Crateria</a> — Linux desktop software in Rust.
</p>

---

### Install (native packages)

On **Debian / Ubuntu / Pop!_OS**:

```bash
sudo mkdir -p /etc/apt/keyrings
sudo curl -fsSL https://crateria.github.io/packages/apt/crateria-keyring.gpg \
  -o /etc/apt/keyrings/crateria.gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/crateria.gpg] https://crateria.github.io/packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/crateria.list
sudo apt update
sudo apt install trance
```

On **Fedora**:

```bash
sudo curl -fsSL https://crateria.github.io/packages/rpm/crateria.repo \
  -o /etc/yum.repos.d/crateria.repo
sudo dnf install trance
```

Package index: [crateria.github.io/packages](https://crateria.github.io/packages/)  
Official plugins: [crateria/trance-plugins](https://github.com/crateria/trance-plugins)

---

### Environment configuration

| Variable | Description | Default |
| :--- | :--- | :---: |
| `TRANCE_IDLE_TIMEOUT_MINS` | Minutes of inactivity before screensaver activates | `10` |
| `TRANCE_ACTIVE_SAVER` | Active plugin name (e.g. `beams`, `matrix`, `flurry`) | `beams` |
| `TRANCE_SHOW_FPS` | Display real-time FPS overlay | `false` |
| `LOG_LEVEL` | Tracing filter (`error`, `warn`, `info`, `debug`) | `info` |

---

### Administration CLI

```bash
trance-cli status              # screensaver state and active plugin
trance-cli enable              # enable automatic idle screensaver
trance-cli disable             # disable automatic idle screensaver
trance-cli preview <plugin>    # full-screen preview of a plugin
```

---

### Architecture

- **Native Wayland** — `ext-idle-notify-v1` and `ext-session-lock-v1`
- **GPU-accelerated rendering** — wgpu cell-based visualizers
- **PAM lock integration** — secure screen lock with local fallback
- **Plugin system** — loadable effects (see trance-plugins)

---

### License

Apache-2.0. See [LICENSE](LICENSE).
