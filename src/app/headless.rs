use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::cli;
use crate::cpu::{create_cpu_thread, CPU};
use crate::events::{CpuJob, Event};
use crate::periph;

pub fn headless(config: &cli::Config) {
    let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) = channel();
    let (cpu_sender, cpu_reader): (Sender<CpuJob>, Receiver<CpuJob>) = channel();

    let tty = periph::new_stdio_uart(event_sender.clone());

    let tty = periph::new_unix_socket_uart(event_sender.clone());

    let cpu_val = {
        if config.bin {
            let entry = config.entryaddress;
            let baseaddress = config.baseaddress;
            CPU::from_bin(&config.file, tty, entry, baseaddress)
        } else {
            CPU::from_elf(&config.file, tty)
        }
    };

    let cpu = Arc::new(Mutex::new(cpu_val));
    create_cpu_thread(&Arc::clone(&cpu), event_sender, cpu_reader);

    cpu_sender.send(CpuJob::AutoStep).unwrap();

    while let Ok(event) = event_receiver.recv() {
        match event {
            Event::CpuStepComplete(continue_exec) => {
                if !continue_exec {
                    break;
                }
            }
            Event::CpuPanic(err) => {
                let cpu = cpu.lock().unwrap();
                println!("\nUnrecoverable error, last instructions:");
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
            Event::InterruptUart => {
                cpu_sender.send(CpuJob::CheckInterrupts).unwrap();
            }
            _ => (),
        }
    }

    if config.testing {
        let cpu = cpu.lock().unwrap();
        let reg = cpu.register.read(17);
        if reg != 93 {
            println!("Test failed: {:}", cpu.register.read(10));
        }
        assert!(cpu.register.read(17) == 93, "Test failed");
    } else {
        println!("Done!");
    }
}
