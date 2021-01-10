use std::u16;
use std::io::{Error, Read};
use std::fs::File;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::instructions::{self, Instruction};

const NUM_OF_REGISTERS: u16 = 8;
const MR_KBSR: u16 = 0xFE00; // keyboard status
const MR_KBDR: u16 = 0xFE02; // keyboard data

pub struct Flags {
    pub neg: bool, 
    pub zer: bool, 
    pub pos: bool
}

// fn check_key() -> bool {
//     const STDIN_FILENO: i32 = 0;

//     let mut readfds = FdSet::new();
//     readfds.insert(STDIN_FILENO);

//     match select(None, &mut readfds, None, None, &mut TimeVal::zero()) {
//         Ok(value) => value == 1,
//         Err(_) => false,
//     }
// }

pub struct CPU {
    registers: [u16; NUM_OF_REGISTERS as usize],
    memory: [u16; u16::MAX as usize],
    pub pc: u16,  // PROGRAMM COUNTER
    pub flags: Flags,
    instruction: Option<Instruction>,
    pub is_finished: bool,
}

impl CPU {
    pub fn initiate() -> Self {
        CPU {
            registers: [0; NUM_OF_REGISTERS as usize],
            memory:  [0; u16::MAX as usize],
            pc: 0x3000,
            flags: Flags {neg: false, zer: false, pos: false},
            instruction: None,
            is_finished: false,
        }
    }

    pub fn register_read(&self, reg: u16) -> u16 {
        if reg >= NUM_OF_REGISTERS {
            panic!("R{} does not exist! Bad register access", reg)
        } else {
            self.registers[reg as usize]
        }
    }

    pub fn register_write(&mut self, reg: u16, val: u16) {
        if reg >= NUM_OF_REGISTERS {
            panic!("R{} does not exist! Bad register access", reg)
        } else {
            self.registers[reg as usize] = val;
        }
    }

    pub fn memory_read(&self, addr: u16) -> u16 {
        // if addr == MR_KBSR {
        //     let device_state = DeviceState::new();
        //     let keys: Vec<Keycode> = device_state.get_keys();
        //     if false {
        //         let mut buf = [0;1];
        //         std::io::stdin().read_exact(&mut buf).unwrap();
        //         self.memory[MR_KBSR as usize] = 1 << 15;
        //         self.memory[MR_KBDR as usize] = buf[0] as u16;
        //     } else {
        //         self.memory[MR_KBSR as usize] = 0;
        //     }
        // }
        self.memory[addr as usize]
    }
    
    pub fn memory_write(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = value;
    }


    pub fn update_flags(&mut self, reg_value: u16) {
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
            Some(addr) => {self.pc = *addr; *addr as usize},
            None => panic!("Image is empty! (Least than 2 bytes)")
        };

        for val in iteration {
            self.memory[addr] = *val;
            addr += 1;
        }

        Ok(())
    }

    pub fn fetch_instruction(&mut self) {
        let dword = self.memory_read(self.pc);
        self.instruction = Some(instructions::parse(dword));
    }

    pub fn execute(&mut self) {
        let inst = match self.instruction {
            Some(i) => i,
            None => panic!("No instruction was loaded")
        };

        instructions::execute(inst, self);
        self.pc += 1;
    }

    pub fn terminate(&self) {

    }
}


