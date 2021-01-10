use std::{io::{self, Read}, u16};

use crate::cpu::CPU;

const BIT_MASK_11: u16= 0x7ff;
const BIT_MASK_9: u16 = 0x1ff;
const BIT_MASK_6: u16 = 0x3f;
const BIT_MASK_5: u16 = 0x1f;
const BIT_MASK_3: u16 = 0x7;

#[derive(Clone, Copy)]
pub enum Instruction {
    BR(bool, bool, bool, u16), /* branch */
    ADD(u16, u16, bool, u16),  /* add  */
    LD(u16, u16),              /* load */
    ST(u16, u16),              /* store */
    JSR(bool, u16),            /* jump register */
    AND(u16, u16, bool, u16),  /* bitwise and */
    LDR(u16, u16, u16),        /* load register */
    STR(u16, u16, u16),        /* store register */
    RTI(),                     /* unused */
    NOT(u16, u16),             /* bitwise not */
    LDI(u16, u16),             /* load indirect */
    STI(u16, u16),             /* store indirect */
    JMP(u16),                  /* jump */
    RES(),                     /* reserved (unused) */
    LEA(u16, u16),             /* load effective address */
    TRAP(u16)                  /* execute trap */
}

#[inline]
fn sign_extend(x: u16, bit_count: u8) -> u16 {
    (((x << (16 - bit_count)) as i16) >> (16 - bit_count)) as u16
}


fn interupt(trap_code: u16, cpu: &mut CPU) {
    match trap_code {
        0x20 => { /* get character from keyboard, not echoed onto the terminal */
            let mut buf = [0; 1];
            io::stdin()
                .read_exact(&mut buf)
                .expect("unable to read from stdin");
            cpu.register_write(0, buf[0] as u16);
        },  
        0x21 => { /* output a character */
            print!("{}", char::from(cpu.register_read(0) as u8));
        },  
        0x22 => { /* output a word string */
            let mut addr = cpu.register_read(0);
            loop {
                let c = cpu.memory_read(addr) as u8;
                if c == 0 {break}
                print!("{}", char::from(c));
                addr += 1;
                
            }
        },
        0x23 => {  /* get character from keyboard, echoed onto the terminal */
            let mut buf = [0; 1];
            io::stdin()
                .read_exact(&mut buf)
                .expect("unable to read from stdin");
            print!("{}", char::from(buf[0] as u8));
            cpu.register_write(0, buf[0] as u16);
        },
        0x24 => {  /* output a byte string */
            let mut addr = cpu.register_read(0);
            loop {
                let c = cpu.memory_read(addr);
                let low = char::from((c & 0xff) as u8);
                print!("{}", low);
                let high = c >> 8;
                if high !=0 {
                    print!("{}", high);
                } else {
                    break
                }
                addr += 1;
            }
        },
        0x25 => {  /* halt the program */
            print!("HALT");
            cpu.is_finished = true;
        }
        _ => panic!("Unkown trap_code <{}>", trap_code)
    }
}


