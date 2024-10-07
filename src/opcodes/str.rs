use super::utils;
use crate::{mem_write, memory};

pub fn str(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x3F, 6);

    let address: u32 = regs[base_reg as usize] as u32 + pc_offset as u32;
    let address: u16 = address as u16;

    // add pc_offset to the content of a base register, look at that memory location and put there the data in the destination register
    mem_write(address, memory, regs[dest_reg as usize]);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::str;
    use crate::ld::ld;
    use crate::memory;

    #[test]
    fn test_01() {
        // str puts in the memory direction defined by the offset and the base register the content of the source register

        let mut regs: [u16; 11] = [0; 11];
        let mut memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

        regs[registers::RR1 as usize] = 16;
        regs[registers::RR2 as usize] = 57;

        // This means 'Put at (offset + reg value) direction of memory the content of the source register'
        let str_instr: u16 = 0b0111010001000001;
        str(str_instr, &mut regs, &mut memory);

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010001;
        ld(ld_instr, &mut regs, &mut memory);

        assert_eq!(57, regs[registers::RR3 as usize]);
    } // This test is similar to the thing I would test with de load type instructions
}
