use super::utils;
use crate::mem_read;
use crate::memory;
use crate::registers;

pub fn ldi(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // add pc_offset to the current PC, look at that memory location to get the final address
    regs[r0 as usize] = *mem_read(*mem_read(regs[registers::RPC as usize] + pc_offset, memory), memory);
    utils::update_flags(r0, regs);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::ldi;

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LDI
    */
}
