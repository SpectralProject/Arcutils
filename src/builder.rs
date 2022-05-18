use std::process::Command;

// ---------------
// CARGO CONSTANTS
// ---------------
const CARGO_COMMAND: &str = "cargo";
const RUSTC_COMMAND: &str = "rustc";
const RUSTC_TARGET_COMMAND: &str = "--target=";
const RUSTC_FLAG_COMMAND: &str = "--";
const STATICLIB_CRATE_TYPE: &str = "--crate-type=staticlib";

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

// build everything at once, arcboot and neutron
// by downloading arcboot and neutron source code and cargo barming them
pub fn full_build() {}
