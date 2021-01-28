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

enum OpCode {
    BR,  // branch
    ADD, // add
    LD,  // load
    ST,  // store
    JSR, // jump register
    AND, // bitwise and
    LDR, // load register
    STR, // store register
    RTI, // unused
    NOT, // bitwise not
    LDI, // load indirect
    STI, // store indirect
    JMP, // jump
    RES, // reserved
    LEA, // load effective address
    TRAP, // execute trap
    ERR,  // error (for vm only)
}

impl From<u16> for OpCode {
    fn from(n: u16) -> OpCode {
        match n {
            n if n == OpCode::BR as u16 => OpCode::BR,
            n if n == OpCode::ADD as u16 => OpCode::ADD,
            n if n == OpCode::LD as u16 => OpCode::LD,
            n if n == OpCode::ST as u16 => OpCode::ST,
            n if n == OpCode::JSR as u16 => OpCode::JSR,
            n if n == OpCode::AND as u16 => OpCode::AND,
            n if n == OpCode::LDR as u16 => OpCode::LDR,
            n if n == OpCode::STR as u16 => OpCode::STR,
            n if n == OpCode::RTI as u16 => OpCode::RTI,
            n if n == OpCode::NOT as u16 => OpCode::NOT,
            n if n == OpCode::LDI as u16 => OpCode::LDI,
            n if n == OpCode::STI as u16 => OpCode::STI,
            n if n == OpCode::JMP as u16 => OpCode::JMP,
            n if n == OpCode::RES as u16 => OpCode::RES,
            n if n == OpCode::LEA as u16 => OpCode::LEA,
            n if n == OpCode::TRAP as u16 => OpCode::TRAP,
            _ => OpCode::ERR,
        }
    }
}

const PC_START: u16 = 0x3000;

fn load_file(memory: &mut [u16], path: &str) -> io::Result<u16> {
    let buffer = fs::read(path).unwrap();
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

        match OpCode::from(op) {
            OpCode::BR => {},
            OpCode::ADD => {},
            OpCode::LD => {},
            OpCode::ST => {},
            OpCode::JSR => {},
            OpCode::AND => {},
            OpCode::LDR => {},
            OpCode::STR => {},
            OpCode::RTI => {},
            OpCode::NOT => {},
            OpCode::LDI => {},
            OpCode::STI => {},
            OpCode::JMP => {},
            OpCode::RES => {},
            OpCode::LEA => {},
            OpCode::TRAP => {},
            _ => {}
        }

        reg.pc += 1;
        running = false;
    }
}
