# keskos-welcome

`keskos-welcome` packages the KeskOS first-boot welcome experience and its fallback setup console. It combines a GTK/Rust desktop app, autostart entries, and a Python first-run helper used immediately after installation.

## What this is

This repository builds the package that introduces new users to the system, handles browser selection, offers maintenance and tweak actions, and provides a fallback console path if the graphical first-run flow cannot launch.

## Role in KeskOS

First-boot GUI app package.

## Package name

```txt
Package: keskos-welcome
Repo: [keskos]
Architecture: x86_64
```

## What it installs or provides

- Installs the main binary at `/usr/bin/kesk-welcome`.
- Installs desktop entries at `/usr/share/applications/kesk-welcome.desktop` and `/usr/share/applications/keskos-first-run.desktop`.
- Installs autostarts at `/etc/xdg/autostart/kesk-welcome.desktop`, `/etc/xdg/autostart/keskos-first-run.desktop`, and `/etc/skel/.config/autostart/keskos-first-run.desktop`.
- Installs the fallback first-run stack under `/usr/bin/keskos-first-run` and `/usr/lib/keskos-first-run/`.

## Commands and launchers

- `kesk-welcome` launches the main GUI directly.
- `kesk welcome` and `kesk welcome --first-run` route into the same flow when `keskos-tools` is installed.
- `kesk-welcome fix update-system|reinstall-packages|reset-keyrings|remove-lock|clear-cache|remove-orphans|rank-mirrors|install-gaming|show-kwin-debug|install-winboat|install-vram-management` runs maintenance tasks.
- `kesk-welcome tweak enable <name>`, `tweak disable <name>`, and `tweak list` manage supported systemd-based tweaks.
- `kesk-welcome dns set|set-custom|reset|list-connections|list-servers|test-latency` manages NetworkManager DNS settings.
- `keskos-first-run` runs the packaged fallback setup console.

## Config, logs, and state

- Welcome logs are written to `${XDG_STATE_HOME:-$HOME/.local/state}/kesk/logs/welcome.log`.
- The fallback first-run script writes to `${XDG_STATE_HOME:-$HOME/.local/state}/keskos/first-run.log`.
- Completion markers are stored in the user state directory so the welcome flow does not rerun unexpectedly.
- The package ships autostart desktop entries but does not ship standalone systemd units; tweak actions can enable or disable existing system or user services on the target machine.

## Dependencies

- Runtime dependencies: `gtk3`, `iputils`, `networkmanager`, `pyside6`, `python`, and `xdg-utils`.
- Optdepends: `keskos-browser-startpage`, `keskos-settings`, and `keskos-tools` for the full branded browser/theme/helper experience.
- Build tooling includes Rust/Cargo plus the makedepends declared in `PKGBUILD`.

## Build

```bash
makepkg -s --noconfirm
cargo build --release --manifest-path files/kesk-welcome/Cargo.toml
```

## Packaging notes

- This package provides, conflicts with, and replaces `kesk-welcome`.
- It stays separate from `keskos-settings` and `keskos-tools` because it is a first-boot app with its own runtime and UX flow.
- Browser installation is handled from Welcome at first boot rather than by making this repo a browser payload package itself.

## Troubleshooting

- If the GUI appears to hang or crash, inspect `${XDG_STATE_HOME:-$HOME/.local/state}/kesk/logs/welcome.log` first.
- If first-run does not open automatically, confirm the autostart desktop entries are installed and the user state marker does not already mark the flow complete.
- If browser installation options are unavailable, verify networking and the relevant pacman/AUR package availability for the selected browser.

## Docs website export notes

- Good website split: first-boot overview, command reference, browser setup flow, and troubleshooting/log paths.
- Keep the listed subcommands synchronized with the Rust clap definitions in `src/cli.rs` and `src/dns.rs`.
