const MEMORY_SIZE: usize = 65536;

pub struct Memory
{
    data: [u8; MEMORY_SIZE],
}

impl Memory
{
    pub fn new() -> Memory
    {
        return Memory
        {
            data: [0; MEMORY_SIZE],
        };
    }

    pub fn read(&self, address: u16) -> u8
    {
        return self.data[address as usize];
    }

    pub fn write(&mut self, address: u16, value: u8)
    {
        self.data[address as usize] = value;
    }



    pub fn load(&mut self, start_address: u16, program: &[u8]) -> Result<(), String>
    {
        let start = start_address as usize;
        let end = start + program.len();

        if end > MEMORY_SIZE
        {
                return Err(
                String::from("Program does not fit in memory")
            );
        }

        for i in 0..program.len()
        {
            self.data[start + i] = program[i];
        }

        return Ok(());
    }
}