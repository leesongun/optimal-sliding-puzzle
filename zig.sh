#!/bin/sh

# Check if jq is installed
command -v jq >/dev/null 2>&1 || { echo >&2 "jq is required but not installed. Aborting."; exit 1; }

# Get the architecture and platform
arch=$(uname -m)
platform=$(uname -s | tr '[:upper:]' '[:lower:]')

# Specify the installation path (change this to your preferred path)
install_path="$HOME/.local/zig"

# Clean up previous installations
if [ -d "$install_path" ]; then
    echo "Cleaning up previous installations in $install_path"
    rm -rf "$install_path"
fi

# Download the JSON file and extract the download URL
URL=$(curl -s https://ziglang.org/download/index.json | jq -r '.master."'$arch'-'$platform'".tarball' 2>&1)

# Download and extract Zig
if [ -n "$URL" ]; then
    echo "Downloading Zig Nightly from $URL"
    temp_dir=$(mktemp -d)
    curl -L "$URL" | tar xJ -C "$temp_dir"
    mv "$temp_dir"/* "$install_path"
    rmdir "$temp_dir"
    echo "Zig Nightly has been installed to $install_path"

    # Add Zig binary to PATH
    echo "export PATH=\"\$PATH:$install_path\"" >> "$HOME/.profile"
    . "$HOME/.profile"
    echo "Zig has been added to your PATH."
else
    echo "Failed to get the download URL for Zig Nightly."
    exit 1
fi
