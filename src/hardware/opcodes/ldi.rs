use super::utils;
use crate::errors::VmError;
use crate::hardware::consts;
use crate::hardware::vm::VM;

/// Loads in a destination register the value stored in the direction obtained by the sum of pc and pc_offset, and then update the flags
pub fn ldi(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // add pc_offset to the current PC, look at that memory location to get the final address
    let address_read = vm.mem_read(vm.get_register_value(consts::RPC)? + pc_offset)?;
    let value = vm.mem_read(address_read)?;

    vm.update_register_value(r0, value)?;
    vm.update_flags(r0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        hardware::{consts, vm::VM},
        ldi::ldi,
        st::st,
    };

    #[test]
    fn test_01() {
        // ldi puts in the source register the content in the memory address defined on the memory direction defined by the offset

        let mut vm = VM::new();

        vm.update_register_value(consts::RR1, 31).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st1_instr: u16 = 0b0011001000000001; // 1
        st(st1_instr, &mut vm).unwrap();

        vm.update_register_value(consts::RR2, 96).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st2_instr: u16 = 0b0011010000011111; // 31
        st(st2_instr, &mut vm).unwrap();

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm).unwrap();

        assert_eq!(96, vm.get_register_value(consts::RR3).unwrap());
        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a zero value, ldi sets zero flag on (values of memory are initialized in zero)

        let mut vm = VM::new();

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_ZRO);
    }
}
