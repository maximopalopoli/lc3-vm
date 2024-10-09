use crate::{errors::VmError, hardware::vm::VM};

use super::utils;

/// Depending on a flag, performs an and between two numbers or a number with an imm5 and puts the results of the operation in a destination register, and then update the flags
pub fn and(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // Get the secont arg from the five bits to extend and puts in the dest_reg the and operation result
        let imm5 = utils::sign_extend(instr & 0x1F, 5);
        vm.update_register_value(dest_reg, vm.get_register_value(sr1)? & imm5)?;
    } else {
        // Get the second reg from instr and puts in the dest_reg the and operation result
        let r2 = instr & 0x7;
        vm.update_register_value(
            dest_reg,
            vm.get_register_value(sr1)? & vm.get_register_value(r2)?,
        )?;
    }

    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hardware::consts;
    use crate::hardware::vm::VM;
    use super::and;

    #[test]
    fn test_01() {
        // Doing an and with two numbers in registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 2).unwrap();
        vm.update_register_value(consts::RR2, 3).unwrap();

        // This means 'Do an AND with RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0101011001000010;

        and(instr, &mut vm).unwrap();

        assert_eq!(2, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_02() {
        // Doing an and with one register number and an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 15).unwrap();

        // This means 'Do an AND with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0101011001100111;

        and(instr, &mut vm).unwrap();

        assert_eq!(7, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_03() {
        // Perform an and with a positive result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Do an and with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        and(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_POS,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_04() {
        // Perform an and with a zero result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Do an and with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001111111;

        and(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_ZRO,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }
}
