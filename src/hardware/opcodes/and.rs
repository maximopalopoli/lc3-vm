use crate::hardware::vm::VM;

use super::utils;

pub fn and(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // The five bits that we need to extend
        let imm5 = utils::sign_extend(instr & 0x1F, 5);
        vm.update_register_value(dest_reg, vm.get_register_value(sr1) & imm5);
    } else {
        let r2 = instr & 0x7;
        vm.update_register_value(
            dest_reg,
            vm.get_register_value(sr1) & vm.get_register_value(r2),
        );
    }

    vm.update_flags(dest_reg);
}

#[cfg(test)]
mod tests {
    use crate::hardware::vm::VM;

    use super::super::super::registers;
    use super::and;

    #[test]
    fn test_01() {
        // Doing an add with two numbers in registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 2);
        vm.update_register_value(registers::RR2, 3);

        // This means 'Do an AND with RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0101011001000010;

        and(instr, &mut vm);

        assert_eq!(2, vm.get_register_value(registers::RR3));
    }

    #[test]
    fn test_02() {
        // Doing an add with one register number and an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 15);

        // This means 'Do an AND with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0101011001100111;

        and(instr, &mut vm);

        assert_eq!(7, vm.get_register_value(registers::RR3));
    }

    // Other tests: try an overflow, see flag updates
}
