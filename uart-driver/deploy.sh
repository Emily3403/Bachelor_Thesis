#!/usr/bin/env bash
set -euo pipefail

UART_DRIVER_HASH_FILE="./src.sha256sum"
if [ -f "$UART_DRIVER_HASH_FILE" ];
then
    existing_hash=$(cat "$UART_DRIVER_HASH_FILE")
else
    existing_hash=""
fi

current_hash=$(sha256sum <(find "./src" -type f -exec sha256sum {} \; | sort) | awk '{print $1}')
if [ "$current_hash" != "$existing_hash" ];
then
    cargo build --release --target=aarch64-unknown-linux-gnu \
        --features "$FEATURES" \
        --bin "$BINARY_NAME"

    rsync -a --mkpath target/aarch64-unknown-linux-gnu/release/"$BINARY_NAME" "$RASPI_CUSTOM_KERNEL_HOST":driver-binaries/"$BINARY_NAME"
    echo "$current_hash" > "$UART_DRIVER_HASH_FILE"
fi


ssh $RASPI_CUSTOM_KERNEL_HOST 'kill -9 $(lsof -t /dev/uio0) &> /dev/null || true'

if [ "$REALTIME" == "true" ];
then
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "chrt 99 ./driver-binaries/$BINARY_NAME"
else
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "./driver-binaries/$BINARY_NAME"
fi
