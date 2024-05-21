#!/usr/bin/env bash

set -e

# Update local package index
echo "Updating cache..."
sudo apt-get update

# Install dependency packages
echo "Installing dependencies..."
sudo apt-get install -y pkg-config

# Install compilation target
rustup target add wasm32-wasi
