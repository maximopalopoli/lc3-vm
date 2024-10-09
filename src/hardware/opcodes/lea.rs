use super::utils;
use crate::{
    errors::VmError,
    hardware::{registers, vm::VM},
};

// Loads in a destination register the sum between pc and an pc_offset, and then update the flags
pub fn lea(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let val: u32 = vm.get_register_value(registers::RPC)? as u32 + pc_offset as u32;

    // add pc_offset to the current PC, and put that direction in the destination register
    vm.update_register_value(dest_reg, val as u16)?;
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::lea;
    use crate::hardware::vm::VM;
    use crate::jmp::jmp;

    #[test]
    fn test_01() {
        // Lea puts in a destination register the sum between the PC register and an offset

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000010;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b1110100000011111; // 31
        lea(instr, &mut vm).unwrap();

        assert_eq!(47, vm.get_register_value(registers::RR4).unwrap());
    }
}
