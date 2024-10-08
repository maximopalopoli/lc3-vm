use super::utils;
use crate::errors::VmError;
use crate::hardware::registers;
use crate::hardware::vm::VM;

/// Loads in a destination register the value stored in pc plus an pc_offset, and then update the flags
pub fn ld(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // Read the value from the place where the memory above was computed
    let address: u32 = pc_offset as u32 + vm.get_register_value(registers::RPC)? as u32;
    let value = vm.mem_read(address as u16)?;

    vm.update_register_value(dest_reg, value)?;
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        hardware::{condition_flags, registers, vm::VM},
        ld::ld,
        st::st,
    };

    #[test]
    fn test_01() {
        // ld puts in the source register the content of the memory direction defined by the offset

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 31).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(31, vm.get_register_value(registers::RR3).unwrap());
        assert!(vm.get_register_value(registers::RCOND).unwrap() == condition_flags::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a negative value, ld sets negative flag on

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, u16::max_value())
            .unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(registers::RCOND).unwrap() == condition_flags::FL_NEG);
    }
}
