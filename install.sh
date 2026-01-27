#!/bin/sh
set -e

REPO="https://gitea.jaidaken.dev/jaidaken/ferrosonic.git"
INSTALL_DIR="/usr/local/bin"

echo "Ferrosonic installer"
echo "===================="

# Detect package manager and install dependencies
if command -v pacman >/dev/null 2>&1; then
    echo "Detected Arch Linux"
    sudo pacman -S --needed --noconfirm mpv pipewire wireplumber base-devel pkgconf dbus
elif command -v dnf >/dev/null 2>&1; then
    echo "Detected Fedora"
    sudo dnf install -y mpv pipewire wireplumber gcc pkgconf-pkg-config dbus-devel
elif command -v apt >/dev/null 2>&1; then
    echo "Detected Debian/Ubuntu"
    sudo apt update
    sudo apt install -y mpv pipewire wireplumber build-essential pkg-config libdbus-1-dev
else
    echo "Unknown package manager. Please install manually: mpv, pipewire, wireplumber, pkg-config, dbus dev headers"
    echo "Then re-run this script."
    exit 1
fi

# Install Rust if not present
if ! command -v cargo >/dev/null 2>&1; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
fi

# Clone and build
TMPDIR=$(mktemp -d)
echo "Building ferrosonic..."
git clone "$REPO" "$TMPDIR/ferrosonic"
cd "$TMPDIR/ferrosonic"
cargo build --release

# Install
sudo cp target/release/ferrosonic "$INSTALL_DIR/ferrosonic"
echo ""
echo "Ferrosonic installed to $INSTALL_DIR/ferrosonic"
echo "Run 'ferrosonic' to start."

# Cleanup
rm -rf "$TMPDIR"
