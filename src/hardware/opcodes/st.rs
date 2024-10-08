use super::utils;
use crate::hardware::{registers, vm::VM};

pub fn st(instr: u16, vm: &mut VM) {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // Add the current PC to the PC offset to get the address where store the data
    let address: u32 = vm.get_register_value(registers::RPC) as u32 + pc_offset as u32;
    let address: u16 = address as u16;

    let value = vm.get_register_value(source_reg);

    vm.mem_write(address, value);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::st;
    use crate::hardware::vm::VM;
    use crate::ld::ld;

    #[test]
    fn test_01() {
        // st puts in the memory direction defined by the offset the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16);

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm);

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm);

        assert_eq!(16, vm.get_register_value(registers::RR3));
    }
}
