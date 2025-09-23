#!/usr/bin/env bash

set -eu  # Make EU great again!

# User configuration
mkdir -p "$HOME/.ssh"
echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINA0V/ByFxlMU8nBJ+R2RGxr0uZAapovARLPbHYmNE2V emily" > "$HOME/.ssh/authorized_keys"
chmod 700 "$HOME/.ssh"
chmod 600 "$HOME/.ssh/"*

sudo apt update
sudo apt -y install neovim fish btop git python3-pip lldb minicom lsof

sudo sh -c 'mkdir -p /root/.ssh && echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHAzQFMYrSvjGtzcOUbR1YHawaPMCBDnO4yRKsV7WHkg emily" > /root/.ssh/authorized_keys && chown -R root:root /root/.ssh && chmod 700 /root/.ssh && chmod 600 /root/.ssh/*'
chsh -s /usr/bin/fish

# System Setup
echo 'SUBSYSTEM=="uio", GROUP="emily", MODE="0660"' | sudo tee /etc/udev/rules.d/99-uart.rules
sudo chown root:root /etc/udev/rules.d/99-uart.rules
sudo sudo chmod 0644 /etc/udev/rules.d/99-uart.rules
sudo udevadm control --reload-rules

# Install necessary packages
pip install --break-system-packages pyserial ipython

# Setup my personal fish shell
mkdir -p "$HOME/.config/fish" "$HOME/.config/btop"
wget --inet4-only https://raw.githubusercontent.com/Emily3403/configAndDotfiles/main/roles/shell/tasks/dotfiles/fish/config.fish -O "$HOME/.config/fish/config.fish"
wget --inet4-only https://raw.githubusercontent.com/Emily3403/configAndDotfiles/main/roles/shell/tasks/dotfiles/btop/btop.conf -O "$HOME/.config/btop/btop.conf"

git config --global user.email "seebeckemily3403@gmail.com"
git config --global user.name "Emily3403"


echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# TODO: config.txt
# TODO: Disabling the linux console via raspi-config

echo 'RUST_LOG=trace
RUST_BACKTRACE=short' > /etc/environment
