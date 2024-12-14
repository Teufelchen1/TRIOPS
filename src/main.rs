//! This is TRIOPS entry point, where `main()` is located.
//! The scope of this file is:
//!  - The argument parsing and handling
//!  - The interactions with the filesystem
//!  - Setup and run the emulator
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use std::sync::mpsc;

use clap::Parser;

mod ui;
use ui::tui_loop;

mod periph;
use crate::periph::{Uart, UartBuffered, UartTty};

mod instructions;

mod decoder;

mod executer;

mod memory;

mod register;

mod cpu;
use cpu::CPU;

mod utils;

#[derive(Parser, Debug)]
struct Args {
    /// If set, no TUI is started.
    ///
    /// TRIOPS will run as fast as the CPU allows.
    /// The UART will be mapped to stdio.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    headless: bool,

    /// If set, the emulation result will be checked.
    ///
    /// TRIOPS will probe the registers according to the riscv-software-src/riscv-tests.
    /// Their contents determine the return value. The checks are done after the emulation completed.
    /// Mainly used for CI.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    testing: bool,

    /// If set, the provided file is treated as pure binary
    ///
    /// When used, the entry address and base address can also be set.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    bin: bool,

    /// The entry address, where execution is started / PC is set to.
    ///
    /// Can be in hex or decimal.
    #[arg(long, default_value_t = String::from("0x20000000"), requires("bin"))]
    entryaddress: String,

    /// The base address, where the bin file is loaded to. Must be in RAM or ROM.
    ///
    /// Can be in hex or decimal.
    #[arg(long, default_value_t = String::from("0x20000000"), requires("bin"))]
    baseaddress: String,

    /// Path to the file that should be executed in the emulator
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let path = args.file;
    let file_data = std::fs::read(&path).unwrap_or_else(|_| panic!("Could not read file {path:?}"));

    // Not headless? Start TUI!
    if !args.headless {
        let (tx, tui_reader): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
        let (tui_writer, rx): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
        let mut buffered = Uart::default(UartBuffered::new(rx, tx));
        let mut cpu = {
            if args.bin {
                let entry = usize_from_str(&args.entryaddress);
                let baseaddress = usize_from_str(&args.baseaddress);
                CPU::from_bin(&file_data, &mut buffered, entry, baseaddress)
            } else {
                CPU::from_elf(&file_data, &mut buffered)
            }
        };

        // Terminated TUI also terminates main()
        tui_loop(&mut cpu, &tui_reader, &tui_writer).expect("Well, your TUI crashed");
        return;
    }

    let mut tty = Uart::default(UartTty::new());
    let mut cpu = {
        if args.bin {
            let entry = usize_from_str(&args.entryaddress);
            let baseaddress = usize_from_str(&args.baseaddress);
            CPU::from_bin(&file_data, &mut tty, entry, baseaddress)
        } else {
            CPU::from_elf(&file_data, &mut tty)
        }
    };

    loop {
        let ok = match cpu.step() {
            Ok(ok) => ok,
            Err(err) => panic!(
                "{}",
                &format!(
                    "Failed to step at address 0x{:X}: {:}",
                    cpu.register.pc, err
                )
            ),
        };
        if !ok {
            break;
        }
    }

    if args.testing {
        let reg = cpu.register.read(17);
        if reg != 93 {
            println!("Test failed: {:}", cpu.register.read(10));
        }
        assert!(cpu.register.read(17) == 93, "Test failed");
    } else {
        println!("Done!");
    }
}

fn usize_from_str(text: &str) -> usize {
    if text.starts_with("0x") {
        usize::from_str_radix(text.trim_start_matches("0x"), 16).unwrap()
    } else {
        text.parse().unwrap()
    }
}
