use super::utils;
use crate::{mem_read, memory};

pub fn ldr(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x3F, 6);

    // add pc_offset to the content of a base register, look at that memory location and put that data in the destination register
    regs[dest_reg as usize] = mem_read(regs[base_reg as usize] + pc_offset, memory);

    utils::update_flags(dest_reg, regs);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::ldr;

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LD
    */
}
