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
    #[arg(short, long)]
    file: String,

    /// If true, no TUI is started. Defaults to false.
    ///
    /// TRIOPS will run as fast as the CPU allows.
    /// The UART will be mapped to stdio.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    headless: bool,

    /// If true, the emulation result will be checked. Defaults to false.
    ///
    /// TRIOPS will probe the registers according to the riscv-software-src/riscv-tests.
    /// Their contents determine the return value. The checks are done after the emulation completed.
    /// Mainly used for CI.
    #[arg(long, default_value_t = false, verbatim_doc_comment)]
    testing: bool,
}

fn main() {
    let args = Args::parse();

    let path = std::path::PathBuf::from(args.file);
    let file_data = std::fs::read(&path).unwrap_or_else(|_| panic!("Could not read file {path:?}"));

    // Not headless? Start TUI!
    if !args.headless {
        let (tx, tui_reader): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
        let (tui_writer, rx): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
        let mut buffered = Uart::default(UartBuffered::new(rx, tx));
        let mut cpu = CPU::default(&file_data, &mut buffered);

        // Terminated TUI also terminates main()
        tui_loop(&mut cpu, &tui_reader, &tui_writer).expect("Well, your TUI crashed");
        return;
    }

    let mut tty = Uart::default(UartTty::new());
    let mut cpu = CPU::default(&file_data, &mut tty);

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
