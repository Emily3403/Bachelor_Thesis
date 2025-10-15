#!/usr/bin/env bash
set -euo pipefail

cargo build --release --target=aarch64-unknown-linux-gnu \
    --features "$FEATURES" \
    --bin "$BINARY_NAME"

# TODO: Trap the exit and kill polling
rsync -a --mkpath target/aarch64-unknown-linux-gnu/release/"$BINARY_NAME" "$RASPI_CUSTOM_KERNEL_HOST":driver-binaries/"$BINARY_NAME"

ssh $RASPI_CUSTOM_KERNEL_HOST 'kill $(lsof -t /dev/uio0) || true &> /dev/null'

if [ -z "${REALTIME+false}" ];
then
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "./driver-binaries/$BINARY_NAME"
else
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "chrt 99 ./driver-binaries/$BINARY_NAME"
fi
