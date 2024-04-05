#!/usr/bin/env bash

set -e

# Update local package index
echo "Updating cache..."
sudo apt update

# Install dependency packages
echo "Installing dependencies..."
sudo apt install -y pkg-config

# Install compilation target
rustup target add wasm32-wasi
