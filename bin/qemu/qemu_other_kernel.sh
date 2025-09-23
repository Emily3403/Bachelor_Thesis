#!/bin/bash

set -e
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "$SCRIPT_DIR/../.envrc"

KERNEL_IMAGE="$SCRIPT_DIR/kernel/kernel8.img"
DTB_FILE="$SCRIPT_DIR/kernel/bcm2710-rpi-3-b-plus.dtb"
FORWARD_PORT=10022


if [ ! -f "$KERNEL_IMAGE" ]
then
  echo -e "ERROR: I can't find the file $KERNEL_IMAGE"
  exit 1
fi

qemu-system-aarch64 -m 1024 -M raspi3b -nographic \
  -kernel "$KERNEL_IMAGE" -dtb "$DTB_FILE" \
  -sd "$SCRIPT_DIR/2024-07-04-raspios-bookworm-arm64-lite.qcow2" \
  -device usb-net,netdev=net0 -netdev user,id=net0,hostfwd=tcp::$FORWARD_PORT-:22 \
  -append "console=ttyAMA0 earlyprintk root=/dev/mmcblk0p2 rw rootfstype=ext4 rootdelay=1"
