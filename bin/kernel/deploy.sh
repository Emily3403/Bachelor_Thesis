#!/usr/bin/env bash
set -euo pipefail

echo ">> Copying to raspi..."

# Send all the rsync processes into the background.
rsync -a "${KBUILD_OUTPUT}/modules-install/lib" "$RASPI_CUSTOM_KERNEL_HOST":/lib/ &

# We have to use `-rltD` as rsync flags and not `-a`, because  /boot/firmware lives on a fat32 filesystem, not allowing ownership (or group) changes.
rsync -rltD "${KBUILD_OUTPUT}/arch/arm64/boot/Image" "$RASPI_CUSTOM_KERNEL_HOST":/boot/firmware/kernel8.img &
rsync -rltD "${KBUILD_OUTPUT}/arch/arm64/boot/dts/broadcom/" "$RASPI_CUSTOM_KERNEL_HOST":/boot/firmware/ &

wait
