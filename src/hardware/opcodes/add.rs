use crate::{errors::VmError, VM};

use super::utils;

/// Depending on a flag, adds two numbers or a number and an imm5 and puts the results of the operation in a destination register, and then update the flags
pub fn add(instr: u16, vm: &mut VM) -> Result<(), VmError> {
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
        let val: u32 = imm5 as u32 + vm.get_register_value(sr1)? as u32;
        vm.update_register_value(dest_reg, val as u16)?;
    } else {
        // Get the second reg from instr and puts in the dest_reg the and operation result
        let r2 = instr & 0x7;

        let val: u32 = vm.get_register_value(sr1)? as u32 + vm.get_register_value(r2)? as u32;

        vm.update_register_value(dest_reg, val as u16)?;
    }
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hardware::consts;
    use crate::hardware::vm::VM;
    use super::add;

    #[test]
    fn test_01() {
        // Adding two number with registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 1).unwrap();
        vm.update_register_value(consts::RR2, 1).unwrap();

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0001011001000010;

        add(instr, &mut vm).unwrap();

        assert_eq!(2, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_02() {
        // Adding one number with an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm).unwrap();

        assert_eq!(10, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_03() {
        // Adding with a positive result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_POS,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_04() {
        // Adding with a zero result lets turned on the zero flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100000;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_ZRO,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_05() {
        // Adding with a negative result lets turned on the negative flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001110000;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_NEG,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }
}
