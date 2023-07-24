#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]

use std::env;
use std::fs;

mod decoder;
use decoder::{decode, Instruction};

mod executer;
use executer::exec;

mod system;
use system::{Memory, RegisterFile};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = {
        match args.len() {
            2 => args[1].parse().unwrap(),
            _ => {
                panic!("Usage: {:} FILE", args[0].parse::<String>().unwrap());
            }
        }
    };

    let mut register_file: RegisterFile = RegisterFile::default();
    let mut memory: Memory = Memory::default_ram(fs::read(file_name).unwrap());
    //register_file.write(2, (memory.ram_base + memory.ram.len()) as u32);
    //register_file.pc = memory.rom_base as u32;
    register_file.pc = memory.ram_base as u32;

    loop {
        let inst = decode(memory.read_word(register_file.pc as usize));
        let inst_ = decode(memory.read_word(register_file.pc as usize));
        println!("PC: 0x{:X} Instruction: {:?}", register_file.pc, inst);
        exec(&mut register_file, &mut memory, inst, true, false);

        if let Instruction::ECALL() = inst_ {
            break;
        }
        if let Instruction::EBREAK() = inst_ {
            break;
        }
    }
    println!("Done!");
}
