use crate::cpu::{CPU, Register};

pub enum Instruction {
    BR,     /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP    /* execute trap */
}

impl Instruction {
    pub fn execute(cpu: &mut CPU, dword: u16) {
        let opcode = dword >> 12;
        match opcode {
            0x0001 => {
                
            }
        }
    }
}

fn add(dst: Register, src: Register, val: u16) {
    
}

fn load(dst: Register, src: Register, val: u16) {}

fn store(dst: Register, src: Register, val: u16) {}

fn and(dst: Register, src: Register) {}

fn not(cpu: &mut CPU, dst: Register, src: Register) {
    cpu.registers[dst as usize] = !cpu.registers[src as usize];
}

