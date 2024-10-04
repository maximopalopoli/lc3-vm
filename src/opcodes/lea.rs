use super::utils;
use crate::registers;

pub fn lea(instr: u16, regs: &mut [u16; 11]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let val: u32 = regs[registers::RPC as usize] as u32 + pc_offset as u32;

    // add pc_offset to the current PC, and put that direction in the destination register
    regs[dest_reg as usize] = val as u16;
    utils::update_flags(dest_reg, regs);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::lea;
    use crate::jmp::jmp;

    #[test]
    fn test_01() {
        // Lea puts in a destination register the sum between the PC register and an offset

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 16;

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000010;
        jmp(jmp_instr, &mut regs);

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b1110100000011111; // 31
        lea(instr, &mut regs);

        assert_eq!(47, regs[registers::RR4 as usize]);
    }
}
