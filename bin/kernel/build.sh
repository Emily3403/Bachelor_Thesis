#!/usr/bin/env bash
set -euo pipefail

echo ">> Building the Kernel ..."

make Image modules dtbs
make INSTALL_MOD_PATH="modules-install" modules_install &> /dev/null
