#!/usr/bin/env bash
set -euo pipefail

make modules

rsync -a --mkpath "${KBUILD_OUTPUT}/drivers/uio/${MODULE_PREFIX}/${MODULE_NAME}" "$RASPI_CUSTOM_KERNEL_HOST":uio-modules/${MODULE_PREFIX}/${MODULE_NAME}
ssh "$RASPI_CUSTOM_KERNEL_HOST" 'kill -9 $(lsof -t /dev/uio0) || true &> /dev/null'
ssh "$RASPI_CUSTOM_KERNEL_HOST" "rmmod uio_uart uart_dint uart_scratch || true"
ssh "$RASPI_CUSTOM_KERNEL_HOST" "insmod uio-modules/${MODULE_PREFIX}/${MODULE_NAME}"
