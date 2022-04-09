## Arcutils

Utilities for creating and managing arcboot and neutron builds. Allows flashing and running of builds easily.

## Features

- create disk images of type GPT, partitioned with FAT and NeFS/BTRFS/NTFS/EXT4
- [flash](https://qemu.readthedocs.io/en/latest/tools/qemu-img.html) my arcboot bootloader onto the disk, in UEFI mode (/EFI/boot/...)
- flash any arm/riscv kernel (elf) images onto a BTRFS/DOS/EXT4 partition, including any extra stuff like the root partition, home partition, user partition, etc

NOTE:

- only works for `sh` shells. I dont really wanna add support for other shells rn
