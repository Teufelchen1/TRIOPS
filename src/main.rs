#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::io;

use clap::Parser;

use ratatui::{backend::CrosstermBackend, prelude::Backend, Terminal};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

use elf::abi;
use elf::endian::AnyEndian;
use elf::ElfBytes;

mod ui;
use ui::ViewState;

mod instructions;
use instructions::Instruction;

mod decoder;
use decoder::decode;

mod executer;
use executer::exec;

mod memory;
use memory::Memory;

mod register;
use register::Register;

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

fn ui_loop(cpu: &mut CPU) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let _ = terminal.clear();

    let mut ui = ViewState::new();

    loop {
        terminal.draw(|f| ui.ui(f, &cpu.register, &cpu.memory))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
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

    terminal.clear();

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
        loop {
            if !cpu.step() {
                break;
            }
        }
    } else {
        return ui_loop(&mut cpu);
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
