pub mod errors;
pub mod hardware;

use errors::VmError;
use hardware::opcodes;
use hardware::vm::VM;
use std::{env, fs::File, io::BufReader};

extern crate termios;
use termios::*;

use byteorder::{BigEndian, ReadBytesExt};

fn execute_instruction(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    let op: u16 = instr >> 12;

    match op {
        hardware::opcodes::OP_ADD => {
            opcodes::add(instr, vm)?;
        }
        hardware::opcodes::OP_AND => {
            opcodes::and(instr, vm)?;
        }
        hardware::opcodes::OP_NOT => {
            opcodes::not(instr, vm)?;
        }
        hardware::opcodes::OP_BR => {
            opcodes::br(instr, vm)?;
        }
        hardware::opcodes::OP_JMP => {
            opcodes::jmp(instr, vm)?;
        }
        hardware::opcodes::OP_JSR => {
            opcodes::jsr(instr, vm)?;
        }
        hardware::opcodes::OP_LD => {
            opcodes::ld(instr, vm)?;
        }
        hardware::opcodes::OP_LDI => {
            opcodes::ldi(instr, vm)?;
        }
        hardware::opcodes::OP_LDR => {
            opcodes::ldr(instr, vm)?;
        }
        hardware::opcodes::OP_LEA => {
            opcodes::lea(instr, vm)?;
        }
        hardware::opcodes::OP_ST => {
            opcodes::st(instr, vm)?;
        }
        hardware::opcodes::OP_STI => {
            opcodes::sti(instr, vm)?;
        }
        hardware::opcodes::OP_STR => {
            opcodes::str(instr, vm)?;
        }
        hardware::opcodes::OP_TRAP => {
            opcodes::trap(instr, vm)?;
        }
        _ => {} // RTI and RES should not be used
    }

    Ok(())
}

fn execute_program(vm: &mut VM) -> Result<(), VmError> {
    while vm.get_register_value(hardware::consts::RPC)? < hardware::consts::MEMORY_MAX as u16 {
        let instruction = vm.mem_read(vm.get_register_value(hardware::consts::RPC)?)?;

        // Increase pc
        let current_pc = vm.get_register_value(hardware::consts::RPC)?;
        vm.update_register_value(hardware::consts::RPC, current_pc + 1)?;

        execute_instruction(instruction, vm)?;
    }
    Ok(())
}

fn main() -> Result<(), VmError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(VmError::NotEnoughArguments);
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
            return Err(VmError::IncorrectFileNameError(args[1].clone(), e)) ;
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
                    return Err(VmError::BadFileError(e)); // Could be a corrupted file
                }
                break;
            }
        }
    }

    // Execute program
    execute_program(&mut vm)?;

    // Reset terminal settings
    tcsetattr(stdin, TCSANOW, &termios).expect("Error from termios when reseting parameters");

    Ok(())
}
