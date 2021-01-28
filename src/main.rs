#![allow(dead_code, unused)]

use std::env;
use std::io;
use std::fs::read;

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

fn load_file(path: &str, memory: &mut [u16]) -> io::Result<()> {
    let buffer = read(path).unwrap();
    let origin = u16::from_be_bytes([buffer[0], buffer[1]]);
    let mut pointer = origin as usize;

    let bytes = {
        let skip = 2;
        let step = 2;
        let iterator = buffer
            .iter()
            .skip(skip + 1)
            .step_by(step);
        buffer
            .iter()
            .skip(skip)
            .step_by(step)
            .zip(iterator)
            .map(|(&b1, &b2)| u16::from_be_bytes([b1, b2]))
    };

    for byte in bytes {
        memory[pointer] = byte;
        pointer += 1;
    }

    println!("{:#06x}", memory[0x3000]);

    Ok(())
}

fn main() {
    let mut memory = [0u16; u16::MAX as usize];
    let mut registers = [0u16; Register::COUNT as usize];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("lc3 [file] ...");
    }

    load_file(&args[1], &mut memory);

    let mut instr: u16;
    let mut op: u16;
    let mut running = true;
    while running {
        // read instr
        // read opcode
        running = false;
    }
}
