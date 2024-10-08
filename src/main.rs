pub mod hardware;

use hardware::opcodes::opcodes_values;
use hardware::opcodes::*;
use hardware::vm::VM;
use std::{env, fs::File, io::BufReader};

extern crate termios;

use termios::*;

use byteorder::{BigEndian, ReadBytesExt};

fn execute_instruction(instr: u16, vm: &mut VM) {
    let op: u16 = instr >> 12;

    match op {
        opcodes_values::OP_ADD => {
            add::add(instr, vm);
        }
        opcodes_values::OP_AND => {
            and::and(instr, vm);
        }
        opcodes_values::OP_NOT => {
            not::not(instr, vm);
        }
        opcodes_values::OP_BR => {
            br::br(instr, vm);
        }
        opcodes_values::OP_JMP => {
            jmp::jmp(instr, vm);
        }
        opcodes_values::OP_JSR => {
            jsr::jsr(instr, vm);
        }
        opcodes_values::OP_LD => {
            ld::ld(instr, vm);
        }
        opcodes_values::OP_LDI => {
            ldi::ldi(instr, vm);
        }
        opcodes_values::OP_LDR => {
            ldr::ldr(instr, vm);
        }
        opcodes_values::OP_LEA => {
            lea::lea(instr, vm);
        }
        opcodes_values::OP_ST => {
            st::st(instr, vm);
        }
        opcodes_values::OP_STI => {
            sti::sti(instr, vm);
        }
        opcodes_values::OP_STR => {
            str::str(instr, vm);
        }
        opcodes_values::OP_TRAP => {
            trap::trap(instr, vm);
        }
        _ => {} // RTI and RES should not be used
    }
}

fn execute_program(vm: &mut VM) {
    while vm.get_register_value(hardware::registers::RPC) < hardware::memory::MEMORY_MAX as u16 {
        let instruction = vm.mem_read(vm.get_register_value(hardware::registers::RPC));

        let current_pc = vm.get_register_value(hardware::registers::RPC);
        vm.update_register_value(hardware::registers::RPC, current_pc + 1);

        execute_instruction(instruction, vm);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...\n");
        return;
    }

    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios;
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin, TCSANOW, &new_termios).unwrap();

    let f = File::open(args[1].clone()).expect("couldn't open file");
    let mut file = BufReader::new(f);

    let mut base_address = file.read_u16::<BigEndian>().expect("error");

    let mut vm = VM::new();

    loop {
        match file.read_u16::<BigEndian>() {
            Ok(instruction) => {
                vm.mem_write(base_address, instruction);
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

    execute_program(&mut vm);

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
