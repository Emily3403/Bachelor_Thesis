#!/usr/bin/env bash
set -euo pipefail

make modules

LOCAL_PATH="${KBUILD_OUTPUT}/drivers/uio/${MODULE_PREFIX}/${MODULE_NAME}"
REMOTE_PATH="uio-modules/${MODULE_PREFIX}/${MODULE_NAME}"

LOCAL_SHA=$(sha256sum "$LOCAL_PATH" | awk '{print $1}')
REMOTE_SHA=$(ssh "$RASPI_CUSTOM_KERNEL_HOST" "sha256sum '$REMOTE_PATH' 2>/dev/null" | awk '{print $1}' || true)

if [ -z "$REMOTE_SHA" ] || [ "$LOCAL_SHA" != "$REMOTE_SHA" ];
then
    rsync -a --mkpath "$LOCAL_PATH" "$RASPI_CUSTOM_KERNEL_HOST":"$REMOTE_PATH"
fi

ssh "$RASPI_CUSTOM_KERNEL_HOST" 'kill -9 $(lsof -t /dev/uio0) || true &> /dev/null'
ssh "$RASPI_CUSTOM_KERNEL_HOST" "rmmod uio_uart uart_dint uart_scratch || true"
ssh "$RASPI_CUSTOM_KERNEL_HOST" "insmod uio-modules/${MODULE_PREFIX}/${MODULE_NAME}"
