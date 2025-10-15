#!/usr/bin/env bash
set -euo pipefail

if [ ! -d "$KERNEL_DIR" ]
then
    echo ">> Cloning the Repo ..."
    git clone --depth=1 --branch="rpi-${KERNEL_VERSION}.y" "https://github.com/raspberrypi/linux" "$KERNEL_DIR"
fi

for file_name in "Kconfig" "Makefile" "irq" "polling"
do
    ln -fs "$(realpath "kernel-patches/uio/$file_name")" "$KERNEL_DIR/drivers/uio/$file_name"
done
ln -fs "$(realpath "kernel-patches/dts/bcm2711-rpi-4-b.dts")" "$KERNEL_DIR/arch/arm/boot/dts/broadcom/bcm2711-rpi-4-b.dts"

echo ">> Resetting the Repo ..."

make bcm2711_defconfig
sed -i 's|CONFIG_LOCALVERSION="-v8"|CONFIG_LOCALVERSION="-Emily_UIO"|' "${KBUILD_OUTPUT}/.config"
sed -i "s|# CONFIG_PREEMPT_RT is not set|CONFIG_PREEMPT_RT=y\nCONFIG_RCU_BOOST=y\nCONFIG_RCU_BOOST_DELAY=500|"      "${KBUILD_OUTPUT}/.config"

sed -i "s|CONFIG_UIO=m|CONFIG_UIO=y|"                                  "${KBUILD_OUTPUT}/.config"
sed -i "s|CONFIG_IKCONFIG=m|CONFIG_IKCONFIG=y|"                        "${KBUILD_OUTPUT}/.config"
