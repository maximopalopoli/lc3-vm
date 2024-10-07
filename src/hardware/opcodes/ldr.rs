use crate::hardware::vm::VM;

use super::utils;

pub fn ldr(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x3F, 6);

    let val = vm.get_register_value(base_reg) as u32 + pc_offset as u32;
    let mem_value = vm.mem_read(val as u16);

    vm.update_register_value(dest_reg, mem_value);

    vm.update_flags(dest_reg);
}

#[cfg(test)]
mod tests {

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LD
    */
}
