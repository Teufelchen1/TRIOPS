#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::io;
use std::sync::mpsc;

use clap::Parser;

use ratatui::{backend::CrosstermBackend, Terminal};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

mod ui;
use ui::ViewState;

mod periph;
use crate::periph::{UartBuffered, UartTty};

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

    #[arg(long, default_value_t = false)]
    headless: bool,

    #[arg(long, default_value_t = false)]
    testing: bool,
}

fn ui_loop(cpu: &mut CPU, uart_rx: &mpsc::Receiver<char>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let _ = terminal.clear();

    let mut ui = ViewState::new();
    let mut show_help = true;

    loop {
        terminal.draw(|f| ui.ui(f, cpu, uart_rx, show_help))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('h') => {
                    show_help = !show_help;
                }
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char('s') => {
                    if !cpu.step() {
                        break;
                    }
                }
                _ => todo!(),
            }
        }
    }

    let _ = terminal.clear();

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = std::path::PathBuf::from(args.file);
    let file_data = std::fs::read(path).unwrap();

    let mut cpu = CPU::default(&file_data);

    if args.headless {
        let tty = UartTty {};
        cpu.memory.periph.push(Box::new(tty));
        loop {
            if !cpu.step() {
                break;
            }
        }
    } else {
        let (tx, rx): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
        let buffered = UartBuffered { sink: tx };
        cpu.memory.periph.push(Box::new(buffered));
        return ui_loop(&mut cpu, &rx);
    }

    if args.testing {
        let reg = cpu.register.read(17);
        if reg != 93 {
            println!("Test failed: {:}", cpu.register.read(10));
        }
        anyhow::ensure!(cpu.register.read(17) == 93, "Test failed");
    } else {
        println!("Done!");
    }

    Ok(())
}
