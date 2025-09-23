#!/usr/bin/env bash

set -eu
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
HOST="user@130.149.220.6"

source "$SCRIPT_DIR/../.envrc"
cd "$SCRIPT_DIR/.."

rsync -a "$SCRIPT_DIR/../" "$HOST:linux-compile/" --info=progress2
ssh "$HOST" "cd linux-compile && ./bin/compile.sh"
rsync -a "$HOST:linux-compile/$KBUILD_OUTPUT/" "$SCRIPT_DIR/../$KBUILD_OUTPUT/" --delete --info=progress2

$SCRIPT_DIR/rpi/deploy-mod.sh