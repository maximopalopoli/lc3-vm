use super::utils;
use crate::mem_read;
use crate::mem_write;
use crate::memory;
use crate::registers;

pub fn sti(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let val: u32 = regs[registers::RPC as usize] as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    // This is the difference between STI and ST
    let address = mem_read(val, memory) as usize;

    mem_write(address as u16, memory, regs[source_reg as usize]);
}

#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::sti;
    use crate::ld::ld;
    use crate::memory;
    use crate::st::st;

    #[test]
    fn test_01() {
        // sti puts in the memory direction placed in the memory direction defined by the offset the content of the source register

        let mut regs: [u16; 11] = [0; 11];
        let mut memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];

        regs[registers::RR1 as usize] = 16;
        regs[registers::RR2 as usize] = 47;

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000011;
        st(st_instr, &mut regs, &mut memory);

        // This means 'Find the offset direction of memory the direction where to put the content of the source register and do it'
        let sti_instr: u16 = 0b1011010000000011;
        sti(sti_instr, &mut regs, &mut memory);

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010000;
        ld(ld_instr, &mut regs, &mut memory);

        assert_eq!(47, regs[registers::RR3 as usize]);
    } // This test is similar to the thing I would test with de load type instructions
}
