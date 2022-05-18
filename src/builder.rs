use std::process::Command;

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

pub enum Arch {
    Riscv64,
    AArch64,
    X86_64,
}

const RISCV64_AS: &str = "riscv64-unknown-elf-as";
const RISCV64_LD: &str = "riscv64-unknown-elf-ld";

const AARCH64_AS: &str = "aarch64-none-elf-as";
const AARCH64_LD: &str = "aarch64-none-elf-ld";

// ---------------
// BUILD API
// ---------------

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
