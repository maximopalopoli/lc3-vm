use super::utils;
use crate::hardware::{registers, vm::VM};

pub fn br(instr: u16, vm: &mut VM) {
    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    // If a flag is set and the br op has that flag activated, enters the block
    if cond_flag & vm.get_register_value(registers::RCOND) != 0 {
        // Set the PC to go to the extended PCoffset
        let val: u32 = vm.get_register_value(registers::RPC) as u32 + pc_offset as u32;
        vm.update_register_value(registers::RPC, val as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::br;
    use crate::add::add;
    use crate::and::and;
    use crate::hardware::condition_flags;
    use crate::hardware::vm::VM;

    #[test]
    fn test_01() {
        // Make an operation that lefts the zero flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 0);
        vm.update_register_value(registers::RR2, 0);

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_ZRO);

        // This means 'If last operation left flag zero, then increment PC in an PCoffset'
        let br_instr = 0b0000010001100000;
        br(br_instr, &mut vm);

        assert_eq!(96, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_02() {
        // Make an operation that lefts the positive flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 1);
        vm.update_register_value(registers::RR2, 4);

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_POS);

        // This means 'If last operation left flag positive, then increment PC in an PCoffset'
        let br_instr = 0b0000001001000001;
        br(br_instr, &mut vm);

        assert_eq!(65, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_03() {
        // Make an operation that lefts the negative flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 1);
        vm.update_register_value(registers::RR2, 4);

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let add_instr: u16 = 0b0001011001111110;
        add(add_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_NEG);

        // This means 'If last operation left flag negative, then increment PC in an PCoffset'
        let br_instr = 0b0000100001000011;
        br(br_instr, &mut vm);

        assert_eq!(67, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_04() {
        // Make an operation that lefts the negative or zero flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(registers::RR1, 162);
        vm.update_register_value(registers::RR2, 0);

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0101011001000010;
        and(add_instr, &mut vm);

        assert!(vm.get_register_value(registers::RCOND) == condition_flags::FL_ZRO);

        // This means 'If last operation left flag negative or zero, then increment PC in an PCoffset'
        let br_instr = 0b0000110001100001;
        br(br_instr, &mut vm);

        assert_eq!(97, vm.get_register_value(registers::RPC));
    }

    #[test]
    fn test_05() {
        // Make a conditional branch and verify the RPC has moved

        let mut vm = VM::new();

        // Set a value bc can be initialized with garbage
        vm.update_register_value(registers::RCOND, condition_flags::FL_POS);

        // This means 'Increment PC in an PCoffset, no matter what happened in last operation'
        let br_instr = 0b0000111011100001;
        br(br_instr, &mut vm);

        assert_eq!(225, vm.get_register_value(registers::RPC));
    }
}
