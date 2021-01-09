use std::u16;
use std::io::{Error, Read};
use std::fs::File;
use crate::instructions::INSTRUCTION;

const NUM_OF_REGISTERS: u8 = 9;

pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,  // Program counter
}

impl Register {
    pub fn from(n: u16) -> Self {
        match n {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            _ => unreachable!("Register with <{}> addr does not exist", n),
        }
    }
}


pub struct Flags {
    neg: bool, 
    zer: bool, 
    pos: bool
}

fn sign_extend(x: u16, bit_count: u8) -> u16 {
    if ((x >> (bit_count - 1)) & 1) == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}


pub struct CPU {
    pub registers: [u16; NUM_OF_REGISTERS as usize],
    pub memory: [u16; u16::MAX as usize],
    flags: Flags,
    instruction: Option<INSTRUCTION>,
    pub is_finished: bool,
}

impl CPU {
    pub fn initiate() -> Self {
        let mut cpu = CPU {
            registers: [0; NUM_OF_REGISTERS as usize],
            memory:  [0; u16::MAX as usize],
            flags: Flags {neg: false, zer: false, pos: false},
            instruction: None,
            is_finished: false,
        };
        cpu.registers[Register::PC as usize] = 0x3000; // a default address.
        cpu
    }


    fn update_flags(&mut self, reg_value: u16) {
        if reg_value == 0 {self.flags.pos = false; self.flags.neg = false; self.flags.zer = true}
        else if reg_value >> 15 == 1 {self.flags.pos = false; self.flags.neg = true; self.flags.zer = false}
        else {self.flags.pos = true; self.flags.neg = false; self.flags.zer = false}
    }


    pub fn load_instructions(&mut self, file: String) -> Result<(), Error> {
        let mut bytes = Vec::new();
        File::open(file)?.read_to_end(&mut bytes)?;
        // read bytes as u16
        let img = bytes
            .chunks(2)
            .map(|byte| ((byte[0] as u16) << 8) | byte[1] as u16)
            .collect::<Vec<u16>>();
        // read the first address
        let mut iteration = img.iter();
        let mut addr = match iteration.next() {
            Some(addr) => {self.registers[Register::PC as usize] = *addr; *addr as usize},
            None => panic!("Image is empty! (Least than 2 bytes)")
        };

        for val in iteration {
            self.memory[addr] = *val;
            addr += 1;
        }

        Ok(())
    }

    pub fn fetch_instruction(&self) {
        
    }

    pub fn execute(&self) {

    }

    pub fn terminate(&self) {

    }
}


