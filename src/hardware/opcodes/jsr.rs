use crate::{
    errors::VmError,
    hardware::{consts, vm::VM},
};

use super::utils;

/// Saves the RPC value on the R7, and then, depending on a flag, increments the pc in an offset, or sets the pc as the value of a base_reg
pub fn jsr(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // if 1, then use pc_offset, if 0
    let use_offset = (instr >> 11) & 1;

    // save the pc in R7
    vm.update_register_value(consts::RR7, vm.get_register_value(consts::RPC)?)?;

    if use_offset != 0 {
        // Inscreases pc in the value of the offset. Use casting to avoid overflow
        let pc_offset = utils::sign_extend(instr & 0x7ff, 11);
        let val: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
        vm.update_register_value(consts::RPC, val as u16)?;
    } else {
        // Updates pc with the value of the register.
        let base_reg = (instr >> 6) & 0x7;
        vm.update_register_value(consts::RPC, vm.get_register_value(base_reg)?)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::jsr;
    use crate::hardware::{vm::VM, consts};
    use crate::jmp::jmp;

    #[test]
    fn test_01() {
        // Jsr saves the pc value and then increments the pc in the passed offset
        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111; // 31
        jsr(instr, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RR7).unwrap());
        assert_eq!(47, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_02() {
        // Jsr saves the pc value and then increments the pc in the value inside the passed register
        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 8).unwrap();
        vm.update_register_value(consts::RR2, 40).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the value in the register'
        let instr: u16 = 0b0100000010000000;
        jsr(instr, &mut vm).unwrap();

        assert_eq!(8, vm.get_register_value(consts::RR7).unwrap());
        assert_eq!(40, vm.get_register_value(consts::RPC).unwrap());
    }
}
