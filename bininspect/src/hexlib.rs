  
  pub fn hexdump(data: &[u8], start: u16) // define a public function named print_hex_dump that takes a slice of bytes and a starting address as parameters
  {

        let mut addr: u16 = start;
        
        for chunk in data.chunks(16)
        {
                print!("{:04X}: ", addr);  // print the current address in hexadecimal format, padded with zeros to 4 digits. The {:04X} format specifier is used to format the address as a hexadecimal number with at least 4 digits, padding with zeros if necessary.
 
                for byte in chunk // iterate over the bytes in the current chunk and print each byte in hexadecimal format, padded with zeros to 2 digits. The {:02X} format specifier is used to format the byte as a hexadecimal number with at least 2 digits, padding with zeros if necessary.
                {
                    print!("{:02X} ", byte);
                }

                for _ in chunk.len()..16  // iterate over the remaining bytes in the chunk (if any) and print spaces to align the output. This is done to ensure that the output is properly aligned even if the last chunk has fewer than 16 bytes.
                {
                    print!("   ");
                }

                print!(" |");

                for byte in chunk // iterate over the bytes in the current chunk and print each byte as a character if it is a printable ASCII character, or a dot (.) if it is not. The is_ascii_graphic() method is used to check if the byte is a printable ASCII character, and the *byte == b' ' condition is used to check if the byte is a space character.
                {
                    if byte.is_ascii_graphic() || *byte == b' ' // check if the byte is a printable ASCII character or a space character
                    {
                        print!("{}", *byte as char);
                    }
                    else
                    {
                        print!(".");
                    }
                }

                println!("|");

                addr   = addr + 16; // increment the address variable by 16 for the next chunk of bytes.
        }
 }



 pub fn checksum_byte(data: &[u8]) -> u8 // define a function named checksum that takes a slice of bytes as a parameter and returns a u8 value representing the checksum of the data
 {
    let mut sum: u8 = 0;

    for byte in data
    {
        sum = sum.wrapping_add(*byte);
    }

    return sum;
}

pub fn checksum_word(data: &[u8]) -> u16
{
    let mut sum: u16 = 0;

    for byte in data
    {
        sum = sum.wrapping_add(u16::from(*byte));
    }

    return sum;
}