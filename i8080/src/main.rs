mod memory;
mod i8080;
mod system;

use system::System;
use std::process::ExitCode;


fn main()  
{
        let mut system = System::new();

        // -----------------------------------------------------
        // Create a small Intel 8080 program.
        // -----------------------------------------------------

        let program: [u8; 3] =
        [
            0x3E,  // MVI A, data
            0x42,  // Immediate value
            0x00,  // NOP
        ];

        
        // -----------------------------------------------------
        // Load the program at 0x0100.
        // -----------------------------------------------------

        let result = system.load_program(0x0100, &program);

        match result
        {
            Ok(()) =>
            {
                println!("Program loaded successfully.");
            }
            Err(error) =>
            {
                eprintln!("Load error: {}", error);
            }
        }

        // -----------------------------------------------------
        // Display CPU state before execution.
        // -----------------------------------------------------

        println!();
        println!("Before execution:");

        println!("A  = 0x{:02X}", system.cpu.a);
        println!("PC = 0x{:04X}", system.cpu.pc);

        // -----------------------------------------------------
        // Execute MVI A, 42H.          
        // -----------------------------------------------------

        let result = system.step();

        match result
        {
            Ok(()) =>
            {
                println!("Instruction executed.");
            }

            Err(error) =>
            {
                eprintln!("CPU error: {}", error);
            }
        }
  

        // -----------------------------------------------------
        // Display CPU state after execution.
        // -----------------------------------------------------

        println!();
        println!("After execution:");

        println!("A  = 0x{:02X}", system.cpu.a);
        println!("PC = 0x{:04X}", system.cpu.pc);
}