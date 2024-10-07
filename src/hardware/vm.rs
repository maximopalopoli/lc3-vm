use super::condition_flags;
use super::memory;
use super::registers;

use std::io::Read;

pub struct VM {
    memory: [u16; memory::MEMORY_MAX],
    regs: [u16; 11],
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        let memory: [u16; memory::MEMORY_MAX] = [0; memory::MEMORY_MAX];
        let regs: [u16; 11] = [0; 11];
        VM { memory, regs }
    }

    pub fn mem_write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn mem_read(&mut self, address: u16) -> u16 {
        if address == memory::MR_KBSR {
            self.handle_keyboard();
        }
        self.memory[address as usize]
    }

    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.mem_write(memory::MR_KBSR, 1 << 15);
            self.mem_write(memory::MR_KBDR, buffer[0] as u16);
        } else {
            self.mem_write(memory::MR_KBSR, 0)
        }
    }

    pub fn update_flags(&mut self, r: u16) {
        if self.regs[r as usize] == 0 {
            self.regs[registers::RCOND as usize] = condition_flags::FL_ZRO;
        } else if self.regs[r as usize] >> 15 == 1 {
            // a 1 in the left-most bit indicates negative
            self.regs[registers::RCOND as usize] = condition_flags::FL_NEG;
        } else {
            self.regs[registers::RCOND as usize] = condition_flags::FL_POS;
        }
    }

    pub fn get_register_value(&self, register_number: u16) -> u16 {
        self.regs[register_number as usize]
    }

    pub fn update_register_value(&mut self, register_number: u16, value: u16) {
        self.regs[register_number as usize] = value;
    }
}
