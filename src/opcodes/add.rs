use super::utils;

pub fn add(instr: u16, regs: &mut [u16; 11]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // The five bits that we need to extend
        let imm5 = utils::sign_extend(instr & 0x1F, 5);
        regs[dest_reg as usize] = regs[sr1 as usize] + imm5;
    } else {
        let r2 = instr & 0x7;
        regs[dest_reg as usize] = regs[sr1 as usize] + regs[r2 as usize];
    }

    utils::update_flags(dest_reg, regs);
}


#[cfg(test)]
mod tests {
    use super::add;
    use super::super::super::registers;


    #[test]
    fn test_01() {
        // Adding two number with registers makes the sum and lefts it in a third register

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 1;
        regs[registers::RR2 as usize] = 1;
        
        // This means 'Add RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0001011001000010;

        add(instr, &mut regs);

        assert_eq!(2, regs[registers::RR3 as usize]);
    }

    #[test]
    fn test_02() {
        // Adding one number with an imm5 makes the sum and lefts the result it in a third register

        let mut regs: [u16; 11] = [0; 11];
        regs[registers::RR1 as usize] = 3;
        
        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut regs);

        assert_eq!(10, regs[registers::RR3 as usize]);
    }

    // Other tests: try an overflow, see flag updates
}
