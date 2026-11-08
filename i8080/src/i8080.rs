

use crate::memory::Memory;

pub struct I8080{
    pub a: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
    pub flags: Flags,
}


pub struct Flags
{
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub auxiliary_carry: bool,
    
}

impl I8080{
    pub fn new() -> Self{
        return I8080 { a: 0x00, b: 0x00, c: 0x00, d: 0x00, e: 0x00, h: 0x00, l: 0x00, pc: 0x0000, sp: 0x0000, flags: Flags::new() };
    }

    pub fn reset(&mut self) {
        self.a = 0x00;
        self.b = 0x00;
        self.c = 0x00;
        self.d = 0x00;
        self.e = 0x00;
        self.h = 0x00;
        self.l = 0x00;
        self.pc = 0x0000;
        self.sp = 0x0000;
    }

    pub fn dump(&self) {
        println!("\nCPU State:");
        println!("BC: {:04X} B: {:02X} C: {:02X}", self.get_bc(), self.b, self.c);
        println!("DE: {:04X} D: {:02X} E: {:02X}", self.get_de(), self.d, self.e);
        println!("HL: {:04X} H: {:02X} L: {:02X}", self.get_hl(), self.h, self.l);
        println!("A: {:02X}", self.a);
        println!("PC: {:04X}", self.pc);
        println!("SP: {:04X}", self.sp);
        println!("\nFlags State:");
        println!("Zero Flag: {}", self.flags.zero);
        println!("Sign Flag: {}", self.flags.sign);
        println!("Parity Flag: {}", self.flags.parity);
        println!("Carry Flag: {}", self.flags.carry);
        println!("Auxiliary Carry Flag: {}", self.flags.auxiliary_carry);
    }

    pub fn get_bc(&self) -> u16
    {
        let high = u16::from(self.b); // Convert the high byte (B) to u16
        let low = u16::from(self.c);  // Convert the low byte (C) to u16

        return (high << 8) | low;
    }

    pub fn get_de(&self) -> u16
    {
        let high = u16::from(self.d);
        let low = u16::from(self.e);

        return (high << 8) | low;
    }

    pub fn get_hl(&self) -> u16
    {
        let high = u16::from(self.h);
        let low = u16::from(self.l);

        return (high << 8) | low;
    }
    
    pub fn set_bc(&mut self, value: u16)
    {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    
    pub fn set_de(&mut self, value: u16)
    {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16)
    {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_flags(&mut self, result: u8){
        self.flags.zero = result == 0;

        self.flags.sign = (result & 0x80) != 0;
        self.flags.parity = I8080::calc_parity(result);
    }

    pub fn calc_parity(value: u8) -> bool
    {
            let mut count: u8 = 0;
            let mut number: u8 = value;

            for _ in 0..8
            {
                if (number & 0x01) != 0
                {
                    count += 1;
                }

                number = number >> 1;
            }

            return (count % 2) == 0;
    }

    pub fn step(&mut self, memory: &Memory) -> Result<(), String>
    {
        // Fetch the instruction at the Program Counter.
        let opcode: u8 = memory.read(self.pc);

      
        // Decode and execute the instruction.
        match opcode
        { 

            //---------------------------------------------
            //NOP - No Operation
            //opcode: 0x00
            // Bytes: 1
            // ---------------------------------------------
            0x00 =>
            {
                // NOP - No Operation
                self.pc = self.pc.wrapping_add(1);
            }

            // ---------------------------------------------
            // MVI A, data
            // Opcode: 0x3E
            // Bytes: 2
            // ---------------------------------------------
            0x3E =>
            {
                // read the next memory location for the data operand
                let data = memory.read(self.pc.wrapping_add(1));

                // store the data in the accumulator register (A)
                self.a = data;

                // update the program counter two bytes to point to the next instruction
                self.pc = self.pc.wrapping_add(2);
            }

            _ =>
            {
                return Err(
                    format!("Unknown opcode: 0x{:02X}", opcode)
                );
            }
        }
        return Ok(());
    }
}


impl Flags
{
    pub fn new() -> Flags
    {
        return Flags
        {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            auxiliary_carry: false,
        };
    }
}

