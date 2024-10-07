use super::utils;
use crate::hardware::registers;
use crate::hardware::vm::VM;

pub fn ld(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let address: u32 = pc_offset as u32 + vm.get_register_value(registers::RPC) as u32;

    // Read the value from the place where the memory above was computed
    let value = vm.mem_read(address as u16);

    vm.update_register_value(dest_reg, value);
    vm.update_flags(dest_reg);
}

#[cfg(test)]
mod tests {

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LD
    */
}
