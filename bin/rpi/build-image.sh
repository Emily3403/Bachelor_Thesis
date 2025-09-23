#!/usr/bin/env bash
# Attention: Execute the script with working dir in the script dir

set -e

# Note the use of -nc: skip downloads that would download to existing files.
if [ ! -f 2024-07-04-raspios-bookworm-arm64-lite.img ]
then
  wget -nc https://downloads.raspberrypi.com/raspios_lite_arm64/images/raspios_lite_arm64-2024-07-04/2024-07-04-raspios-bookworm-arm64-lite.img.xz
fi

rm -f 2024-07-04-raspios-bookworm-arm64-lite.img.sha256
wget https://downloads.raspberrypi.com/raspios_lite_arm64/images/raspios_lite_arm64-2024-07-04/2024-07-04-raspios-bookworm-arm64-lite.img.xz.sha256

expected_shasum=$(cat 2024-07-04-raspios-bookworm-arm64-lite.img.xz.sha256)
actual_shasum=$(sha256sum 2024-07-04-raspios-bookworm-arm64-lite.img.xz)

if [ "$expected_shasum" != "$actual_shasum" ]
then
  echo "ERROR: The sha256sum of the file is not as expected:"
  echo "$actual_shasum"
  echo "$expected_shasum"
  echo "↑ Expected"

  exit 1
fi

echo -e "\nStarting to unpack the archive. This might take a while ..."
unxz 2024-07-04-raspios-bookworm-arm64-lite.img.xz

echo "Done unpacking the archive!"

if [ ! -f 2024-07-04-raspios-bookworm-arm64-lite.qcow2 ]
then
  mkdir -p boot kernel
  sudo mount -o loop,offset=4194304 2024-07-04-raspios-bookworm-arm64-lite.img boot
  sudo bash -c "echo 'pi:\$y\$j9T\$X2rogXskOZapYyaiRaEQf1\$P15LoqszASuZlJHw47BSWjw7Yn4WBCro9F5Az62LnD.' > boot/userconf"
  cp boot/kernel8.img kernel/
  cp boot/bcm2710-rpi-3-b-plus.dtb kernel/
  sudo umount boot

  qemu-img convert -f raw -O qcow2 2024-07-04-raspios-bookworm-arm64-lite.img 2024-07-04-raspios-bookworm-arm64-lite.qcow2
  qemu-img resize 2024-07-04-raspios-bookworm-arm64-lite.qcow2 4g > /dev/null
fi

echo -e "\nThe rootfs is now available as 2024-07-04-raspios-bookworm-arm64-lite.qcow2 - have fun ^-^"
