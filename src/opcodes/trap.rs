use std::{io::{self, Read, Write}, process};

use crate::{mem_read, memory, registers};

use super::utils;

pub const TRAP_GETC: u16 = 0x20;    /* get character from keyboard, not echoed onto the terminal */
pub const TRAP_OUT: u16 = 0x21;     /* output a character */
pub const TRAP_PUTS: u16 = 0x22;    /* output a word string */
pub const TRAP_IN: u16 = 0x23;      /* get character from keyboard, echoed onto the terminal */
pub const TRAP_PUTSP: u16 = 0x24;   /* output a byte string */
pub const TRAP_HALT: u16 = 0x25;    /* halt the program */


pub fn trap(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // Set the Reg7 to the PC value
    regs[registers::RR7 as usize] = regs[registers::RPC as usize];

    match instr & 0xFF {
        TRAP_GETC => {
            //Read a single character from the keyboard. The character is not echoed onto the
            //console. Its ASCII code is copied into R0. The high eight bits of R0 are cleared.

            let mut buf: [u8;1] = [0;1];
            io::stdin().read_exact(&mut buf).unwrap();
            // Should handle unwrap

            regs[registers::RR0 as usize] = buf[0] as u16;
            utils::update_flags(registers::RR0, regs);
        }
        TRAP_OUT => {
            //Write a character in R0 to the console display.
            
            print!("{}", (regs[registers::RR0 as usize] as u8) as char);
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_PUTS => {
            // Write a string of ASCII characters to the console display.

            let mut index = regs[registers::RR0 as usize];
            let mut c = *mem_read(index, memory);

            // 0x0000 is a the NULL character equivalent 
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = *mem_read(index, memory);
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_IN => {
            //Print a prompt on the screen and read a single character from the keyboard. The
            //character is echoed onto the console monitor.
            
            println!("Enter a character: ");

            let mut buf: [u8;1] = [0;1];
            io::stdin().read_exact(&mut buf).unwrap();
            // Should handle unwrap

            let c = buf[0];
            print!("{}", c as char);
            io::stdout().flush().expect("failed to flush");

            regs[registers::RR0 as usize] = c as u16;
            utils::update_flags(registers::RR0, regs);
        }
        TRAP_PUTSP => {
            // Write a string of ASCII characters to the console in parts (first half, second half)

            let mut index = regs[registers::RR0 as usize];
            let mut c = *mem_read(index, memory);

            // 0x0000 is a the NULL character equivalent 
            while c != 0x0000 {
                let char_1 = c & 0xFF;
                print!("{}", (char_1 as u8) as char);
                let char_2 = c >> 8;
                if char_2 != 0x0000 {
                    print!("{}", (char_2 as u8) as char);
                }
                index += 1;
                c = *mem_read(index, memory);
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_HALT => {
            // Halts
            io::stdout().flush().expect("Failed to flush");
            process::exit(1);
        }
        _ => {
            // Should not reach this point
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    // ???
}

