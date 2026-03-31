use crate::hifive1b::Hifive1b;
use crate::instructions::Instruction;
use crate::utils::map_to_unixsocket;
use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::thread::JoinHandle;

use crate::cli;
use crate::cpu::{create_cpu_thread, AddrBus, CPU};
use crate::events::{CpuJob, Event};

fn input_thread(sender: &Sender<Event>, output: Option<&Sender<u8>>) {
    let mut buffer = [0; 1];
    while let Ok(size) = io::stdin().read(&mut buffer) {
        if size == 0 {
            break;
        }
        if let Some(output) = output {
            output.send(buffer[0]).unwrap();
        }
    }
    sender.send(Event::ExitApp).unwrap();
}

fn create_input_thread(sender: Sender<Event>, output: Option<Sender<u8>>) -> JoinHandle<()> {
    spawn(move || input_thread(&sender, output.as_ref()))
}

pub fn headless(config: &cli::Config) -> anyhow::Result<()> {
    if !config.testing {
        println!("Use ^D to terminate.");
    }
    let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) = channel();
    let (cpu_sender, cpu_reader): (Sender<CpuJob>, Receiver<CpuJob>) = channel();

    let mut hifive1b = Hifive1b::new(event_sender.clone());

    let uart0 = hifive1b.uart0channel.take().unwrap();
    if let Some(path) = &config.uart0 {
        map_to_unixsocket(uart0, path.clone());
        create_input_thread(event_sender.clone(), None);
    } else {
        let (uart_tx, uart_rx) = uart0;
        create_input_thread(event_sender.clone(), Some(uart_tx));
        spawn(move || loop {
            while let Ok(data) = uart_rx.recv() {
                print!("{:}", data as char);
            }
        });
    }

    let uart1 = hifive1b.uart1channel.take().unwrap();
    if let Some(path) = &config.uart1 {
        map_to_unixsocket(uart1, path.clone());
    }

    let memory_map = hifive1b.memory.take().unwrap();

    let cpu_val = if config.bin {
        let entry = config.entryaddress;
        let baseaddress = config.baseaddress;
        CPU::from_bin(&config.file, memory_map, entry, baseaddress)
    } else {
        CPU::from_elf(&config.file, memory_map)
    };
    let cpu = Arc::new(Mutex::new(cpu_val));

    cpu_job_loop(
        config,
        &cpu,
        &event_receiver,
        event_sender,
        cpu_reader,
        &cpu_sender,
    )
}

fn cpu_job_loop(
    config: &cli::Config,
    cpu: &Arc<Mutex<CPU<impl AddrBus + Send + 'static>>>,
    event_receiver: &Receiver<Event>,
    event_sender: Sender<Event>,
    cpu_reader: Receiver<CpuJob>,
    cpu_sender: &Sender<CpuJob>,
) -> anyhow::Result<()> {
    create_cpu_thread(&Arc::clone(cpu), event_sender, cpu_reader);

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
            Event::CpuObserved(addr, inst) => {
                match inst {
                    Instruction::JAL(_, immediate) | Instruction::CJAL(immediate) => {
                        let destination = if immediate.is_negative() {
                            addr.wrapping_sub(immediate.unsigned_abs() as usize)
                        } else {
                            addr.wrapping_add(immediate.unsigned_abs() as usize)
                        };

                        if let Some(ref addr2line) = config.addr2line {
                            if let Ok(Some(location)) = addr2line.find_location(destination as u64)
                            {
                                let file = location.file.unwrap_or("???");
                                let line = location.line.unwrap_or(0);
                                println!("cJAL  0x{addr:x}->0x{destination:x}:{file}:{line}");
                            } else {
                                println!("cJAL to 0x{destination:x}");
                            }
                        }
                    }
                    Instruction::CJALR(rs) => {
                        let destination = {
                            let cpu = cpu.lock().unwrap();
                            cpu.register.read(rs)
                        };

                        if let Some(ref addr2line) = config.addr2line {
                            if let Ok(Some(location)) = addr2line.find_location(destination as u64)
                            {
                                let file = location.file.unwrap_or("???");
                                let line = location.line.unwrap_or(0);
                                println!("CJALR 0x{addr:x}->0x{destination:x}:{file}:{line}");
                            } else {
                                println!("CJALR({rs}) 0x{addr:x}->0x{destination:x}");
                            }
                        }
                    }
                    Instruction::JALR(_, rs, immediate) => {
                        let addr = {
                            let cpu = cpu.lock().unwrap();
                            cpu.register.read(rs)
                        };
                        let destination = if immediate.is_negative() {
                            addr.wrapping_sub(immediate.unsigned_abs())
                        } else {
                            addr.wrapping_add(immediate.unsigned_abs())
                        };

                        if let Some(ref addr2line) = config.addr2line {
                            if let Ok(Some(location)) = addr2line.find_location(destination as u64)
                            {
                                let file = location.file.unwrap_or("???");
                                let line = location.line.unwrap_or(0);
                                println!(" JALR 0x{addr:x}->0x{destination:x}:{file}:{line}");
                            } else {
                                println!(" JALR to 0x{destination:x}");
                            }
                        }
                    }
                    _ => (),
                }
            }
            Event::Interrupt(_type) => {
                cpu_sender.send(CpuJob::CheckInterrupts).unwrap();
            }
            Event::ExitApp => {
                break;
            }
            _ => (),
        }
    }

    if config.testing {
        let cpu = cpu.lock().unwrap();
        let reg = cpu.register.read(17);
        anyhow::ensure!(reg == 93);
    } else {
        println!("Done!");
    }
    Ok(())
}
