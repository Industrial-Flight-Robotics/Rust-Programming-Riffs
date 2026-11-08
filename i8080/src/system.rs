

use crate::i8080::I8080;
use crate::memory::Memory;

pub struct System 
{
    pub cpu: I8080,
    pub memory: Memory,
}

impl System
{
    pub fn new() -> System
    {
        return System
        {
            cpu: I8080::new(),
            memory: Memory::new(),
        };
    }

    // -------------------------------------------------
    // Load a program into memory
    // -------------------------------------------------

    pub fn load_program(&mut self, load_address: u16, program: &[u8]) -> Result<(), String>
    {
        // Copy the program into memory.
        let result = self.memory.load(load_address, program);

        match result
        {
            Ok(()) =>
            {
                // Set the Program Counter to the
                // beginning of the loaded program.
                self.cpu.pc = load_address;

                return Ok(());
            }

            Err(error) =>
            {
                return Err(error);
            }
        }
    }

    pub fn step(&mut self) -> Result<(), String>
    {
        return self.cpu.step(&self.memory);
    }
}