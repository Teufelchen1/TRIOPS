#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::io;

use clap::Parser;

use tui::{backend::CrosstermBackend, Terminal};

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
mod decoder_q0;
mod decoder_q1;
mod decoder_q2;
mod decoder_q3;
use decoder::decode;

mod executer;
use executer::exec;

mod system;
use system::{Memory, RegisterFile};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(long, default_value_t = false)]
    headless: bool,

    #[arg(long, default_value_t = false)]
    testing: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let path = std::path::PathBuf::from(args.file);
    let file_data = std::fs::read(path).unwrap();
    let slice = file_data.as_slice();
    let elffile = ElfBytes::<AnyEndian>::minimal_parse(slice).unwrap();

    let mut register_file: RegisterFile = RegisterFile::default();
    let mut memory: Memory = Memory::default_hifive();

    for phdr in elffile.segments().unwrap() {
        if phdr.p_type == abi::PT_LOAD {
            let mut addr = usize::try_from(phdr.p_paddr).unwrap();
            if memory.is_rom(addr) {
                for i in elffile.segment_data(&phdr).unwrap() {
                    memory.rom[addr - memory.rom_base] = *i;
                    addr += 1;
                }
            } else if memory.is_ram(addr) {
                for i in elffile.segment_data(&phdr).unwrap() {
                    memory.ram[addr - memory.ram_base] = *i;
                    addr += 1;
                }
            }
        }
    }

    register_file.pc = u32::try_from(elffile.ehdr.e_entry).unwrap();

    if args.headless {
        loop {
            let inst = decode(memory.read_word(register_file.pc as usize)).unwrap();

            if !exec(&mut register_file, &mut memory, &inst, true, true) {
                break;
            }
        }
    } else {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let _ = terminal.clear();

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

    if args.testing {
        anyhow::ensure!(register_file.read(17) == 93, "Test failed");
    } else {
        println!("Done!");
    }

    Ok(())
}
