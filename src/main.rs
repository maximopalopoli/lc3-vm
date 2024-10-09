pub mod errors;
pub mod hardware;

use errors::VmError;
use hardware::opcodes::opcodes_values;
use hardware::opcodes::*;
use hardware::vm::VM;
use std::{env, fs::File, io::BufReader};

extern crate termios;
use termios::*;

use byteorder::{BigEndian, ReadBytesExt};

fn execute_instruction(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    let op: u16 = instr >> 12;

    match op {
        opcodes_values::OP_ADD => {
            add::add(instr, vm)?;
        }
        opcodes_values::OP_AND => {
            and::and(instr, vm)?;
        }
        opcodes_values::OP_NOT => {
            not::not(instr, vm)?;
        }
        opcodes_values::OP_BR => {
            br::br(instr, vm)?;
        }
        opcodes_values::OP_JMP => {
            jmp::jmp(instr, vm)?;
        }
        opcodes_values::OP_JSR => {
            jsr::jsr(instr, vm)?;
        }
        opcodes_values::OP_LD => {
            ld::ld(instr, vm)?;
        }
        opcodes_values::OP_LDI => {
            ldi::ldi(instr, vm)?;
        }
        opcodes_values::OP_LDR => {
            ldr::ldr(instr, vm)?;
        }
        opcodes_values::OP_LEA => {
            lea::lea(instr, vm)?;
        }
        opcodes_values::OP_ST => {
            st::st(instr, vm)?;
        }
        opcodes_values::OP_STI => {
            sti::sti(instr, vm)?;
        }
        opcodes_values::OP_STR => {
            str::str(instr, vm)?;
        }
        opcodes_values::OP_TRAP => {
            trap::trap(instr, vm)?;
        }
        _ => {} // RTI and RES should not be used
    }

    Ok(())
}

fn execute_program(vm: &mut VM) -> Result<(), VmError> {
    while vm.get_register_value(hardware::registers::RPC)? < hardware::memory::MEMORY_MAX as u16 {
        let instruction = vm.mem_read(vm.get_register_value(hardware::registers::RPC)?)?;

        // Increase pc
        let current_pc = vm.get_register_value(hardware::registers::RPC)?;
        vm.update_register_value(hardware::registers::RPC, current_pc + 1)?;

        execute_instruction(instruction, vm)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lc3 [image-file1] ...\n");
        return;
    }

    // Termios set up
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).expect("Error initializing termios from stdin");

    let mut new_termios = termios;
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin, TCSANOW, &new_termios).expect("Error from termios when setting parameters");

    // File read
    let f = match File::open(args[1].clone()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening the file '{}': {}", args[1], e);
            return;
        }
    };
    let mut file = BufReader::new(f);

    let mut base_address = file
        .read_u16::<BigEndian>()
        .expect("Error reading the base address from the file");

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
                    return; // Could be a corrupted file
                }
                break;
            }
        }
    }

    // Execute program and check for errors
    if let Err(e) = execute_program(&mut vm) {
        match e {
            VmError::OutOfBoundsError => {
                println!(
                    "Error accesing registers: the number intended to access was out of bounds"
                );
                return;
            }
            VmError::KeyboardInputError(e) => {
                println!("Error while receiving input from keyboard: {}", e);
                return;
            }
        }
    }

    // Reset terminal settings
    tcsetattr(stdin, TCSANOW, &termios).expect("Error from termios when reseting parameters");
}
