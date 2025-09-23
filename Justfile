set dotenv-load := true
set dotenv-path := "./.envrc"

mod kernel "./bin/kernel.just"
mod polling "./bin/polling.just"
mod irq "./bin/irq.just"

# Installs neccessary dependencies to your local machine.
[group("Setup")]
system-setup:
    @echo "Setting up your Machine (Arch Linux) with dependencies ..."

    sudo pacman -S --needed git clang llvm lld aarch64-linux-gnu-linux-api-headers
    command -v cargo || (echo "Error: cargo is needed, install externally!"; exit 1)
    command -v rustc || (echo "Error: rustc is needed, install externally!"; exit 1)
