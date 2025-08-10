target_dir := "target/"
img_dir := prepend(target_dir, "img/")
boot_dir := prepend(img_dir, "efi/boot")

sutrs_dir := "target/x86_64-unknown-uefi/debug/"
sutrs_bin := "bootx64.efi"
sutrs_bin_path := prepend(sutrs_dir, sutrs_bin)

ymirs_dir := "target/x86_64-unknown-none/debug/"
ymirs_bin := "ymirs"
ymirs_dest := prepend(ymirs_bin, ".elf")
ymirs_bin_path := prepend(ymirs_dir, ymirs_bin)
ymirs_boot_path := prepend(img_dir, ymirs_dest)

alias b := build
alias r := run
alias bs := build-sutrs
alias by := build-ymirs


run: build
    qemu-system-x86_64 -m 512M \
        -bios /usr/share/ovmf/OVMF.fd \
        -drive \
        file=fat:rw:{{img_dir}},format=raw \
        -nographic \
        -serial mon:stdio \
        -no-reboot \
        -enable-kvm \
        -cpu host \
        -s

build: build-sutrs build-ymirs

build-sutrs:
    cd sutrs && cargo b
    mkdir -p {{boot_dir}}
    cp {{sutrs_bin_path}} {{boot_dir}}

build-ymirs:
    cd ymirs && cargo b
    mkdir -p {{img_dir}}
    cp {{ymirs_bin_path}} {{ymirs_boot_path}}