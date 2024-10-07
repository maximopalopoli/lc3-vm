use crate::hardware::{registers, vm::VM};

use super::utils;

pub fn jsr(instr: u16, vm: &mut VM) {
    vm.update_register_value(registers::RR7, vm.get_register_value(registers::RPC));
    let long_flag = (instr >> 11) & 1;
    if long_flag == 0 {
        let base_reg = (instr >> 6) & 0x7;
        vm.update_register_value(registers::RPC, vm.get_register_value(base_reg));
    } else {
        let extended_dir = utils::sign_extend(instr & 0x1FF, 11);
        let val: u32 = vm.get_register_value(registers::RPC) as u32 + extended_dir as u32;
        vm.update_register_value(registers::RPC, val as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::jsr;
    use crate::hardware::vm::VM;
    use crate::jmp::jmp;

    #[test]
    fn test_01() {
        // Jsr saves the pc value and then increments the pc in the passed offset
        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16);

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000010;
        jmp(jmp_instr, &mut vm);

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111; // 31
        jsr(instr, &mut vm);

        assert_eq!(16, vm.get_register_value(registers::RR7));
        assert_eq!(47, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_02() {
        // Jsr saves the pc value and then increments the pc in the value inside the passed register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 8);
        vm.update_register_value(registers::RR2, 40);

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm);

        // This means 'Save PC at R7 ad then increment it in the value in the register'
        let instr: u16 = 0b0100000010000000;
        jsr(instr, &mut vm);

        assert_eq!(8, vm.get_register_value(registers::RR7));
        assert_eq!(40, vm.get_register_value(registers::RPC));
    }
}
