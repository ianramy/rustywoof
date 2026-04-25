#!/bin/bash
# install.sh for Rustywoof

set -e

echo "[INFO] Sniffing system architecture..."

# Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$ARCH" = "x86_64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="aarch64"
else
    echo "[CRITICAL] Unsupported architecture: $ARCH"
    exit 1
fi

REPO="ianramy/rustywoof"
VERSION="v0.1.9"
BINARY_NAME="woof-${OS}-${ARCH}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}"

echo "[INFO] Downloading Watchdog ($OS-$ARCH)..."
curl -sSL -o /tmp/woof.tar.gz "$DOWNLOAD_URL"

echo "[INFO] Unpacking executable..."
tar -xzf /tmp/woof.tar.gz -C /tmp

echo "[INFO] Installing to /usr/local/bin (may require sudo)..."
sudo mv /tmp/woof /usr/local/bin/woof
chmod +x /usr/local/bin/woof

rm /tmp/woof.tar.gz

echo ""
echo "✅ Rustywoof installed successfully!"
echo "🐾 Run 'woof --help' to get started."