pub fn execute(inst: Instruction, cpu: &mut CPU) {
        match inst {
            Instruction::BR(n,z,p, addr) => {
                if (cpu.flags.zer && z) || 
                   (cpu.flags.neg && n) || 
                   (cpu.flags.pos && p) {
                    cpu.pc = cpu.pc.wrapping_add(sign_extend(addr, 9));
                   }
            },
            Instruction::ADD(dst, src, mode, imr) => {
                let src1 = cpu.register_read(src);
                if mode {
                    cpu.register_write(dst, src1 + sign_extend(imr, 5)); 
                } else {
                    let scr2 = imr & BIT_MASK_3;
                    cpu.register_write(dst, src1 + scr2);
                }
                cpu.update_flags(dst);
            },
            Instruction::LD(dst, addr) => {
                cpu.register_write(
                    dst, 
                    cpu.memory_read(
                        cpu.pc + sign_extend(addr, 9)
                    )
                );
                cpu.update_flags(dst);
            },
            Instruction::ST(src, addr) => {
                cpu.memory_write(
                    cpu.pc + sign_extend(addr, 9), 
                    cpu.register_read(src)
                );
            },
            Instruction::JSR(mode, addr) => {
                cpu.register_write(7, cpu.pc);
                if mode {
                    cpu.pc = cpu.register_read((addr >> 6) & BIT_MASK_3);
                } else {
                    cpu.pc = sign_extend(addr, 11);
                }
            },
            Instruction::AND(dst, src, mode, imr) => {
                let src1 = cpu.register_read(src);
                if mode {
                    cpu.register_write(dst, src1 & sign_extend(imr, 5)); 
                } else {
                    let scr2 = imr & BIT_MASK_3;
                    cpu.register_write(dst, src1 & scr2);
                }
                cpu.update_flags(dst);
            },
            Instruction::LDR(dst, src, addr) => {
                cpu.register_write(
                    dst, 
                    cpu.memory_read(
                        cpu.register_read(src) + sign_extend(addr, 6)
                    )
                );
                cpu.update_flags(dst);
            },
            Instruction::STR(src, base, addr) => {
                cpu.memory_write(
                    cpu.register_read(base) + sign_extend(addr, 6),
                    cpu.register_read(src));
            },
            Instruction::RTI() => {}
            Instruction::NOT(dst, src) => {
                cpu.register_write(dst, !cpu.register_read(src));
                cpu.update_flags(dst);
                
            },
            Instruction::LDI(dst, addr) => {
                cpu.register_write(
                    dst,
                    cpu.memory_read(
                        cpu.memory_read(
                            cpu.pc + sign_extend(addr, 9)
                        )
                    ) 
                );
                cpu.update_flags(dst);
            },
            Instruction::STI(src, addr) => {
                cpu.memory_write(
                    cpu.memory_read(cpu.pc + sign_extend(addr, 9)), 
                    cpu.register_read(src)
                );
            },
            Instruction::JMP(reg) => {
                cpu.pc = cpu.register_read(reg);
            },
            Instruction::RES() => {}
            Instruction::LEA(dst, addr) => {
                cpu.register_write(dst, cpu.pc + sign_extend(addr, 9));
                cpu.update_flags(dst);
            },
            Instruction::TRAP(trap_code) => {
                interupt(trap_code, cpu);
            },
        }
    }


/// Instructions are packed in a 16 bit (dword)
/// For example ADD instruction looks as follows:
pub fn parse(dword: u16) -> Instruction {
        // take instruction
        let opcode = dword >> 12;
        match opcode {
           0x0000 => Instruction::BR(
               (dword >> 11) & 0x1 == 1,
               (dword >> 12) & 0x1 == 1,
               (dword >> 13) & 0x1 == 1,
               dword & BIT_MASK_9
            ),
           0x0001 => Instruction::ADD(
               (dword >> 9) & BIT_MASK_3,
               (dword >> 6) & BIT_MASK_3,
               (dword >> 5) & 0x1 == 1,
               dword & BIT_MASK_5,
            ),
           0x0010 => Instruction::LD(
               (dword >> 9) & BIT_MASK_3,
               dword & BIT_MASK_9,
           ),
           0x0011 => Instruction::ST(
               (dword >> 9) & BIT_MASK_3,
               dword & BIT_MASK_9,
           ),
           0x0100 => Instruction::JSR(
               (dword >> 11) & 0x1 == 1,
                dword & BIT_MASK_11,
           ),
           0x0101 => Instruction::AND(
               (dword >> 9) & BIT_MASK_3,
               (dword >> 6) & BIT_MASK_3,
               (dword >> 5) & 0x1 == 1,
               dword & BIT_MASK_5,
           ),
           0x0110 => Instruction::LDR(
               (dword >> 9) & BIT_MASK_3,
               (dword >> 6) & BIT_MASK_3,
               dword & BIT_MASK_5,
           ),
           0x0111 => Instruction::STR(
               (dword >> 9) & BIT_MASK_3,
               (dword >> 6) & BIT_MASK_3,
               dword & BIT_MASK_6,
           ),
           0x1000 => Instruction::RTI(),
           0x1001 => Instruction::NOT(
               (dword >> 9) & BIT_MASK_3,
               (dword >> 6) & BIT_MASK_3,
           ),
           0x1010 => Instruction::LDI(
               (dword >> 9) & BIT_MASK_3,
               dword & BIT_MASK_9,
           ),
           0x1011 => Instruction::STI(
               (dword >> 9) & BIT_MASK_3,
               dword & BIT_MASK_9,
           ),
           0x1100 => Instruction::JMP(
               (dword >> 6) & BIT_MASK_3,
           ),
           0x1101 => Instruction::RES(),
           0x1110 => Instruction::LEA(
               (dword >> 9) & BIT_MASK_3,
               dword & BIT_MASK_9,
           ),
           0x1111 => Instruction::TRAP(
               dword & 0xff,
           ),
           _ => panic!("OPCODE <{}> does not match any known instruction.", opcode)
        }
    }