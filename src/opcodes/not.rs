use super::utils;

pub fn not(instr: u16, regs: &mut [u16; 11]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    regs[dest_reg as usize] = !regs[base_reg as usize];

    utils::update_flags(dest_reg, regs);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::not;

    #[test]
    fn test_01() {
        // Not puts in a destination register the result of the not operation on the base register

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = u16::max_value();
        regs[registers::RR2 as usize] = 5;

        // This means 'Put in the destination register the result of the not operation on the base register'
        let instr: u16 = 0b1001010001111111;
        not(instr, &mut regs);

        assert_eq!(0, regs[registers::RR2 as usize]);
    }
}
