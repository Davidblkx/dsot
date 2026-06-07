# Setting Up DSOT on Linux

This guide provides instructions for setting up the DSOT Music Manager development environment on Linux.

## Prerequisites

DSOT is built using Rust, SQLite, and the Dioxus framework for the desktop user interface. As a result, you will need several system dependencies installed for compiling both SQLite/SQLx and the GTK/WebKit-based webview for Dioxus.

### 1. Install System Dependencies

Install the required build tools, SSL libraries, SQLite, and WebKit2GTK packages based on your Linux distribution:

#### Fedora / RHEL
```bash
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y \
  pkgconfig \
  openssl-devel \
  sqlite-devel \
  webkit2gtk4.1-devel \
  gtk3-devel \
  libayatana-appindicator3-devel
```

#### Debian / Ubuntu
```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  libsqlite3-dev \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev
```

#### Arch Linux
```bash
sudo pacman -Syu --needed \
  base-devel \
  pkgconf \
  openssl \
  sqlite \
  webkit2gtk-4.1 \
  gtk3 \
  libayatana-appindicator
```

#### If want to develop for android

https://dioxuslabs.com/learn/0.7/guides/platforms/mobile
https://v2.tauri.app/start/prerequisites/#configure-for-mobile-targets

```nushell
$env.JAVA_HOME = '/opt/android/jbr'
$env.ANDROID_HOME = $"($env.HOME)/Android/Sdk"
$env.NDK_HOME = (ls $"($env.HOME)/Android/Sdk/ndk" | last | get name)

$env.PATH ++= [$"($env.ANDROID_HOME)/platform-tools"]
$env.PATH ++= [$"($env.ANDROID_HOME)/emulator"]
```

---

### 2. Install Rust Toolchain

DSOT uses the **Rust 2024 Edition**, which requires Rust **v1.85.0** or newer.

1. Install `rustup` (the Rust toolchain manager) if you don't have it:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Follow the on-screen prompts, then restart your terminal or run:
   ```bash
   source "$HOME/.cargo/env"
   ```
3. Ensure you have the stable toolchain updated:
   ```bash
   rustup update stable
   ```

---

### 3. Install Dioxus CLI (`dx`)

The Dioxus CLI tool is used to run, build, and serve the desktop application.

You can install it using the official Dioxus installation script:
```bash
curl -sSL http://dioxus.dev/install.sh | sh
```
*Alternatively, you can build it from source via Cargo (this may take a few minutes):*
```bash
cargo install dioxus-cli
```

Verify that the CLI is installed correctly:
```bash
dx --version
```

---

### 4. Configuration (`.env`)

DSOT uses environment variables to configure its data paths. A default `.env` file is included in the project root:

```ini
DATABASE_URL=sqlite:./target/local.db
```

No additional configuration is necessary to get started, as SQLite database migrations will run automatically upon starting the application.

If you ever wish to manually inspect or run migrations using the SQLx CLI, you can optionally install the tool:
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

---

## Building and Running

### Compilation

To compile the entire workspace (all modules and the desktop app):
```bash
cargo build
```

### Running Tests

To run the unit and integration tests across all workspace member crates:
```bash
cargo test
```

### Running the Desktop App

```bash
dx serve --package dsot_desktop
```
