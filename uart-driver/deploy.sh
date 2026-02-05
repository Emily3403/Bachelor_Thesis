#!/usr/bin/env bash
set -euo pipefail

cargo build --release --target=aarch64-unknown-linux-gnu \
    --features "$FEATURES" \
    --bin "$BINARY_NAME"

rsync -a --mkpath target/aarch64-unknown-linux-gnu/release/"$BINARY_NAME" "$RASPI_CUSTOM_KERNEL_HOST":driver-binaries/"$BINARY_NAME"


ssh $RASPI_CUSTOM_KERNEL_HOST 'kill -9 $(lsof -t /dev/uio0) &> /dev/null || true'

if [ "$REALTIME" == "true" ];
then
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "chrt 99 ./driver-binaries/$BINARY_NAME -b '$BAUDRATE' --savedir '$SAVEDIR' --num-data-bytes '$NUM_DATA_BYTES'"
else
    ssh -t $RASPI_CUSTOM_KERNEL_HOST "./driver-binaries/$BINARY_NAME -b '$BAUDRATE' --savedir '$SAVEDIR' --num-data-bytes '$NUM_DATA_BYTES'"
fi
