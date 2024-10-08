use std::{
    io::{self, Read, Write},
    process,
};

use crate::hardware::{registers, vm::VM};

pub const TRAP_GETC: u16 = 0x20;
pub const TRAP_OUT: u16 = 0x21;
pub const TRAP_PUTS: u16 = 0x22;
pub const TRAP_IN: u16 = 0x23;
pub const TRAP_PUTSP: u16 = 0x24;
pub const TRAP_HALT: u16 = 0x25;

pub fn trap(instr: u16, vm: &mut VM) {
    // Set the Reg7 to the PC value
    let pc_value = vm.get_register_value(registers::RPC);
    vm.update_register_value(registers::RR7, pc_value);

    match instr & 0xFF {
        TRAP_GETC => {
            //Read a single character from the keyboard. The character is not echoed onto the
            //console. Its ASCII code is copied into R0. The high eight bits of R0 are cleared.

            let mut buf = [0; 1];
            io::stdin().read_exact(&mut buf).unwrap();

            vm.update_register_value(registers::RR0, buf[0] as u16);
        }
        TRAP_OUT => {
            //Write a character in R0 to the console display.

            let c = vm.get_register_value(registers::RR0) as u8;
            print!("{}", c as char);
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_PUTS => {
            // Write a string of ASCII characters to the console display.

            let mut index = vm.get_register_value(registers::RR0);
            let mut c = vm.mem_read(index);

            // 0x0000 is a the NULL character equivalent
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = vm.mem_read(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_IN => {
            //Print a prompt on the screen and read a single character from the keyboard. The
            //character is echoed onto the console monitor.

            println!("Enter a character: ");

            let mut buf: [u8; 1] = [0; 1];
            io::stdin().read_exact(&mut buf).unwrap();

            let c = buf[0];
            print!("{}", c as char);
            io::stdout().flush().expect("failed to flush");

            vm.update_register_value(registers::RR0, c as u16);
            vm.update_flags(registers::RR0);
        }
        TRAP_PUTSP => {
            // Write a string of ASCII characters to the console in parts (first half, second half)

            let mut index = vm.get_register_value(registers::RR0);
            let mut c = vm.mem_read(index);

            // 0x0000 is a the NULL character equivalent
            while c != 0x0000 {
                let char_1 = c & 0xFF;
                print!("{}", (char_1 as u8) as char);
                let char_2 = c >> 8;
                if char_2 != 0x0000 {
                    print!("{}", (char_2 as u8) as char);
                }
                index += 1;
                c = vm.mem_read(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_HALT => {
            // Stop the program
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
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
    use crate::{
        hardware::{registers, vm::VM},
        jmp::jmp,
        trap::{trap, TRAP_OUT},
    };

    #[test]
    fn test_01() {
        //Check that the value of the PC is saved in R7
        let mut vm = VM::new();

        vm.update_register_value(registers::RR1, 16);

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm);

        trap(TRAP_OUT, &mut vm);

        assert_eq!(16, vm.get_register_value(registers::RR7));
    }

    // I imagine other tests, but for that cases i would have to mock i/o operations
}
