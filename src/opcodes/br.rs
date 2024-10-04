use super::utils;
use crate::registers;

pub fn br(instr: u16, regs: &mut [u16; 11]) {
    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    // If a flag is set and the br op has that flag activated, enters the block
    if cond_flag & regs[registers::RCOND as usize] != 0 {
        // Set the PC to go to the extended PCoffset
        let val: u32 = regs[registers::RPC as usize] as u32 + pc_offset as u32;
        regs[registers::RPC as usize] = val as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::br;
    use crate::add::add;
    use crate::and::and;
    use crate::condition_flags;

    #[test]
    fn test_01() {
        // Make an operation that lefts the zero flag on, and then make a conditional branch

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 0;
        regs[registers::RR2 as usize] = 0;

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut regs);

        assert!(regs[registers::RCOND as usize] == condition_flags::FL_ZRO);

        // This means 'If last operation left flag zero, then increment PC in an PCoffset'
        let br_instr = 0b0000010001100000;
        br(br_instr, &mut regs);

        assert_eq!(96, regs[registers::RPC as usize]);
    }

    #[test]
    fn test_02() {
        // Make an operation that lefts the positive flag on, and then make a conditional branch

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 1;
        regs[registers::RR2 as usize] = 4;

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut regs);

        assert!(regs[registers::RCOND as usize] == condition_flags::FL_POS);

        // This means 'If last operation left flag positive, then increment PC in an PCoffset'
        let br_instr = 0b0000001001000001;
        br(br_instr, &mut regs);

        assert_eq!(65, regs[registers::RPC as usize]);
    }

    #[test]
    fn test_03() {
        // Make an operation that lefts the negative or zero flag on, and then make a conditional branch

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 162;
        regs[registers::RR2 as usize] = 0;

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0101011001000010;
        and(add_instr, &mut regs);

        assert!(regs[registers::RCOND as usize] == condition_flags::FL_ZRO);

        // This means 'If last operation left flag negative or zero, then increment PC in an PCoffset'
        let br_instr = 0b0000110001100001;
        br(br_instr, &mut regs);

        assert_eq!(97, regs[registers::RPC as usize]);
    }

    #[test]
    fn test_04() {
        // Make a conditional branch and verify the RPC has moved

        let mut regs: [u16; 11] = [0; 11];

        // Set a value bc can be initialized with garbage
        regs[registers::RCOND as usize] = condition_flags::FL_POS;

        // This means 'Increment PC in an PCoffset, no matter what happened in last operation'
        let br_instr = 0b0000111011100001;
        br(br_instr, &mut regs);

        assert_eq!(225, regs[registers::RPC as usize]);
    }

    // Can't try the negative flag because could not perform a negative operation (unsigned 16)
}
