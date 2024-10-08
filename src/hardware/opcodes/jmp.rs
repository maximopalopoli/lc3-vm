use crate::hardware::{registers::RPC, vm::VM};

pub fn jmp(instr: u16, vm: &mut VM) {
    let base_reg = (instr >> 6) & 0x7;
    vm.update_register_value(RPC, vm.get_register_value(base_reg));
}

#[cfg(test)]
mod tests {
    use crate::hardware::vm::VM;
    use crate::jsr::jsr;

    use super::super::super::registers;
    use super::jmp;

    #[test]
    fn test_01() {
        // Jump increments the pc in the passed register value

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16);

        // This means 'Increment PC in the content in the base register'
        let instr: u16 = 0b1100000001000000;
        jmp(instr, &mut vm);

        assert_eq!(16, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_02() {
        // Jump returns to the original pc value after a jsr

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 16);

        // This means 'Set PC in the content in the base register'
        let instr: u16 = 0b1100000001000000;
        jmp(instr, &mut vm);

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111;
        jsr(instr, &mut vm);

        // This means 'Set PC in the content in the RR7'
        let instr: u16 = 0b1100000111000000;
        jmp(instr, &mut vm);

        assert_eq!(16, vm.get_register_value(registers::RPC));
    }
}
