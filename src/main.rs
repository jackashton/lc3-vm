use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs;
use std::ops::{Index, IndexMut};

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

struct Registers([u16; Register::COUNT as usize]);

impl Index<Register> for Registers {
    type Output = u16;
    fn index(&self, i: Register) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl Index<u16> for Registers {
    type Output = u16;
    fn index(&self, i: u16) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, i: Register) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl IndexMut<u16> for Registers {
    fn index_mut(&mut self, i: u16) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

enum ConditionCode {
    N = 1 << 2, // Ob100
    Z = 1 << 1, // 0b010
    P = 1       // 0b001
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

    let origin = words.next().unwrap();
    let mut pointer = origin as usize;

    for word in words {
        memory[pointer] = word;
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

fn setcc(reg: &mut Registers, r: u16) {
    reg[Register::CC] = {
        if (reg[r] >> 15) == 1 {
            ConditionCode::N
        } else if reg[r] == 0 {
            ConditionCode::Z
        } else {
            ConditionCode::P
        }
    } as u16;
}

fn sext(x: u16, w: u16) -> u16 {
    if ((x >> (w - 1)) & 1) == 1 {
        x | (0xFFFF << w)
    } else {
        x
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("lc3 [file] ...");
    }

    let mut memory = [0u16; u16::MAX as usize];
    let mut reg = Registers([0u16; Register::COUNT as usize]);

    reg[Register::PC] = match load_file(&mut memory, &args[1]) {
        Ok(origin) => origin,
        Err(_) => PC_START
    };

    let mut instr: u16;
    let mut op: u16;
    let mut running = true;

    while running {
        instr = mread(&mut memory, reg[Register::PC]);
        reg[Register::PC] += 1;
        op = instr >> 12;

        match op {
            0b0000 => { // BR, branch
                let pc_offset = instr & 0x01FF;
                let cc = (instr >> 9) & 0x7;
                if (cc & reg[Register::CC]) > 0 {
                    reg[Register::PC] += sext(pc_offset, 9);
                }
            },
            0b0001 => { // ADD, add
                let dr = (instr >> 9) & 0x7;
                let sr1 = (instr >> 6) & 0x7;
                let imm = (instr >> 5) & 1;
                if imm == 1 {
                    let imm5 = instr & 0x001F;
                    reg[dr] = reg[sr1] + sext(imm5, 5);
                } else {
                    let sr2 = instr & 0x7;
                    reg[dr] = reg[sr1] + reg[sr2];
                }
                setcc(&mut reg, dr);
            },
            0b0010 => { // LD, load
                let dr = (instr >> 9) & 0x7;
                let pc_offset = instr & 0x01FF;
                reg[dr] = mread(&mut memory, reg[Register::PC] + sext(pc_offset, 9));
                setcc(&mut reg, dr);
            },
            0b0011 => { // ST, store
                let sr = (instr >> 9) & 0x7;
                let pc_offset = instr & 0x1FF;
                mwrite(&mut memory, reg[Register::PC] + sext(pc_offset, 9), reg[sr]);
            },
            0b0100 => { // JSR, jump register
                let flag = (instr >> 11) & 1;
                if flag == 0 {
                    let base_r = (instr >> 6) & 0x7;
                    reg[Register::PC] = reg[base_r];
                } else {
                    let pc_offset = instr & 0x7FF;
                    reg[Register::PC] += sext(pc_offset, 11);
                }
            },
            0b0101 => { // AND, bitwise and
                let dr = (instr >> 9) & 0x7;
                let sr1 = (instr >> 6) & 0x7;
                let imm = (instr >> 5) & 1;
                if imm == 1 {
                    let imm5 = instr & 0x001F;
                    reg[dr] = reg[sr1] & sext(imm5, 5);
                } else {
                    let sr2 = instr & 0x7;
                    reg[dr] = reg[sr1] & reg[sr2];
                }
                setcc(&mut reg, dr);
            },
            0b0110 => { // LDR, load register
                let dr = (instr >> 9) & 0x7;
                let base_r = (instr >> 6) & 0x7;
                let offset = instr & 0x3F;
                reg[dr] = mread(&mut memory, base_r + sext(offset, 6));
                setcc(&mut reg, dr);
            },
            0b0111 => { // STR, store register
                let sr = (instr >> 9) & 0x7;
                let base_r = (instr >> 6) & 0x7;
                let pc_offset = instr & 0x3F;
                mwrite(&mut memory, reg[base_r] + sext(pc_offset, 6), reg[sr]);
            },
            0b1000 => { // RTI, unused

            },
            0b1001 => { // NOT, bitwise not
                let dr = (instr >> 9) & 0x7;
                let sr = (instr >> 6) & 0x7;
                reg[dr] = !reg[sr];
                setcc(&mut reg, dr);
            },
            0b1010 => { // LDI, load indirect
                let dr = (instr >> 9) & 0x7;
                let pc_offset = instr & 0x01FF;
                let addr = mread(&mut memory, reg[Register::PC] + sext(pc_offset, 9));
                reg[dr] = mread(&mut memory, addr);
                setcc(&mut reg, dr);
            },
            0b1011 => { // STI, store indirect
                let sr = (instr >> 9) & 0x7;
                let pc_offset = instr & 0x1FF;
                let addr = mread(&mut memory, reg[Register::PC] + sext(pc_offset, 9));
                mwrite(&mut memory, addr, reg[sr]);
            },
            0b1100 => { // JMP, jump
                let base_r = (instr >> 6) & 0x7;
                reg[Register::PC] = reg[base_r];
            },
            0b1101 => { // RES, reserved

            },
            0b1110 => { // LEA, load effective address
                let dr = (instr >> 9) & 0x7;
                let pc_offset = 0x1FF;
                reg[dr] = reg[Register::PC] + sext(pc_offset, 9);
                setcc(&mut reg, dr);
            },
            0b1111 => { // TRAP, execute trap
                match instr & 0xFF {
                    0x20 => { // GETC, get character from keyboard. Not echoed in terminal
                        reg[Register::R0] = io::stdin()
                            .bytes()
                            .next()
                            .unwrap()
                            .expect("failed to read character")
                            .into();
                    },
                    0x21 => { // OUT, output character to terminal
                        print!("{}", (reg[Register::R0] as u8) as char);
                        io::stdout().flush().unwrap();
                    },
                    0x22 => { // PUTS, output null terminating string to terminal
                        let mut addr = reg[Register::R0];
                        let mut c = mread(&mut memory, addr);
                        while c != 0 {
                            print!("{}", (c as u8) as char);
                            addr += 1;
                            c = mread(&mut memory, addr);
                        }
                        io::stdout().flush().unwrap();
                    },
                    0x23 => { // IN, get character from keyboard. Echoed in terminal
                        print!("Enter a character: ");
                        let input = io::stdin()
                            .bytes()
                            .next()
                            .unwrap()
                            .expect("failed to read character");

                        print!("{}", input);
                        reg[Register::R0] = input as u16;
                    },
                    0x24 => { // PUTSP, same as PUTS but two characters per memory address
                        let mut addr = reg[Register::R0];
                        let mut c = mread(&mut memory, addr);
                        while c != 0 {
                            let char1 = ((c & 0xFF) as u8) as char;
                            let char2 = ((c >> 8) as u8) as char;
                            print!("{}{}", char1, char2);
                            addr += 1;
                            c = mread(&mut memory, addr);
                        }
                        io::stdout().flush().unwrap();
                    },
                    0x25 => { // HALT, halt program
                        running = false;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}
