
mod hexlib;
mod memory;

use memory::Memory;
use std::env;
use std::fs;
use std::process::ExitCode;


// while in the src dir, to run, carpgo run -- .\binary filename. bin files need to be in the same dir as the src dir, or you can provide the full path to the file.

fn main()-> ExitCode {  // Adding ExitCode as the return type of the main function allows us to return an exit code to the operating system when the program finishes executing. This is useful for indicating whether the program ran successfully or encountered an error.
                       // Also in Windows, run $LASTEXITCODE to check if the program ran successfully or encountered an error. If the program ran successfully, $LASTEXITCODE will be 0. If the program encountered an error, $LASTEXITCODE will be a non-zero value.                


    // Collect the command line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();


   if args.len() < 2 || args.len() > 3
    {
        eprintln!("Usage: bininspect <filename> [load-address]");
        return ExitCode::FAILURE;
    }


    // Print the command line arguments in Debug Format. {:#?} is the pretty Debug format specifier. Regular Debug format specifier is {:?}. The pretty Debug format specifier prints the output in a more readable format with indentation and line breaks.
    // println!("Arguments: {:#?}", args);

    //println!("Arguments: {}");
    // Iterate over the command line arguments and print each one in a new line.
    //for arg in args.iter(){
    //   println!("Argument: {}", arg);
    //}


    // &args[1] is an example of "borrowing" in Rust. It creates a reference to the second element of the args vector, which is the filename provided by the user. This allows us to use the filename without taking ownership of it, which is important in Rust's ownership model.
    // if we need to just look at some data without taking ownership of it, we can borrow it. Borrowing is a way to access data without taking ownership of it. In this case, we are borrowing the filename from the args vector so that we can use it without taking ownership of it.
    let filename = &args[1];


    // Get the optional load address.
    //
    // If the user does not provide one,
    // use 0x0000.
    //
    let load_address: u16 = match args.get(2)
    {
        Some(value) =>
        {
            match parse_address(value)
            {
                Ok(address) =>
                {
                    address
                }

                Err(error) =>
                {
                    eprintln!("Error: {}", error);
                    return ExitCode::FAILURE;
                }
            }
        }

        None =>
        {
            0x0000
        }
    };
    

    let file =  fs::read(filename); // Read the contents of the file specified by the filename argument. The read function returns a Result type, which can be either Ok or Err. If the file is read successfully, it returns Ok with the contents of the file as a vector of bytes. If there is an error reading the file, it returns Err with an error message.


    match file {
        Ok(data) => {
            let end_address = load_address as usize + data.len();

            if end_address > 65536
            {
                eprintln!("Error: file does not fit in 64 KB memory");
                return ExitCode::FAILURE;
            }


            println!("File: {}", filename); // Print the filename.
            println!("Size: {} bytes", data.len()); // Print the size of the file.

            let checksum_8 = hexlib::checksum_byte(&data);
            let checksum_16 = hexlib::checksum_word(&data);


            println!("File Checksum: 0x{:02X}", checksum_8);
            println!("File Checksum: 0x{:04X}", checksum_16);


            println!("Dump File:");
            hexlib::hexdump(&data, load_address); // Call the print_hex_dump function to print the hex dump of the file contents. We pass a reference to the data vector and the starting address (0) as arguments.
         
        
    
            let mut memory = Memory::new();

            let result = memory.load(load_address, &data);

            match result
            {
                Ok(()) =>
                {
                    println!("Program loaded successfully.");
                }

                Err(error) =>
                {
                    eprintln!("Error loading program: {}", error);
                    return ExitCode::FAILURE;
                }
            }


            let start = load_address as usize;
            for i in 0..data.len(){
                println!("Memory[0x{:04X}] = 0x{:02X}", start + i, memory.read((start + i) as u16));
            }


            return ExitCode::SUCCESS; // Return a success exit code if the file was read successfully.
        }
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error); // Print an error message if there was an error reading the file. The eprintln! macro is used to print the error message to the standard error stream.
            return ExitCode::FAILURE; // Return a failure exit code if there was an error reading the file.
        }
    }
}



fn parse_address(value: &str) -> Result<u16, String>
{
    if value.starts_with("0x")
    {
        let hex = &value[2..];

        match u16::from_str_radix(hex, 16)
        {
            Ok(address) =>
            {
                return Ok(address);
            }

            Err(_) =>
            {
                return Err(String::from("Invalid hexadecimal address"));
            }
        }
    }
    else
    {
        match value.parse::<u16>()
        {
            Ok(address) =>
            {
                return Ok(address);
            }

            Err(_) =>
            {
                return Err(String::from("Invalid decimal address"));
            }
        }
    }
}