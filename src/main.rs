use std::env;
use std::io;
use std::fs;

#[derive(Default)]
struct Registers {
    r0: u16, // register 0
    r1: u16, // register 1
    r2: u16, // register 2
    r3: u16, // register 3
    r4: u16, // register 4
    r5: u16, // register 5
    r6: u16, // register 6
    r7: u16, // register 7
    pc: u16, // program counter
    cc: ConditionCode,
}

enum ConditionCode {
    P,
    Z,
    N
}

impl Default for ConditionCode {
    fn default() -> ConditionCode {
        ConditionCode::Z
    }
}

const PC_START: u16 = 0x3000;

fn load_file(memory: &mut [u16], path: &str) -> io::Result<u16> {
    let buffer = fs::read(path).unwrap();

    let mut words = {
        let step = 2;
        let iterator = buffer
            .iter()
            .skip(1)
            .step_by(step);
        buffer
            .iter()
            .step_by(step)
            .zip(iterator)
            .map(|(&b1, &b2)| u16::from_be_bytes([b1, b2]))
    };


    let origin = match words.next() {
        Some(origin) => origin,
        None => PC_START
    };
    
    let mut pointer = origin as usize;


    for word in words {
        memory[pointer] = word;
        pointer += 1;
    }

    println!("{:#06x}", memory[(origin) as usize]);

    Ok(origin)
}

fn mread(memory: &mut [u16], addr: u16) -> u16 {
    memory[addr as usize]
}

fn mwrite(memory: &mut [u16], addr: u16, val: u16) {
    memory[addr as usize] = val;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("lc3 [file] ...");
    }

    let mut memory = [0u16; u16::MAX as usize];
    let mut reg = Registers::default();

    reg.pc = match load_file(&mut memory, &args[1]) {
        Ok(origin) => origin,
        Err(_) => PC_START
    };

    let mut instr: u16;
    let mut op: u16;
    let mut running = true;

    while running {
        instr = mread(&mut memory, reg.pc);
        op = instr >> 12;

        match op {
            0b0000 => { // BR, branch

            },
            0b0001 => { // ADD, add

            },
            0b0010 => { // LD, load

            },
            0b0011 => { // ST, store

            },
            0b0100 => { // JSR, jump register

            },
            0b0101 => { // AND, bitwise and

            },
            0b0110 => { // LDR, load register

            },
            0b0111 => { // STR, store register

            },
            0b1000 => { // RTI, unused

            },
            0b1001 => { // NOT, bitwise not

            },
            0b1010 => { // LDI, load indirect

            },
            0b1011 => { // STI, store indirect

            },
            0b1100 => { // JMP, jump

            },
            0b1101 => { // RES, reserved

            },
            0b1110 => { // LEA, load effective address

            },
            0b1111 => { // TRAP, execute trap

            },
            _ => {}
        }

        reg.pc += 1;
        running = false; // remove this!
    }
}
