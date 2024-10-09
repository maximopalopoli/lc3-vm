use crate::errors::VmError;

use super::consts;

use std::io::Read;

pub struct VM {
    memory: [u16; consts::MEMORY_MAX],
    regs: [u16; 11],
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        let memory: [u16; consts::MEMORY_MAX] = [0; consts::MEMORY_MAX];
        let regs: [u16; 11] = [0; 11];
        VM { memory, regs }
    }

    /// There is no way to write in a forbidden address since it's limited by the u16 limits
    pub fn mem_write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    /// There is no way to access to a forbidden address since it's limited by the u16 limits
    pub fn mem_read(&mut self, address: u16) -> Result<u16, VmError> {
        if address == consts::MR_KBSR {
            self.handle_keyboard()?;
        }
        Ok(self.memory[address as usize])
    }

    fn handle_keyboard(&mut self) -> Result<(), VmError> {
        let mut buffer = [0; 1];
        match std::io::stdin().read_exact(&mut buffer) {
            Ok(_) => {
                if buffer[0] != 0 {
                    self.mem_write(consts::MR_KBSR, 1 << 15);
                    self.mem_write(consts::MR_KBDR, buffer[0] as u16);
                } else {
                    self.mem_write(consts::MR_KBSR, 0)
                }
                Ok(())
            }
            Err(e) => Err(VmError::KeyboardInputError(e)),
        }
    }

    pub fn update_flags(&mut self, register_number: u16) -> Result<(), VmError> {
        if register_number as usize > self.regs.len() {
            Err(VmError::OutOfBoundsError)
        } else {
            if self.regs[register_number as usize] == 0 {
                self.regs[consts::RCOND as usize] = consts::FL_ZRO;
            } else if self.regs[register_number as usize] >> 15 == 1 {
                // a 1 in the left-most bit indicates negative
                self.regs[consts::RCOND as usize] = consts::FL_NEG;
            } else {
                self.regs[consts::RCOND as usize] = consts::FL_POS;
            }
            Ok(())
        }
    }

    pub fn get_register_value(&self, register_number: u16) -> Result<u16, VmError> {
        if register_number as usize > self.regs.len() {
            Err(VmError::OutOfBoundsError)
        } else {
            Ok(self.regs[register_number as usize])
        }
    }

    pub fn update_register_value(
        &mut self,
        register_number: u16,
        value: u16,
    ) -> Result<(), VmError> {
        if register_number as usize > self.regs.len() {
            Err(VmError::OutOfBoundsError)
        } else {
            self.regs[register_number as usize] = value;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::vm::VM;

    #[test]
    fn test_01() {
        // When initialized, the VM starts with memory and registers are set in 0
        let vm = VM::new();

        assert_eq!(0, vm.memory[435]);
        assert_eq!(0, vm.regs[4]);
    }

    #[test]
    fn test_02() {
        // When calling mem_write, updates the vm memory
        let mut vm = VM::new();

        let address = 7;
        let value = 18;
        vm.mem_write(address, value);

        assert_eq!(value, vm.memory[address as usize]);
    }

    #[test]
    fn test_03() {
        // mem_read returns the same as the value in the memory
        let mut vm = VM::new();

        let address = 7;
        let value = 18;
        vm.mem_write(address, value);

        let read = vm.mem_read(address).unwrap();

        assert_eq!(read, vm.memory[address as usize]);
    }

    #[test]
    fn test_04() {
        // update_register_value changes the value of the register
        let mut vm = VM::new();

        let register_number = 5;
        let value = 89;
        vm.update_register_value(register_number, value).unwrap();

        assert_eq!(value, vm.regs[register_number as usize]);
    }

    #[test]
    fn test_05() {
        // get_register_value gets the value of the registers array
        let mut vm = VM::new();

        let register_number = 3;
        let value = 89;
        vm.update_register_value(register_number, value).unwrap();

        let read = vm.get_register_value(register_number).unwrap();

        assert_eq!(read, vm.regs[register_number as usize]);
    }

    #[test]
    fn test_06() {
        // update_register_value when passing an out of bounds index returns an error
        let mut vm = VM::new();

        let value = 89;
        let result = vm.update_register_value(100, value);

        assert!(result.is_err());
    } 
}
