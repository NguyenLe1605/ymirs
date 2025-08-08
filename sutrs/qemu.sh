#!/usr/bin/bash

IMAGE_PATH="img"
BOOT_PATH="$IMAGE_PATH/efi/boot"
IMAGE="bootx64.efi"
OUT_PATH="${PWD}/../target/x86_64-unknown-uefi/debug"
VFAT_PATH="${OUT_PATH}/${IMAGE_PATH}"

cd $OUT_PATH
mkdir -p $BOOT_PATH
cp $IMAGE $BOOT_PATH

qemu-system-x86_64 -m 512M \
    -bios /usr/share/ovmf/OVMF.fd \
    -drive \
    file=fat:rw:${VFAT_PATH},format=raw \
    -nographic \
    -serial mon:stdio \
    -no-reboot \
    -enable-kvm \
    -cpu host \
    -s

