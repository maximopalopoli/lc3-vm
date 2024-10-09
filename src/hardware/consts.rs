

// Cond Flags
pub const FL_POS: u16 = 1 << 0;
pub const FL_ZRO: u16 = 1 << 1; /* Z */
pub const FL_NEG: u16 = 1 << 2; /* N */


// Memory
pub const MEMORY_MAX: usize = u16::MAX as usize;

pub const MR_KBSR: u16 = 0xFE00; /* keyboard status */
pub const MR_KBDR: u16 = 0xFE02; /* keyboard data */


// Registers
pub const RR0: u16 = 0;
pub const RR1: u16 = 1;
pub const RR2: u16 = 2;
pub const RR3: u16 = 3;
pub const RR4: u16 = 4;
pub const RR5: u16 = 5;
pub const RR6: u16 = 6;
pub const RR7: u16 = 7;
pub const RPC: u16 = 8;
pub const RCOND: u16 = 9;
pub const RCOUNT: u16 = 10;

pub const PC_START: u16 = 0x3000;
