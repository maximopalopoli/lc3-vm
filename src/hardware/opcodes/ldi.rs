use super::utils;
use crate::hardware::registers;
use crate::hardware::vm::VM;

pub fn ldi(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let address_read = vm.mem_read(vm.get_register_value(registers::RPC) + pc_offset);

    let value = vm.mem_read(address_read);

    // add pc_offset to the current PC, look at that memory location to get the final address
    vm.update_register_value(r0, value);
    vm.update_flags(r0);
}

#[cfg(test)]
mod tests {
    use crate::{hardware::{condition_flags, registers, vm::VM}, ldi::ldi, st::st};

    #[test]
    fn test_01() {
        // ldi puts in the source register the content in the memory address defined on the memory direction defined by the offset

        let mut vm = VM::new();

        vm.update_register_value(registers::RR1, 31);

        // This means 'Put at offset direction of memory the content of the source register'
        let st1_instr: u16 = 0b0011001000000001; // 1
        st(st1_instr, &mut vm);

        vm.update_register_value(registers::RR2, 96);

        // This means 'Put at offset direction of memory the content of the source register'
        let st2_instr: u16 = 0b0011010000011111; // 31
        st(st2_instr, &mut vm);

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm);

        assert_eq!(96, vm.get_register_value(registers::RR3));
        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a zero value, ldi sets zero flag on (values of memory are initialized in zero)
        
        let mut vm = VM::new();

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_ZRO);
    }
}
