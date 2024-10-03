use super::utils;
use crate::mem_read;
use crate::registers;

pub fn ld(instr: u16, regs: &mut [u16; 11]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // add pc_offset to the current PC, look at that memory location and put that data in the destination register
    regs[dest_reg as usize] = mem_read(regs[registers::RPC as usize] + pc_offset);
    utils::update_flags(dest_reg, regs);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::ld;

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LD
    */
}
