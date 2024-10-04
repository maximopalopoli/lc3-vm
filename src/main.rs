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

fn execute_instruction(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    let op: u16 = instr >> 12;

    match op {
        opcodes_values::OP_ADD => {
            add::add(instr, regs);
        }
        opcodes_values::OP_AND => {
            and::and(instr, regs);
        }
        opcodes_values::OP_NOT => {
            not::not(instr, regs);
        }
        opcodes_values::OP_BR => {
            br::br(instr, regs);
        }
        opcodes_values::OP_JMP => {
            jmp::jmp(instr, regs);
        }
        opcodes_values::OP_JSR => {
            jsr::jsr(instr, regs);
        }
        opcodes_values::OP_LD => {
            ld::ld(instr, regs, memory);
        }
        opcodes_values::OP_LDI => {
            ldi::ldi(instr, regs, memory);
        }
        opcodes_values::OP_LDR => {
            ldr::ldr(instr, regs, memory);
        }
        opcodes_values::OP_LEA => {
            lea::lea(instr, regs);
        }
        opcodes_values::OP_ST => {
            st::st(instr, regs, memory);
        }
        opcodes_values::OP_STI => {
            sti::sti(instr, regs, memory);
        }
        opcodes_values::OP_STR => {
            str::str(instr, regs, memory);
        }
        opcodes_values::OP_TRAP => {
            trap::trap(instr, regs, memory);
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

    println!("Regs: {}, and mem: {}", regs[registers::RPC as usize], memory::MEMORY_MAX);
    while (regs[registers::RPC as usize] as usize) < memory::MEMORY_MAX {

        // Read instruction
        let instruction = mem_read(regs[registers::RPC as usize], &mut memory);
/*         println!("Registers: {:?}", regs);
        println!("Memory: {:?}", &memory[regs[registers::RPC as usize] as usize]);
 */

        // Increment program counter
        regs[registers::RPC as usize] += 1;


        // Extract op_code and execute operation
        execute_instruction(instruction, &mut regs, &mut memory);

//        println!("{:?}", memory);
    }


    // Shutdown
}
