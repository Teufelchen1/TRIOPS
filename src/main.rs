#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

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

use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use elf::parse::ParsingTable;
use elf::segment::ProgramHeader;
use elf::segment::SegmentTable;
use elf::ElfStream;
use elf::abi;

use comfy_table::{Cell, Table};

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

    let path = std::path::PathBuf::from(args.file);
    let file_data = std::fs::read(path).unwrap();
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).unwrap();
    
    let mut ram: Vec<u8> = vec!();

    for phdr in file.segments().unwrap() {
        if phdr.p_type == abi::PT_LOAD {
            println!("Addr: {:#X}, Size: {:#X}", phdr.p_paddr, phdr.p_filesz);
            let start = (phdr.p_paddr - 0x8000_0000) as usize;
            if start > ram.len() {
                ram.resize(start, 0);
            }
            ram.extend_from_slice(file.segment_data(&phdr).unwrap());
        }
    }

    let mut register_file: RegisterFile = RegisterFile::default();
    //let mut memory: Memory = Memory::default_ram(fs::read(args.file).unwrap());
    let mut memory: Memory = Memory::default_ram(ram);
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
                        if !exec(&mut register_file, &mut memory, &inst, true, true) {
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
