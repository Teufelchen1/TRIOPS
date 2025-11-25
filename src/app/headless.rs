use crate::hifive1b::Hifive1b;
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
    println!("Use ^D to terminate.");
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

pub fn headless(config: &cli::Config) {
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
    );
}

fn cpu_job_loop(
    config: &cli::Config,
    cpu: &Arc<Mutex<CPU<impl AddrBus + Send + 'static>>>,
    event_receiver: &Receiver<Event>,
    event_sender: Sender<Event>,
    cpu_reader: Receiver<CpuJob>,
    cpu_sender: &Sender<CpuJob>,
) {
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
            Event::InterruptUart => {
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
        if reg != 93 {
            println!("Test failed: {:}", cpu.register.read(10));
        }
        assert!(cpu.register.read(17) == 93, "Test failed");
    } else {
        println!("Done!");
    }
}
