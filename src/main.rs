#![allow(dead_code, unused)]

use std::env;
use std::mem;
use std::io;
use std::io::prelude::*;
use std::fs::File;

enum Register {
    R0,    // register 0
    R1,    // register 1
    R2,    // register 2
    R3,    // register 3
    R4,    // register 4
    R5,    // register 5
    R6,    // register 6
    R7,    // register 7
    PC,    // program counter
    CC,    // condition code
    COUNT, // number of registers
}

enum Opcode {
    BR,  // branch
    ADD, // add
    AND, // bitwise and
    NOT, // bitwise not
    LD,  // load
    ST,  // store
    LDI, // load indirect
    STI, // store indirect
    JMP, // jump
    JSR, // jump register
    LDR, // load register
    STR, // store register
    RTI, // unused
    RES, // reserved
    LEA, // load effective address
    TRAP // execute trap
}

enum ConditionCode {
    P = 1,
    Z = 1 << 1,
    N = 1 << 2
}

const PC_START: u16 = 0x3000;

fn read_image(path: &str) -> io::Result<()> {
    let mut file = File::open(&path)?;

    let origin = PC_START;
    let mut buffer: [u8; mem::size_of::<u16>()] = [0; mem::size_of::<u16>()];

    let n = file.read(&mut buffer[..])?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

fn main() {
    let mut memory: [u16; u16::MAX as usize] = [0; u16::MAX as usize];
    let mut registers: [u16; Register::COUNT as usize] = [0; Register::COUNT as usize];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("lc3 [image file] ...");
    }

    read_image(&args[1]);

    let mut instr: u16;
    let mut op: u16;
    let mut running = true;
    while running {
        // read instr
        // read opcode
    }

    /*
    println!("{}", Register::R7 as u8);
    println!("{:?}", registers);
    println!("{}", registers[Register::R7 as usize]);

    registers[Register::CC as usize] = (ConditionCode::N as u16);
    assert_eq!(registers[Register::CC as usize], ConditionCode::N as u16);
    */
}
