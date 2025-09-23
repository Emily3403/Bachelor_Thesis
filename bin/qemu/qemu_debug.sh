#!/bin/bash

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
source "$SCRIPT_DIR/../.envrc"

if [ ! -f "$SCRIPT_DIR/../$KBUILD_OUTPUT/arch/arm64/boot/Image" ]
then
  echo -e "ERROR: I can't find the file arch/arm64/boot/Image!\nPlease make sure you compiled the kernel with \`./compile.sh\`"
  exit 1
fi

bash "$SCRIPT_DIR/qemu_stop.sh" & disown

qemu-system-aarch64 -m 1024 -M raspi3b -nographic -s -S \
  -kernel "$SCRIPT_DIR/../$KBUILD_OUTPUT/arch/arm64/boot/Image" -dtb "$SCRIPT_DIR/../$KBUILD_OUTPUT/arch/arm64/boot/dts/broadcom/bcm2837-rpi-3-b.dtb" \
  -sd "$SCRIPT_DIR/2022-09-22-raspios-bullseye-arm64-lite.qcow2" \
  -append "console=ttyAMA0 root=/dev/mmcblk0p2" & disown