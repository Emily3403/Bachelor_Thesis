#!/usr/bin/env bash
set -euo pipefail

KERNEL_HASH_FILE="kernel-patches/uio.sha256sum"
if [ -f "$KERNEL_HASH_FILE" ];
then
    existing_hash=$(cat "$KERNEL_HASH_FILE")
else
    existing_hash=""
fi


current_hash=$(sha256sum <(find "kernel-patches/uio" -type f -exec sha256sum {} \; | sort) | awk '{print $1}')
if [ "$current_hash" != "$existing_hash" ];
then
    make modules
    for dir in "irq" "polling"
    do
        rsync -a --mkpath \
            "${KBUILD_OUTPUT}/drivers/uio/$dir/"*.ko \
            "$RASPI_CUSTOM_KERNEL_HOST":"uio-modules/$dir/" &
    done

    wait
    echo "$current_hash" > "$KERNEL_HASH_FILE"
fi


ssh "$RASPI_CUSTOM_KERNEL_HOST" "kill -9 \$(lsof -t /dev/uio0) &> /dev/null || true;
rmmod uio_uart uart_dint uart_scratch &> /dev/null || true;
insmod uio-modules/${MODULE_PREFIX}/${MODULE_NAME}"
