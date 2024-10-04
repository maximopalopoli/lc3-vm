use super::utils;
use crate::mem_read;
use crate::memory;
use crate::registers;

pub fn ld(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let address: u32 = pc_offset as u32 + regs[registers::RPC as usize] as u32;

    // Read the value from the place where the memory above was computed
    let value = mem_read(address as u16, memory);

    regs[dest_reg as usize] = value;
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
