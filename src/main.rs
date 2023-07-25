#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]

use std::env;
use std::fs;
use std::io;

use clap::Parser;

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod ui;
use ui::ViewState;

mod decoder;
use decoder::{decode, Instruction};

mod executer;
use executer::exec;

mod system;
use system::{Memory, RegisterFile};

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,

    /// Number of times to greet
    #[arg(long, default_value_t = false)]
    headless: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut register_file: RegisterFile = RegisterFile::default();
    let mut memory: Memory = Memory::default_ram(fs::read(args.file).unwrap());
    register_file.pc = u32::try_from(memory.ram_base).unwrap();

    if args.headless {
        loop {
            let inst = decode(memory.read_word(register_file.pc as usize)).unwrap();

            if !exec(&mut register_file, &mut memory, &inst, true, true) {
                break;
            }
        }
        anyhow::ensure!(register_file.read(17) == 93, "Test failed");
    } else {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear();

        let mut ui = ViewState::new();

        loop {
            terminal.draw(|f| ui.ui(f, &register_file, &memory))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('s') => {
                        let inst = decode(memory.read_word(register_file.pc as usize)).unwrap();
                        if !exec(&mut register_file, &mut memory, &inst, true, false) {
                            break;
                        }
                    }
                    _ => todo!(),
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }

    println!("\nDone!");
    Ok(())
}
