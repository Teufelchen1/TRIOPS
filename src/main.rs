use std::fs;

mod decoder;
use decoder::{decode, Instruction};

mod executer;
use executer::exec;

mod system;
use system::{RegisterFile, Memory};

fn main() {
    let mut register_file: RegisterFile = RegisterFile::default();
    let mut memory: Memory = Memory::default(fs::read("test.hex").unwrap()); // Memory { mem: [0; 4096], code: fs::read("test.hex").unwrap() };
    register_file.write(2, (memory.ram_base + memory.ram.len()) as u32);
    register_file.pc = memory.rom_base as u32;

    loop {
        let inst = decode(memory.read_word(register_file.pc as usize));
        let inst_ = decode(memory.read_word(register_file.pc as usize));
        //println!("PC: 0x{:X} Instruction: {:?}, {:}, {:}", register_file.pc, inst, register_file.read(14), register_file.read(14));
        exec(&mut register_file, &mut memory, inst);

        if let Instruction::ECALL() = inst_ {
            break;
        }
        if let Instruction::EBREAK() = inst_ {
            break;
        }
    }
    println!("Done!");
}
