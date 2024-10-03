use super::super::condition_flags;
use super::super::registers;

fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

fn update_flags(r: u16, regs: &mut [u16; 11]) {
    if regs[r as usize] == 0 {
        regs[registers::RCOND as usize] = condition_flags::FL_ZRO;
    } else if regs[r as usize] >> 15 == 1 {
        // a 1 in the left-most bit indicates negative
        regs[registers::RCOND as usize] = condition_flags::FL_NEG;
    } else {
        regs[registers::RCOND as usize] = condition_flags::FL_POS;
    }
}

pub fn add(instr: u16, regs: &mut [u16; 11]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // The five bits that we need to extend
        let imm5 = sign_extend(instr & 0x1F, 5);
        regs[dest_reg as usize] = regs[sr1 as usize] + imm5;
    } else {
        let r2 = instr & 0x7;
        regs[dest_reg as usize] = regs[sr1 as usize] + regs[r2 as usize];
    }

    update_flags(dest_reg, regs);
}
