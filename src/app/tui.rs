use crate::hifive1b::Hifive1b;
use crate::utils::map_to_unixsocket;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use crossterm::{
    event::MouseEventKind,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use super::ui::ViewState;
use crate::cli;
use crate::cpu::{create_cpu_thread, AddrBus, CPU};
use crate::events::{CpuJob, Event};

pub enum Job {
    Step(usize),
    AutoStepOn,
    AutoStepOff,
    ReadUart(String),
    Idle,
    Exit,
}

fn input_thread(sender: &Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key) => {
                sender.send(Event::TerminalKey(key)).unwrap();
            }
            crossterm::event::Event::Mouse(mouse) => {
                if matches!(
                    mouse.kind,
                    MouseEventKind::ScrollDown | MouseEventKind::ScrollUp
                ) {
                    sender.send(Event::TerminalMouse(mouse)).unwrap();
                }
            }
            crossterm::event::Event::Resize(_columns, _rows) => {
                sender.send(Event::TerminalResize).unwrap();
            }
            _ => (),
        }
    }
}

fn create_input_thread(sender: Sender<Event>) -> JoinHandle<()> {
    spawn(move || input_thread(&sender))
}

fn event_loop_tui<T: AddrBus>(
    input: &Receiver<Event>,
    cpu: &Arc<Mutex<CPU<T>>>,
    cpu_sender: &Sender<CpuJob>,
    uart_rx: &Receiver<u8>,
    uart_tx: &Sender<u8>,
) -> anyhow::Result<()> {
    let mut input_app = ViewState::new();

    // Why?
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let _ = terminal.clear();

    {
        let cpu = cpu.lock().unwrap();
        terminal.draw(|f| input_app.ui(f, &cpu, uart_rx))?;
    }

    loop {
        let job: Job = match input.recv_timeout(Duration::from_millis(10000)) {
            Ok(event) => match event {
                Event::TerminalKey(key) => input_app.on_key(key),
                Event::TerminalMouse(mouse) => input_app.on_mouse(mouse),
                Event::TerminalResize => Job::Idle,
                Event::ExitApp => Job::Exit,
                Event::CpuStepComplete(continue_exec) => {
                    if continue_exec {
                        Job::Idle
                    } else {
                        Job::Exit
                    }
                }
                Event::CpuPanic(err) => return Err(err),
                Event::Interrupt(_type) => {
                    cpu_sender.send(CpuJob::CheckInterrupts)?;
                    Job::Idle
                }
            },
            Err(_) => Job::Idle,
        };

        match job {
            Job::Idle => {}
            Job::Exit => break,
            Job::ReadUart(msg) => {
                for ch in msg.chars() {
                    uart_tx.send(ch as u8)?;
                }
            }
            Job::Step(num) => {
                cpu_sender.send(CpuJob::Step(num))?;
            }
            Job::AutoStepOn => {
                cpu_sender.send(CpuJob::AutoStep)?;
            }
            Job::AutoStepOff => {
                cpu_sender.send(CpuJob::Stop)?;
            }
        }

        {
            let cpu = cpu.lock().unwrap();
            terminal.draw(|f| input_app.ui(f, &cpu, uart_rx))?;
        }
    }

    let _ = terminal.clear();

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    let _ = terminal.clear();
    Ok(())
}

pub fn tui(config: &cli::Config) {
    let (_tx, mut tui_reader): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
    let (mut tui_writer, _rx): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();

    let (event_sender, event_reader): (Sender<Event>, Receiver<Event>) = channel();
    let (cpu_sender, cpu_reader): (Sender<CpuJob>, Receiver<CpuJob>) = channel();

    let mut hifive1b = Hifive1b::new(event_sender.clone());

    let uart0 = hifive1b.uart0channel.take().unwrap();
    if let Some(path) = &config.uart0 {
        map_to_unixsocket(uart0, path.clone());
    } else {
        let (uart_tx, uart_rx) = uart0;
        tui_reader = uart_rx;
        tui_writer = uart_tx;
    }

    let uart1 = hifive1b.uart1channel.take().unwrap();
    if let Some(path) = &config.uart1 {
        map_to_unixsocket(uart1, path.clone());
    }

    let memory_map = hifive1b.memory.take().unwrap();

    let cpu_val = {
        if config.bin {
            let entry = config.entryaddress;
            let baseaddress = config.baseaddress;
            CPU::from_bin(&config.file, memory_map, entry, baseaddress)
        } else {
            CPU::from_elf(&config.file, memory_map)
        }
    };

    let cpu = Arc::new(Mutex::new(cpu_val));

    create_input_thread(event_sender.clone());
    create_cpu_thread(&Arc::clone(&cpu), event_sender, cpu_reader);
    if let Err(e) = event_loop_tui(
        &event_reader,
        &Arc::clone(&cpu),
        &cpu_sender,
        &tui_reader,
        &tui_writer,
    ) {
        println!("{e}");
    }
}
