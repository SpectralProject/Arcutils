## Arcutils

Utilities for creating and managing arcboot and neutron builds. Allows flashing and running of builds easily.

- install with `cargo install --git <arcutils http url>`

## Features

- create disk images of type GPT, partitioned with FAT and NeFS/BTRFS/NTFS/EXT4
- [flash](https://qemu.readthedocs.io/en/latest/tools/qemu-img.html) my arcboot bootloader onto the disk, in UEFI mode (/EFI/boot/...)
- flash any arm/riscv kernel (elf) images onto a BTRFS/DOS/EXT4 partition, including any extra stuff like the root partition, home partition, user partition, etc

NOTE:

- calls the `sh` program so ensure you have it in PATH

## Arcutils as a runner

It is possible to use `arcutils run` for Neutron and Arcboot. Just specify `runner = "arcutils run <args>"` in config.toml.

- wrappers around `qemu` and `cargo / rustc` to simplify the building, running, testing and debugging process

For debugging, simply runs `qemu -s -S` and `lldb` so ensure `lldb` is installed.

For testing, compiles repo as profile `test` and uses `arcutils run`.

You'll also need to specify extra options in `[package.arcutils.metadata]`.

## Arcutils by itself

MAKE SURE TO specify a `arcutils.yml` in the root of the dir.

Then `arcutils run | test | build | debug` should work well.
