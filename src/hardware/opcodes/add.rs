use crate::VM;

use super::utils;

pub fn add(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // The five bits to extend
        let imm5 = utils::sign_extend(instr & 0x1F, 5);

        // I use casting to prevent an overflow.
        let val: u32 = imm5 as u32 + vm.get_register_value(sr1) as u32;
        vm.update_register_value(dest_reg, val as u16);
    } else {
        let r2 = instr & 0x7;

        let val: u32 = vm.get_register_value(sr1) as u32 + vm.get_register_value(r2) as u32;

        vm.update_register_value(dest_reg, val as u16);
    }

    vm.update_flags(dest_reg);
}

#[cfg(test)]
mod tests {
    use crate::hardware::condition_flags;
    use crate::hardware::vm::VM;

    use super::super::super::registers;
    use super::add;

    #[test]
    fn test_01() {
        // Adding two number with registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 1);
        vm.update_register_value(registers::RR2, 1);

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0001011001000010;

        add(instr, &mut vm);

        assert_eq!(2, vm.get_register_value(registers::RR3));
    }

    #[test]
    fn test_02() {
        // Adding one number with an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 3);

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm);

        assert_eq!(10, vm.get_register_value(registers::RR3));
    }

    #[test]
    fn test_03() {
        // Adding with a positive result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 3);

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm);

        assert_eq!(condition_flags::FL_POS, vm.get_register_value(registers::RCOND));
    }

    #[test]
    fn test_04() {
        // Adding with a zero result lets turned on the zero flag

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 0);

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100000;

        add(instr, &mut vm);

        assert_eq!(condition_flags::FL_ZRO, vm.get_register_value(registers::RCOND));
    }

    #[test]
    fn test_05() {
        // Adding with a negative result lets turned on the negative flag

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 0);

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001110000;

        add(instr, &mut vm);

        assert_eq!(condition_flags::FL_NEG, vm.get_register_value(registers::RCOND));
    }
}
