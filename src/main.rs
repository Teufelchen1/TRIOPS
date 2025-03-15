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

mod ui;
use ui::tui_loop;

mod periph;

mod instructions;

mod cli;

mod cpu;
use cpu::CPU;

fn main() {
    let config = cli::Config::parse();

    if config.headless {
        let mut tty = periph::new_stdio_uart();
        let mut cpu = {
            if config.bin {
                let entry = config.entryaddress;
                let baseaddress = config.baseaddress;
                CPU::from_bin(&config.file, &mut tty, entry, baseaddress)
            } else {
                CPU::from_elf(&config.file, &mut tty)
            }
        };

        loop {
            let ok = match cpu.step() {
                Ok(ok) => ok,
                Err(err) => {
                    println!("\nUnrecoverable error, last 5 instructions:");
                    for data in cpu.last_n_instructions(10).iter().flatten() {
                        let (addr, instruction) = data;
                        println!("0x{addr:08X}:{}", instruction.print());
                    }
                    panic!(
                        "\n{}",
                        &format!(
                            "Failed to step at address 0x{:08X}: {:}",
                            cpu.register.pc, err
                        )
                    )
                }
            };
            // if cpu.register.pc == 0x200112ba {
            //     println!("a0: {}, a1: 0x{:08X}", cpu.register.read(10),cpu.register.read(11));
            // }
            // print!("{:}", String::from_utf8_lossy(
            //     &Command::new("riscv64-elf-addr2line")
            //     .arg("-e")
            //     .arg("./default.elf")
            //     .arg(format!("0x{:08X}", cpu.register.pc))
            //     .output()
            //     .expect("failed to execute process").stdout));
            if !ok {
                break;
            }
        }

        if config.testing {
            let reg = cpu.register.read(17);
            if reg != 93 {
                println!("Test failed: {:}", cpu.register.read(10));
            }
            assert!(cpu.register.read(17) == 93, "Test failed");
        } else {
            println!("Done!");
        }
    } else {
        // Not headless? Start TUI!
        let (tx, tui_reader): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
        let (tui_writer, rx): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
        let mut buffered = periph::new_buffered_uart(rx, tx);
        let mut cpu = {
            if config.bin {
                let entry = config.entryaddress;
                let baseaddress = config.baseaddress;
                CPU::from_bin(&config.file, &mut buffered, entry, baseaddress)
            } else {
                CPU::from_elf(&config.file, &mut buffered)
            }
        };

        // Terminated TUI also terminates main()
        match tui_loop(&mut cpu, &tui_reader, &tui_writer) {
            Ok(()) => (),
            Err(err) => panic!("{err:}"),
        }
    }
}
