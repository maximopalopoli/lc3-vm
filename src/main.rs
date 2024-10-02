pub mod memory;
pub mod registers;
pub mod opcodes;
pub mod condition_flags;

use std::env;

fn mem_read(param: u16) -> u16 {

    16
}

fn get_opcode(instr: u16) -> u16 {
    //let op: u16 = instr >> 12;
    0
}

fn read_image(arg: &String) -> bool {

    true
}


fn main () {
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
    pub static mut memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

    let mut regs: [u16; 11] = [0;11];
    let reg_ref = &mut regs[registers::RCOND as usize];
    *reg_ref = condition_flags::FL_ZRO;

    let running = true;
    while running {
        let instr: u16 = mem_read(*(regs.get_mut(registers::RPC as usize).unwrap()) + 1);

        let op: u16 = get_opcode(instr);

        match op {
            opcodes::OP_ADD => {
                // Add impl
            },
            opcodes::OP_AND => {
                // And impl
            },
            opcodes::OP_NOT => {
                // Not impl
            },
            opcodes::OP_BR => {
                // Br impl
            },
            opcodes::OP_JMP => {
                // Jmp impl
            },
            opcodes::OP_JSR => {
                // Jsr impl
            },
            opcodes::OP_LD => {
                // Ld impl
            },
            opcodes::OP_LDI => {
                // Ld impl
            },
            opcodes::OP_LEA => {
                // Lea impl
            },
            opcodes::OP_ST => {
                // St impl
            },
            opcodes::OP_STI => {
                // Sti impl
            },
            opcodes::OP_STR => {
                // Str impl
            },
            opcodes::OP_TRAP => {
                // Trap impl
            },
            opcodes::OP_RTI => {
                // Trap impl
            },
            _ => {

            }
        }
    }

    // Shutdown
}
