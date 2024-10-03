use crate::registers;


pub fn jmp (instr: u16, regs: &mut [u16; 11]) {
    let base_reg = (instr >> 6) & 0x7;
    regs[registers::RPC as usize] = regs[base_reg as usize];
}


#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::jmp;

    #[test]
    fn test_01() {
        // Jump increments the pc in the passed register value

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 16;

        // This means 'Increment PC in the content in the base register'
        let instr: u16 = 0b1100000001000000;
        jmp(instr, &mut regs);

        assert_eq!(16, regs[registers::RPC as usize]);
    }
    // Maybe a test that involves jsr, like go to a subroutine and come back with jmp
}
