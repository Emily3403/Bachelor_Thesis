#!/bin/bash

set -e
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

if [ ! -f "$SCRIPT_DIR/../$KBUILD_OUTPUT/vmlinux" ]
then
  echo -e "ERROR: I can't find the file \`vmlinux\`!\nPlease make sure you compiled the kernel with \`./compile.sh\`"
  exit 1
fi

gdb-multiarch "$SCRIPT_DIR/../$KBUILD_OUTPUT/vmlinux" -ex "target remote localhost:1234"