use super::super::condition_flags;
use super::super::registers;


pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1) & 1) != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

pub fn update_flags(r: u16, regs: &mut [u16; 11]) {
    if regs[r as usize] == 0 {
        regs[registers::RCOND as usize] = condition_flags::FL_ZRO;
    } else if regs[r as usize] >> 15 == 1 {
        // a 1 in the left-most bit indicates negative
        regs[registers::RCOND as usize] = condition_flags::FL_NEG;
    } else {
        regs[registers::RCOND as usize] = condition_flags::FL_POS;
    }
}
