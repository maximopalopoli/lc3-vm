pub mod condition_flags;
pub mod memory;
pub mod opcodes;
pub mod registers;

use opcodes::opcodes_values;
use opcodes::*;
use std::io::Read;
use std::{env, fs::File, io::BufReader};

use byteorder::{BigEndian, ReadBytesExt};


fn mem_write(address: u16, memory: &mut [u16; memory::MEMORY_MAX], value: u16) {
    memory[address as usize] = value;
}

fn mem_read(address: u16, memory: &mut [u16; memory::MEMORY_MAX]) -> u16 {
    if address == memory::MR_KBSR as u16 {
        handle_keyboard(memory);
    }
    memory[address as usize]
}

fn handle_keyboard(memory: &mut [u16; memory::MEMORY_MAX]) {
    let mut buffer = [0; 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    if buffer[0] != 0 {
        mem_write(memory::MR_KBSR,memory, 1 << 15);
        mem_write(memory::MR_KBDR,memory, buffer[0] as u16);
    } else {
        mem_write(memory::MR_KBSR,memory, 0)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...\n");
        return;
        //exit(2);
    }

    let f = File::open(args[1].clone()).expect("couldn't open file");
    let mut file = BufReader::new(f);

    // Note how we're using `read_u16` _and_ BigEndian to read the binary file.
    let mut base_address = file.read_u16::<BigEndian>().expect("error");

    //@{Setup}
    let mut memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

    loop {
        match file.read_u16::<BigEndian>() {
            Ok(instruction) => {
                mem_write(base_address, &mut memory, instruction);
                base_address += 1;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("OK")
                } else {
                    println!("failed: {}", e);
                }
                break;
            }
        }
    }

    let mut regs: [u16; 11] = [0; 11];

    // Set as Zero bc can be initialized with garbage
    regs[registers::RCOND as usize] = condition_flags::FL_ZRO;
    regs[registers::RPC as usize] = registers::PC_START;

    let running = true;
    while running {
        let instr: u16 = mem_read(
            *(regs.get_mut(registers::RPC as usize).unwrap()) + 1,
            &mut memory,
        );

        let op: u16 = instr >> 12;

        match op {
            opcodes_values::OP_ADD => {
                // Add impl
                add::add(instr, &mut regs);
            }
            opcodes_values::OP_AND => {
                // And impl
                and::and(instr, &mut regs);
            }
            opcodes_values::OP_NOT => {
                // Not impl
                not::not(instr, &mut regs);
            }
            opcodes_values::OP_BR => {
                // Br impl
                br::br(instr, &mut regs);
            }
            opcodes_values::OP_JMP => {
                // Jmp impl
                jmp::jmp(instr, &mut regs);
            }
            opcodes_values::OP_JSR => {
                // Jsr impl
                jsr::jsr(instr, &mut regs);
            }
            opcodes_values::OP_LD => {
                // Ld impl
                ld::ld(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_LDI => {
                ldi::ldi(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_LDR => {
                ldr::ldr(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_LEA => {
                // Lea impl
                lea::lea(instr, &mut regs);
            }
            opcodes_values::OP_ST => {
                // St impl
                st::st(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_STI => {
                // Sti impl
                sti::sti(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_STR => {
                // Str impl
                str::str(instr, &mut regs, &mut memory);
            }
            opcodes_values::OP_TRAP => {
                // Trap impl
                trap::trap(instr, &mut regs, &mut memory);
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
