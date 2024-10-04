use crate::registers;

use super::utils;

pub fn jsr(instr: u16, regs: &mut [u16; 11]) {
    regs[registers::RR7 as usize] = regs[registers::RPC as usize];
    let long_flag = (instr >> 11) & 1;
    if long_flag == 0 {
        let base_reg = (instr >> 6) & 0x7;
        regs[registers::RPC as usize] = regs[base_reg as usize];
    } else {
        let extended_dir = utils::sign_extend(instr & 0x1FF, 11);
        let val: u32 = regs[registers::RPC as usize] as u32 + extended_dir as u32;
        regs[registers::RPC as usize] = val as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::jsr;
    use crate::jmp::jmp;

    #[test]
    fn test_01() {
        // Jsr saves the pc value and then increments the pc in the passed offset

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 16;

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000010;
        jmp(jmp_instr, &mut regs);

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111; // 31
        jsr(instr, &mut regs);

        assert_eq!(16, regs[registers::RR7 as usize]);
        assert_eq!(47, regs[registers::RPC as usize]);
    }

    #[test]
    fn test_02() {
        // Jsr saves the pc value and then increments the pc in the value inside the passed register

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 8;
        regs[registers::RR2 as usize] = 40;

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut regs);

        // This means 'Save PC at R7 ad then increment it in the value in the register'
        let instr: u16 = 0b0100000010000000;
        jsr(instr, &mut regs);

        assert_eq!(8, regs[registers::RR7 as usize]);
        assert_eq!(40, regs[registers::RPC as usize]);
    }
}
