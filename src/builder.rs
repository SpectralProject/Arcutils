use std::{
    error::Error,
    process::{exit, Command, ExitStatus},
};

// ---------------
// CARGO CONSTANTS
// ---------------

const CARGO: &str = "cargo";
const RUSTC: &str = "rustc";
const RUSTC_TARGET: &str = "--target=";
const RUSTC_FLAG: &str = "--";
const OPTIMIZATION_PROFILE: &[&str] = &["--release", "--debug", "--test"];

// ---------------
// ARCH
// ---------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arch {
    Riscv64,
    AArch64,
    X86_64,
}

const RISCV64_UNKNOWN_ELF: &'static str = "riscv64-unknown-elf";
const AARCH64_NONE_ELF: &'static str = "aarch64-none-elf";

// ---------------
// COMMON
// ---------------

pub fn run_command(mut cmd: Command) -> Result<ExitStatus, ()> {
    let status = cmd.status();

    match status {
        Ok(s) => Ok(s),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
    }
}

// ---------------
// BUILD API
// ---------------

use const_format::concatcp;

const ASSEMBLER: &str = "as";
const LINKER: &str = "ld";

const RISCV64_AS: &str = concatcp!(RISCV64_UNKNOWN_ELF, ASSEMBLER);
const RISCV64_LD: &str = concatcp!(RISCV64_UNKNOWN_ELF, LINKER);

const AARCH64_AS: &str = concatcp!(AARCH64_NONE_ELF, ASSEMBLER);
const AARCH64_LD: &str = concatcp!(AARCH64_NONE_ELF, ASSEMBLER);

// builds are usually quite basic. Run may require QEMU support
pub fn basic_build(arch: Arch) -> std::process::Output {
    let mut CARGO_COMMAND = Command::new("cargo");

    match arch {
        Arch::Riscv64 => CARGO_COMMAND.arg("barm").output().expect("Could not build"),
        Arch::AArch64 => CARGO_COMMAND.arg("barm").output().expect("Could not build"),
        Arch::X86_64 => CARGO_COMMAND.arg("barm").output().expect("Could not build"),
    }
}

// build everything at once, arcboot and neutron
// by downloading arcboot and neutron source code and cargo barming them
pub fn full_build() {}

// ---------------
// RUN API
// ---------------

pub const QEMU_RISCV: &str = "qemu-system-riscv64";
pub const QEMU_AARCH64: &str = "qemu-system-aarch64";

pub fn create_vfat() {
    Command::new("mkfs -t vfat /dev/vdisk1")
        .output()
        .expect("Couldn't create VFAT. Is /dev/vdisk1 populated already?");
}

pub fn mount_vfat() {
    Command::new("mount")
        .arg("/dev/vdisk1 /mnt/disk.vfat")
        .output()
        .expect("Couldn't mount VFAT. Is /mnt/disk.vfat mounted already?");
}

// join ovmf into a firmware image (bin)
// note: could prob use concat! for some of these but too many brackets
// * I dont think this is needed rn, just pass them to QEMU separately
pub fn join_ovmf(arch: &str) {
    let dir = std::format!("build/{arch}/");

    Command::new("cat")
        .arg(dir.to_owned() + "qemu_ovmf_code.fd")
        .arg(dir.to_owned() + "qemu_ovmf_raw.fd")
        .arg("--output ".to_owned() + &dir + "qemu_ovmf.bin")
        .output()
        .expect("Couldn't create OVMF combined image");
}

// run arcboot on qemu with options
pub fn run_arcboot(arch: Arch) {
    match arch {
        Arch::Riscv64 => todo!(),
        Arch::AArch64 => {
            let mut QEMU_COMMAND = Command::new(QEMU_AARCH64);

            let code_ovmf = std::format!(
                "-drive if=pflash,format=raw,readonly={},file=build/aarch64/qemu_ovmf_code.fd",
                true
            );
            let vars_ovmf = std::format!(
                "-drive if=pflash,format=raw,readonly={},file=build/aarch64/qemu_ovmf_vars.fd",
                false
            );

            // assumes using a disk called vfat and qemu_ovmf.bin
            QEMU_COMMAND
                .arg("-machine virt")
                .arg("-cpu cortex-a72")
                .arg("-m 2048")
                .arg("-vga std")
                .arg("-serial stdio")
                .arg("-kernel build/arcboot")
                .arg("-drive file=fat:rsw:vdisk")
                .arg(code_ovmf)
                .arg(vars_ovmf)
                .output()
                .expect("Couldn't run QEMU");
        }
        Arch::X86_64 => todo!(),
    };
}
