pub mod condition_flags;
pub mod memory;
pub mod opcodes;
pub mod registers;

use opcodes::opcodes_values;
use opcodes::*;
use std::env;

fn mem_read(param: u16) -> u16 {
    16
}

fn get_opcode(instr: u16) -> u16 {
    //let op: u16 = instr >> 12;
    0
}

fn read_image(arg: &str) -> bool {
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...\n");
        return;
        //exit(2);
    }

    for arg in args {
        if !read_image(&arg) {
            println!("failed to load image: {}", arg);
            return;
            // exit(1)
        }
    }

    //@{Setup}
    let memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

    let mut regs: [u16; 11] = [0; 11];
    let reg_ref = &mut regs[registers::RCOND as usize];
    *reg_ref = condition_flags::FL_ZRO;

    let running = true;
    while running {
        let instr: u16 = mem_read(*(regs.get_mut(registers::RPC as usize).unwrap()) + 1);

        let op: u16 = get_opcode(instr);

        match op {
            opcodes_values::OP_ADD => {
                // Add impl
                add::add(instr, &mut regs);
            }
            opcodes_values::OP_AND => {
                // And impl
            }
            opcodes_values::OP_NOT => {
                // Not impl
            }
            opcodes_values::OP_BR => {
                // Br impl
            }
            opcodes_values::OP_JMP => {
                // Jmp impl
            }
            opcodes_values::OP_JSR => {
                // Jsr impl
            }
            opcodes_values::OP_LD => {
                // Ld impl
            }
            opcodes_values::OP_LDI => {
                // Ld impl
            }
            opcodes_values::OP_LEA => {
                // Lea impl
            }
            opcodes_values::OP_ST => {
                // St impl
            }
            opcodes_values::OP_STI => {
                // Sti impl
            }
            opcodes_values::OP_STR => {
                // Str impl
            }
            opcodes_values::OP_TRAP => {
                // Trap impl
            }
            opcodes_values::OP_RTI => {
                // Rti impl - Should not be used
            }
            opcodes_values::OP_RES => {
                // Res impl - Should not be used
            }
            _ => {}
        }
    }

    // Shutdown
}
