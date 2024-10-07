use super::utils;
use crate::hardware::vm::VM;
use crate::hardware::registers;

pub fn ldi(instr: u16, vm: &mut VM) {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let address_read = vm.mem_read(vm.get_register_value(registers::RPC) + pc_offset);

    let value = vm.mem_read(address_read);

    // add pc_offset to the current PC, look at that memory location to get the final address
    vm.update_register_value(r0, value);
    vm.update_flags(r0);
}

#[cfg(test)]
mod tests {

    /*
        Posible tests:
        - Do a ST an the verify the storaged value is there with an LDI
    */
}
