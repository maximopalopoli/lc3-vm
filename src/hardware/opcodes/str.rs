use crate::hardware::vm::VM;

use super::utils;

pub fn str(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x3F, 6);

    // Add the content of the base register to the offset to get the address where store the data
    let address: u32 = vm.get_register_value(base_reg) as u32 + pc_offset as u32;
    let address: u16 = address as u16;
    
    let value = vm.get_register_value(dest_reg);

    vm.mem_write(address, value);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::str;
    use crate::hardware::vm::VM;
    use crate::ld::ld;

    #[test]
    fn test_01() {
        // str puts in the memory direction defined by the offset and the base register the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16);
        vm.update_register_value(registers::RR2, 57);

        // This means 'Put at (offset + reg value) direction of memory the content of the source register'
        let str_instr: u16 = 0b0111010001000001;
        str(str_instr, &mut vm);

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010001;
        ld(ld_instr, &mut vm);

        assert_eq!(57, vm.get_register_value(registers::RR3));
    } // This test is similar to the thing I would test with de load type instructions
}
