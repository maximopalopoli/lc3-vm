use crate::hardware::vm::VM;

use super::utils;

pub fn ldr(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x3F, 6);

    let val = vm.get_register_value(base_reg) as u32 + pc_offset as u32;
    let mem_value = vm.mem_read(val as u16);

    vm.update_register_value(dest_reg, mem_value);

    vm.update_flags(dest_reg);
}

#[cfg(test)]
mod tests {
    use crate::{hardware::{condition_flags, registers, vm::VM}, ldr::ldr, st::st};

    #[test]
    fn test_01() {
        // ldr puts in the source register the content in the memory address defined between the offset and the base register

        let mut vm = VM::new();

        vm.update_register_value(registers::RR1, 49);
        vm.update_register_value(registers::RR2, 16);

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000011111; // 31
        st(st_instr, &mut vm);

        // This means 'Put at source register the content of offset direction of memory + base register value'
        let ldr_instr: u16 = 0b0110011010001111;
        ldr(ldr_instr, &mut vm);

        assert_eq!(49, vm.get_register_value(registers::RR3));
        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a zero value, ldr sets zero flag on (values of memory and registers are initialized in zero)
        
        let mut vm = VM::new();

        // This means 'Put at source register the content of offset direction of memory + base register value'
        let ldr_instr: u16 = 0b0110001000000001;
        ldr(ldr_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_ZRO);
    }
}
