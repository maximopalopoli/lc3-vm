use crate::{errors::VmError, hardware::vm::VM};

pub fn not(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    let val = !vm.get_register_value(base_reg)?;
    vm.update_register_value(dest_reg, val)?;

    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hardware::vm::VM;

    use super::super::super::registers;
    use super::not;

    #[test]
    fn test_01() {
        // Not puts in a destination register the result of the not operation on the base register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, u16::max_value())
            .unwrap();
        vm.update_register_value(registers::RR2, 5).unwrap();

        // This means 'Put in the destination register the result of the not operation on the base register'
        let instr: u16 = 0b1001010001111111;
        not(instr, &mut vm).unwrap();

        assert_eq!(0, vm.get_register_value(registers::RR2).unwrap());
    }
}
