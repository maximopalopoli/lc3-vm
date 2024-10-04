use super::utils;
use crate::memory;
use crate::registers;
use crate::mem_read;

pub fn sti(instr: u16, regs: &mut [u16; 11], memory: &mut [u16; memory::MEMORY_MAX]) {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    *mem_read(*mem_read(regs[registers::RPC as usize] + pc_offset, memory), memory) = regs[source_reg as usize];
    // add pc_offset to the current PC, look at the direction at that direction, and put that data in the source register
    regs[source_reg as usize] = regs[registers::RPC as usize] + pc_offset;
    utils::update_flags(source_reg, regs);
}


#[cfg(test)]
mod tests {
    use super::super::super::registers;
    use super::sti;
    use crate::ld::ld;
    use crate::st::st;
    use crate::memory;

    #[test]
    fn test_01() {
        // sti puts in the memory direction defined by the offset the content of the source register

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

        println!("{:?}", &memory[..20]);
        println!("{:?}", regs);

        assert_eq!(47, regs[registers::RR3 as usize]);
    } // This test is similar to the thing I would test with de load type instructions
}

