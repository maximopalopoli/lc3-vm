use super::utils;
use crate::{
    errors::VmError,
    hardware::{consts, vm::VM},
};

/// Puts in source register the value stored in an address obtained searching in the direction (pc + a pc_offset) of memory
pub fn sti(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // Add the pc to the offset to get the address where read
    let var_address: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
    let var_address: u16 = var_address as u16;

    let address = vm.mem_read(var_address)? as usize;

    // store the reg value to the adress read above
    let value = vm.get_register_value(source_reg)?;
    vm.mem_write(address as u16, value);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sti;
    use crate::hardware::{vm::VM, consts};
    use crate::ld::ld;
    use crate::st::st;

    #[test]
    fn test_01() {
        // sti puts in the memory direction placed in the memory direction defined by the offset the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();
        vm.update_register_value(consts::RR2, 47).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000011;
        st(st_instr, &mut vm).unwrap();

        // This means 'Find the offset direction of memory the direction where to put the content of the source register and do it'
        let sti_instr: u16 = 0b1011010000000011;
        sti(sti_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010000;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(47, vm.get_register_value(consts::RR3).unwrap());
    }
}
