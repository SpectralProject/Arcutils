use std::env;
use std::process::{exit, Command};

pub mod bootimg;
pub mod builder;
pub mod readenv;

use builder::*;

// -------------
// HELP
// -------------

const HELP_MSG: &str = "
Commands
    help / ? - display this message

    qemu - build arcboot/neutron and run using qemu-system-<arch>
        [ --debug | --release ]
        --arch [ arm | riscv | x86 ] (default is arm)

    build - build arcboot/neutron
        --src (default is cwd)
        --output-dir (default is build/arcboot.app and build/arcutils.app)
        --arch [ arm | riscv | x86 ] (default is arm)

    debug - build arcboot in debug mode and attach gdb to the runner
        --runner [ qemu | hw ] (default is qemu)

    test - simply runs cargo test on arcboot for now
";

fn help() {
    print!("{}", HELP_MSG)
}

// -------------
// CONSTANTS
// -------------

const DEFAULT_ARCH: Arch = Arch::AArch64;

#[derive(PartialEq, Clone, Copy)]
enum BuildTarget {
    Neutron,
    Arcboot,
    Full,
}

// -------------
// MAIN
// -------------

fn main() {
    // SECTION 0: CHECKS

    let allowed_commands = ["build", "test", "run", "flash"];

    // Check what was run, either arcboot build | test | flash
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || !allowed_commands.contains(&&args[1].as_ref()) {
        help();
        exit(1);
    }

    // 1. check if --arcboot was specified
    // 2. if not, check if --neutron was specified
    let mut build_target: BuildTarget = if args.contains(&"--arcboot".to_string()) {
        BuildTarget::Arcboot
    } else if args.contains(&"--neutron".to_string()) {
        BuildTarget::Neutron
    } else if args.contains(&"--full".to_string()) {
        BuildTarget::Full
    } else {
        println!("No build target specified, assuming arcboot");
        BuildTarget::Arcboot
    };

    // collect arch, if not specified, use default arch
    let arch = if args.contains(&"x86".to_string()) {
        Arch::X86_64
    } else if args.contains(&"riscv".to_string()) {
        Arch::Riscv64
    } else if args.contains(&"arm".to_string()) {
        Arch::AArch64
    } else {
        DEFAULT_ARCH
    };

    /*
        OPTION A: BUILD THE KERNEL LIBRARY
    */

    // Mostly for testing purposes, kind of like a dry run build
    // Take the config file kernel.build and build it
    if args[1] == "build" {
        // 1. run cargo barm/rv/x86
        basic_build(arch);
    }

    /*
        OPTION B: ARCTEST
    */

    // Uses --features arcboot and runs it on qemu
    if args[1] == "test" {
        // *NOTE: will build the kernel in `arctest` mode with its own EFI stub and set println = UART instead of fd = 1 (usually the main console)

        Command::new("cargo")
            .arg("rustc")
            .arg("--features")
            .arg("arctest")
            .arg("--")
            .arg("--nocapture")
            .output()
            .expect("Couldn't run cargo with arctest");

        Command::new(QEMU_RISCV).arg("");
    }

    /*
        OPTION C: RUN A BUILT KERNEL IMAGE
    */

    // Run with either spectro/pi4b on QEMU using a prebuilt kernel .a and arcboot .o when specified with --full
    if args[1] == "run" {
        if build_target == BuildTarget::Arcboot {
            // * NOTE: no need to do this manually as QEMU does it for us
            // if fat doesnt already exist, create it
            if !std::path::Path::new("/dev/vdisk1").exists() {
                create_vfat();
            }
            // if fat isnt already mounted, mount it
            if !std::path::Path::new("/mnt/disk.vfat").exists() {
                mount_vfat();
            }

            // 2. just create a vdisk/ dir in cwd if not already there
            if !std::path::Path::new("vdisk").exists() {
                Command::new("mkdir")
                    .arg("vdisk")
                    .output()
                    .expect("Couldn't create a directory for VFAT");
            }

            // run with ovmf or u-boot if riscv
            match arch {
                Arch::Riscv64 => {
                    // ensure U boot img exists
                    if !std::path::Path::new("uboot.img").exists() {
                        eprintln!("Could not find uboot.img!");
                        exit(1);
                    }
                }
                Arch::AArch64 => {
                    // combine ovmf image
                    // join_ovmf("aarch64");
                    // build a standard aarch64 build into build/arcboot
                    basic_build(arch);
                    // run qemu
                    run_arcboot(arch);
                }
                Arch::X86_64 => todo!(),
            }
        }
    }

    /*
        OPTION D: FLASH ARCBOOT BL
    */

    // Flash arcboot bootloader for a certain arch, arm, riscv, x86 onto a clean GPT drive as a single partition
    // Package an arcboot .exe and neutron/quantii .exe and any .config ascii files in a dir and create an ISO, then flash onto the disk as two separate partitions
    if args[1] == "flash" {
        Command::new("flash");

        // OPTION 1: install arcboot bl by itself. Basically creates a FAT32 partition of around 200MB
        // then copies the files required by UEFI like /EFI/boot/* and any extra config files in the root dir

        // OPTION 2: Recommended for neutron systems. Packages arcboot bl and neutron lib into a single filesystem (FAT32). Arcboot bl still does the job of locating the neutron image on disk, but stored on the same FAT32 partition
        // easy to get started and up, the other stuff that neutron/quantii would use, e.g. the root filesystem and other /mnt/ disks can be other partitions on the same or different disks
    }
}
