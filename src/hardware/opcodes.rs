use crate::{errors::VmError, VM};
use super::consts;

pub const OP_BR: u16 = 0; /* branch */
pub const OP_ADD: u16 = 1; /* add  */
pub const OP_LD: u16 = 2; /* load */
pub const OP_ST: u16 = 3; /* store */
pub const OP_JSR: u16 = 4; /* jump register */
pub const OP_AND: u16 = 5; /* bitwise and */
pub const OP_LDR: u16 = 6; /* load register */
pub const OP_STR: u16 = 7; /* store register */
pub const OP_RTI: u16 = 8; /* unused */
pub const OP_NOT: u16 = 9; /* bitwise not */
pub const OP_LDI: u16 = 10; /* load indirect */
pub const OP_STI: u16 = 11; /* store indirect */
pub const OP_JMP: u16 = 12; /* jump */
pub const OP_RES: u16 = 13; /* reserved (unused) */
pub const OP_LEA: u16 = 14; /* load effective address */
pub const OP_TRAP: u16 = 15; /* execute trap */


/// If the first number from left to right is a 1, extends the 1. Otherwise, returns the original value
pub fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}




// ADD


/// Depending on a flag, adds two numbers or a number and an imm5 and puts the results of the operation in a destination register, and then update the flags
pub fn add(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // The five bits to extend
        let imm5 = sign_extend(instr & 0x1F, 5);

        // I use casting to prevent an overflow.
        let val: u32 = imm5 as u32 + vm.get_register_value(sr1)? as u32;
        vm.update_register_value(dest_reg, val as u16)?;
    } else {
        // Get the second reg from instr and puts in the dest_reg the and operation result
        let r2 = instr & 0x7;

        let val: u32 = vm.get_register_value(sr1)? as u32 + vm.get_register_value(r2)? as u32;

        vm.update_register_value(dest_reg, val as u16)?;
    }
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_add {
    use crate::hardware::consts;
    use crate::hardware::vm::VM;
    use super::add;

    #[test]
    fn test_01() {
        // Adding two number with registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 1).unwrap();
        vm.update_register_value(consts::RR2, 1).unwrap();

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0001011001000010;

        add(instr, &mut vm).unwrap();

        assert_eq!(2, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_02() {
        // Adding one number with an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm).unwrap();

        assert_eq!(10, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_03() {
        // Adding with a positive result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_POS,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_04() {
        // Adding with a zero result lets turned on the zero flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100000;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_ZRO,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_05() {
        // Adding with a negative result lets turned on the negative flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001110000;

        add(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_NEG,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }
}


// AND

/// Depending on a flag, performs an and between two numbers or a number with an imm5 and puts the results of the operation in a destination register, and then update the flags
pub fn and(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // first operand (SR1)
    let sr1 = (instr >> 6) & 0x7;

    // immediate (1) or register (0) mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        // Get the secont arg from the five bits to extend and puts in the dest_reg the and operation result
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.update_register_value(dest_reg, vm.get_register_value(sr1)? & imm5)?;
    } else {
        // Get the second reg from instr and puts in the dest_reg the and operation result
        let r2 = instr & 0x7;
        vm.update_register_value(
            dest_reg,
            vm.get_register_value(sr1)? & vm.get_register_value(r2)?,
        )?;
    }

    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_and {
    use crate::hardware::consts;
    use crate::hardware::vm::VM;
    use super::and;

    #[test]
    fn test_01() {
        // Doing an and with two numbers in registers makes the sum and lefts it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 2).unwrap();
        vm.update_register_value(consts::RR2, 3).unwrap();

        // This means 'Do an AND with RR1 and RR2 and put the result on RR3'
        let instr: u16 = 0b0101011001000010;

        and(instr, &mut vm).unwrap();

        assert_eq!(2, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_02() {
        // Doing an and with one register number and an imm5 makes the sum and lefts the result it in a third register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 15).unwrap();

        // This means 'Do an AND with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0101011001100111;

        and(instr, &mut vm).unwrap();

        assert_eq!(7, vm.get_register_value(consts::RR3).unwrap());
    }

    #[test]
    fn test_03() {
        // Perform an and with a positive result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 3).unwrap();

        // This means 'Do an and with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001100111;

        and(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_POS,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_04() {
        // Perform an and with a zero result lets turned on the positive flag

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();

        // This means 'Do an and with RR1 and an imm5 and put the result on RR3'
        let instr: u16 = 0b0001011001111111;

        and(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_ZRO,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }
}


// BR

/// Depending on the cond_flag, if it matches the current conditional register, then jumps to the position defined in the pc_offset
pub fn br(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    // If a flag is set and the br op has that flag activated, enters the block
    if cond_flag & vm.get_register_value(consts::RCOND)? != 0 {
        // Set the PC to go to the extended PCoffset
        let val: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
        vm.update_register_value(consts::RPC, val as u16)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests_br {
    use super::{add, and, br};

    use crate::hardware::consts;
    use crate::hardware::vm::VM;

    #[test]
    fn test_01() {
        // Make an operation that lefts the zero flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 0).unwrap();
        vm.update_register_value(consts::RR2, 0).unwrap();

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_ZRO);

        // This means 'If last operation left flag zero, then increment PC in an PCoffset'
        let br_instr = 0b0000010001100000;
        br(br_instr, &mut vm).unwrap();

        assert_eq!(96, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_02() {
        // Make an operation that lefts the positive flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 1).unwrap();
        vm.update_register_value(consts::RR2, 4).unwrap();

        // This means 'Add RR1 and RR2 and put the result on RR3'
        let add_instr: u16 = 0b0001011001000010;
        add(add_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_POS);

        // This means 'If last operation left flag positive, then increment PC in an PCoffset'
        let br_instr = 0b0000001001000001;
        br(br_instr, &mut vm).unwrap();

        assert_eq!(65, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_03() {
        // Make an operation that lefts the negative flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 1).unwrap();
        vm.update_register_value(consts::RR2, 4).unwrap();

        // This means 'Add RR1 and an imm5 and put the result on RR3'
        let add_instr: u16 = 0b0001011001111110;
        add(add_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_NEG);

        // This means 'If last operation left flag negative, then increment PC in an PCoffset'
        let br_instr = 0b0000100001000011;
        br(br_instr, &mut vm).unwrap();

        assert_eq!(67, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_04() {
        // Make an operation that lefts the negative or zero flag on, and then make a conditional branch

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 162).unwrap();
        vm.update_register_value(consts::RR2, 0).unwrap();

        // This means 'And RR1 and RR2 and put the result on RR3'
        let and_instr: u16 = 0b0101011001000010;
        and(and_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_ZRO);

        // This means 'If last operation left flag negative or zero, then increment PC in an PCoffset'
        let br_instr = 0b0000110001100001;
        br(br_instr, &mut vm).unwrap();

        assert_eq!(97, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_05() {
        // Make a conditional branch and verify the RPC has moved

        let mut vm = VM::new();

        // Set a value bc can be initialized with garbage
        vm.update_register_value(consts::RCOND, consts::FL_POS)
            .unwrap();

        // This means 'Increment PC in an PCoffset, no matter what happened in last operation'
        let br_instr = 0b0000111011100001;
        br(br_instr, &mut vm).unwrap();

        assert_eq!(225, vm.get_register_value(consts::RPC).unwrap());
    }
}


// JMP

/// Sets the pc as the value in the base register
pub fn jmp(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    let base_reg = (instr >> 6) & 0x7;
    vm.update_register_value(consts::RPC, vm.get_register_value(base_reg)?)?;
    Ok(())
}

#[cfg(test)]
mod tests_jmp {
    use crate::hardware::{vm::VM, consts};
    use super::{jsr, jmp};

    #[test]
    fn test_01() {
        // Jump increments the pc in the passed register value

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let instr: u16 = 0b1100000001000000;
        jmp(instr, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_02() {
        // Jump returns to the original pc value after a jsr

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Set PC in the content in the base register'
        let instr: u16 = 0b1100000001000000;
        jmp(instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111;
        jsr(instr, &mut vm).unwrap();

        // This means 'Set PC in the content in the RR7'
        let instr: u16 = 0b1100000111000000;
        jmp(instr, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RPC).unwrap());
    }
}


// JSR

/// Saves the RPC value on the R7, and then, depending on a flag, increments the pc in an offset, or sets the pc as the value of a base_reg
pub fn jsr(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // if 1, then use pc_offset, if 0
    let use_offset = (instr >> 11) & 1;

    // save the pc in R7
    vm.update_register_value(consts::RR7, vm.get_register_value(consts::RPC)?)?;

    if use_offset != 0 {
        // Inscreases pc in the value of the offset. Use casting to avoid overflow
        let pc_offset = sign_extend(instr & 0x7ff, 11);
        let val: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
        vm.update_register_value(consts::RPC, val as u16)?;
    } else {
        // Updates pc with the value of the register.
        let base_reg = (instr >> 6) & 0x7;
        vm.update_register_value(consts::RPC, vm.get_register_value(base_reg)?)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests_jsr {
    use super::{jsr, jmp};
    use crate::hardware::{vm::VM, consts};

    #[test]
    fn test_01() {
        // Jsr saves the pc value and then increments the pc in the passed offset
        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b0100100000011111; // 31
        jsr(instr, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RR7).unwrap());
        assert_eq!(47, vm.get_register_value(consts::RPC).unwrap());
    }

    #[test]
    fn test_02() {
        // Jsr saves the pc value and then increments the pc in the value inside the passed register
        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 8).unwrap();
        vm.update_register_value(consts::RR2, 40).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the value in the register'
        let instr: u16 = 0b0100000010000000;
        jsr(instr, &mut vm).unwrap();

        assert_eq!(8, vm.get_register_value(consts::RR7).unwrap());
        assert_eq!(40, vm.get_register_value(consts::RPC).unwrap());
    }
}


// LD

/// Loads in a destination register the value stored in pc plus an pc_offset, and then update the flags
pub fn ld(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // Read the value from the place where the memory above was computed
    let address: u32 = pc_offset as u32 + vm.get_register_value(consts::RPC)? as u32;
    let value = vm.mem_read(address as u16)?;

    vm.update_register_value(dest_reg, value)?;
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_ld {
    use crate::hardware::{consts, vm::VM};
    use super::{ld, st};

    #[test]
    fn test_01() {
        // ld puts in the source register the content of the memory direction defined by the offset

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 31).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(31, vm.get_register_value(consts::RR3).unwrap());
        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a negative value, ld sets negative flag on

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, u16::max_value())
            .unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_NEG);
    }
}


// LDI

/// Loads in a destination register the value stored in the direction obtained by the sum of pc and pc_offset, and then update the flags
pub fn ldi(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let r0 = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // add pc_offset to the current PC, look at that memory location to get the final address
    let address_read = vm.mem_read(vm.get_register_value(consts::RPC)? + pc_offset)?;
    let value = vm.mem_read(address_read)?;

    vm.update_register_value(r0, value)?;
    vm.update_flags(r0)?;

    Ok(())
}

#[cfg(test)]
mod tests_ldi {
    use crate::hardware::{consts, vm::VM};
    use super::{ldi, st};

    #[test]
    fn test_01() {
        // ldi puts in the source register the content in the memory address defined on the memory direction defined by the offset

        let mut vm = VM::new();

        vm.update_register_value(consts::RR1, 31).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st1_instr: u16 = 0b0011001000000001; // 1
        st(st1_instr, &mut vm).unwrap();

        vm.update_register_value(consts::RR2, 96).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st2_instr: u16 = 0b0011010000011111; // 31
        st(st2_instr, &mut vm).unwrap();

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm).unwrap();

        assert_eq!(96, vm.get_register_value(consts::RR3).unwrap());
        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a zero value, ldi sets zero flag on (values of memory are initialized in zero)

        let mut vm = VM::new();

        // This means 'Put at source register the content defined on the direction of memory product of pc+offset'
        let ldi_instr: u16 = 0b1010011000000001;
        ldi(ldi_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_ZRO);
    }
}


// LDR

/// Loads in a destination register the value stored in the value in a base register plus an pc_offset, and then update the flags
pub fn ldr(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x3F, 6);

    // Add the value in base reg to the offset to get the address to read
    let address = vm.get_register_value(base_reg)? as u32 + pc_offset as u32;
    let mem_value = vm.mem_read(address as u16)?;

    vm.update_register_value(dest_reg, mem_value)?;

    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_ldr {
    use crate::{
        hardware::consts, hardware::vm::VM};
    use super::{ldr, st};

    #[test]
    fn test_01() {
        // ldr puts in the source register the content in the memory address defined between the offset and the base register

        let mut vm = VM::new();

        vm.update_register_value(consts::RR1, 49).unwrap();
        vm.update_register_value(consts::RR2, 16).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000011111; // 31
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory + base register value'
        let ldr_instr: u16 = 0b0110011010001111;
        ldr(ldr_instr, &mut vm).unwrap();

        assert_eq!(49, vm.get_register_value(consts::RR3).unwrap());
        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_POS);
    }

    #[test]
    fn test_02() {
        // When putting a zero value, ldr sets zero flag on (values of memory and registers are initialized in zero)

        let mut vm = VM::new();

        // This means 'Put at source register the content of offset direction of memory + base register value'
        let ldr_instr: u16 = 0b0110001000000001;
        ldr(ldr_instr, &mut vm).unwrap();

        assert!(vm.get_register_value(consts::RCOND).unwrap() == consts::FL_ZRO);
    }
}


// LEA

// Loads in a destination register the sum between pc and an pc_offset, and then update the flags
pub fn lea(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let val: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;

    // add pc_offset to the current PC, and put that direction in the destination register
    vm.update_register_value(dest_reg, val as u16)?;
    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_lea {
    use crate::hardware::{vm::VM, consts};
    use super::{lea, jmp};

    #[test]
    fn test_01() {
        // Lea puts in a destination register the sum between the PC register and an offset

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000010;
        jmp(jmp_instr, &mut vm).unwrap();

        // This means 'Save PC at R7 ad then increment it in the extended PCoffset'
        let instr: u16 = 0b1110100000011111; // 31
        lea(instr, &mut vm).unwrap();

        assert_eq!(47, vm.get_register_value(consts::RR4).unwrap());
    }
}


// NOT

/// Performs an not on the value of a base register and puts the result in a destination register, and then update the flags
pub fn not(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    let val = !vm.get_register_value(base_reg)?;
    vm.update_register_value(dest_reg, val)?;

    vm.update_flags(dest_reg)?;

    Ok(())
}

#[cfg(test)]
mod tests_not {
    use crate::hardware::{vm::VM, consts};
    use super::not;

    #[test]
    fn test_01() {
        // Not puts in a destination register the result of the not operation on the base register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, u16::max_value())
            .unwrap();
        vm.update_register_value(consts::RR2, 5).unwrap();

        // This means 'Put in the destination register the result of the not operation on the base register'
        let instr: u16 = 0b1001010001111111;
        not(instr, &mut vm).unwrap();

        assert_eq!(0, vm.get_register_value(consts::RR2).unwrap());
    }

    #[test]
    fn test_02() {
        // When performing with a positive number, sets the negative flag on

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 6).unwrap();

        // This means 'Put in the destination register the result of the not operation on the base register'
        let instr: u16 = 0b1001010001111111;
        not(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_NEG,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }

    #[test]
    fn test_03() {
        // When performing with a 'negative' number, sets the positive flag on

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, u16::max_value() - 10)
            .unwrap();

        // This means 'Put in the destination register the result of the not operation on the base register'
        let instr: u16 = 0b1001010001111111;
        not(instr, &mut vm).unwrap();

        assert_eq!(
            consts::FL_POS,
            vm.get_register_value(consts::RCOND).unwrap()
        );
    }
}


// ST

/// Puts in source register the value stored in pc + a pc_offset
pub fn st(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // Add the current PC to the PC offset to get the address where store the data
    let address: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
    let address: u16 = address as u16;

    let value = vm.get_register_value(source_reg)?;

    vm.mem_write(address, value);

    Ok(())
}

#[cfg(test)]
mod tests_st {
    use super::{st, ld};
    use crate::hardware::{vm::VM, consts};

    #[test]
    fn test_01() {
        // st puts in the memory direction defined by the offset the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000001;
        st(st_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000000001;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RR3).unwrap());
    }
}


// STI

/// Puts in source register the value stored in an address obtained searching in the direction (pc + a pc_offset) of memory
pub fn sti(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // source register (SR)
    let source_reg = (instr >> 9) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // Add the pc to the offset to get the address where read
    let var_address: u32 = vm.get_register_value(consts::RPC)? as u32 + pc_offset as u32;
    let var_address: u16 = var_address as u16;

    let address = vm.mem_read(var_address)? as usize;

    // store the reg value to the adress read above
    let value = vm.get_register_value(source_reg)?;
    vm.mem_write(address as u16, value);

    Ok(())
}

#[cfg(test)]
mod tests_sti {
    use super::{sti, st, ld};
    use crate::hardware::{vm::VM, consts};

    #[test]
    fn test_01() {
        // sti puts in the memory direction placed in the memory direction defined by the offset the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();
        vm.update_register_value(consts::RR2, 47).unwrap();

        // This means 'Put at offset direction of memory the content of the source register'
        let st_instr: u16 = 0b0011001000000011;
        st(st_instr, &mut vm).unwrap();

        // This means 'Find the offset direction of memory the direction where to put the content of the source register and do it'
        let sti_instr: u16 = 0b1011010000000011;
        sti(sti_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010000;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(47, vm.get_register_value(consts::RR3).unwrap());
    }
}


// STR

/// Puts in source register the value stored in base register + a pc_offset
pub fn str(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // destination register (DR)
    let dest_reg = (instr >> 9) & 0x7;

    // base register (BR)
    let base_reg = (instr >> 6) & 0x7;

    // PCoffset (9 bits)
    let pc_offset = sign_extend(instr & 0x3F, 6);

    // Add the content of the base register to the offset to get the address where store the data
    let address: u32 = vm.get_register_value(base_reg)? as u32 + pc_offset as u32;
    let address: u16 = address as u16;

    let value = vm.get_register_value(dest_reg)?;

    vm.mem_write(address, value);

    Ok(())
}

#[cfg(test)]
mod tests_str {
    use super::{str, ld};
    use crate::hardware::{vm::VM, consts};

    #[test]
    fn test_01() {
        // str puts in the memory direction defined by the offset and the base register the content of the source register

        let mut vm = VM::new();
        vm.update_register_value(consts::RR1, 16).unwrap();
        vm.update_register_value(consts::RR2, 57).unwrap();

        // This means 'Put at (offset + reg value) direction of memory the content of the source register'
        let str_instr: u16 = 0b0111010001000001;
        str(str_instr, &mut vm).unwrap();

        // This means 'Put at source register the content of offset direction of memory'
        let ld_instr: u16 = 0b0010011000010001;
        ld(ld_instr, &mut vm).unwrap();

        assert_eq!(57, vm.get_register_value(consts::RR3).unwrap());
    } // This test is similar to the thing I would test with de load type instructions
}


// TRAP
use std::{
    io::{self, Read, Write},
    process,
};

pub const TRAP_GETC: u16 = 0x20;
pub const TRAP_OUT: u16 = 0x21;
pub const TRAP_PUTS: u16 = 0x22;
pub const TRAP_IN: u16 = 0x23;
pub const TRAP_PUTSP: u16 = 0x24;
pub const TRAP_HALT: u16 = 0x25;

/// Performs the corresponding trap operation
pub fn trap(instr: u16, vm: &mut VM) -> Result<(), VmError> {
    // Set the Reg7 to the PC value
    let pc_value = vm.get_register_value(consts::RPC)?;
    vm.update_register_value(consts::RR7, pc_value)?;

    match instr & 0xFF {
        TRAP_GETC => {
            //Read a single character from the keyboard. The character is not echoed onto the
            //console. Its ASCII code is copied into R0. The high eight bits of R0 are cleared.

            let mut buf = [0; 1];
            if let Err(e) = io::stdin().read_exact(&mut buf) {
                return Err(VmError::KeyboardInputError(e));
            }

            vm.update_register_value(consts::RR0, buf[0] as u16)?;
        }
        TRAP_OUT => {
            //Write a character in R0 to the console display.

            let c = vm.get_register_value(consts::RR0)? as u8;
            print!("{}", c as char);
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_PUTS => {
            // Write a string of ASCII characters to the console display.

            let mut index = vm.get_register_value(consts::RR0)?;
            let mut c = vm.mem_read(index)?;

            // 0x0000 is a the NULL character equivalent
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = vm.mem_read(index)?;
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_IN => {
            //Print a prompt on the screen and read a single character from the keyboard. The
            //character is echoed onto the console monitor.

            println!("Enter a character: ");

            let mut buf: [u8; 1] = [0; 1];
            if let Err(e) = io::stdin().read_exact(&mut buf) {
                return Err(VmError::KeyboardInputError(e));
            }

            let c = buf[0];
            print!("{}", c as char);
            io::stdout().flush().expect("failed to flush");

            vm.update_register_value(consts::RR0, c as u16)?;
            vm.update_flags(consts::RR0)?;
        }
        TRAP_PUTSP => {
            // Write a string of ASCII characters to the console in parts (first half, second half)

            let mut index = vm.get_register_value(consts::RR0)?;
            let mut c = vm.mem_read(index)?;

            // 0x0000 is a the NULL character equivalent
            while c != 0x0000 {
                let char_1 = c & 0xFF;
                print!("{}", (char_1 as u8) as char);
                let char_2 = c >> 8;
                if char_2 != 0x0000 {
                    print!("{}", (char_2 as u8) as char);
                }
                index += 1;
                c = vm.mem_read(index)?;
            }
            io::stdout().flush().expect("failed to flush");
        }
        TRAP_HALT => {
            // Stop the program
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
            process::exit(1);
        }
        _ => {
            // Should not reach this point
            process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests_trap {
    use crate::
        hardware::{consts, vm::VM};
    use super::{jmp, trap, TRAP_OUT};

    #[test]
    fn test_01() {
        //Check that the value of the PC is saved in R7
        let mut vm = VM::new();

        vm.update_register_value(consts::RR1, 16).unwrap();

        // This means 'Increment PC in the content in the base register'
        let jmp_instr: u16 = 0b1100000001000000;
        jmp(jmp_instr, &mut vm).unwrap();

        trap(TRAP_OUT, &mut vm).unwrap();

        assert_eq!(16, vm.get_register_value(consts::RR7).unwrap());
    }

    // I imagine other tests, but for that cases i would have to mock i/o operations
}
