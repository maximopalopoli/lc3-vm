use super::utils;
use crate::mem_read;
use crate::mem_write;
use crate::memory;
use crate::registers;

pub fn st(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    // Add the current PC to the PC offset
    // We're doing these conversions to avoid overflow
    let val: u32 = regs[registers::RPC as usize] as u32 + pc_offset as u32;
    let val: u16 = val as u16;


    mem_write(val, memory, regs[source_reg as usize]);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::st;
    use crate::ld::ld;
    use crate::memory;

    #[test]
    fn test_01() {
        // st puts in the memory direction defined by the offset the content of the source register

        let mut regs: [u16; 11] = [0; 11];
        let mut memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

        regs[registers::RR1 as usize] = 16;

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut regs, &mut memory);

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut regs, &mut memory);

        assert_eq!(16, regs[registers::RR3 as usize]);
    } // This test is similar to the thing I would test with de load type instructions
}
