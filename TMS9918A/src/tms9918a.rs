pub struct Tms9918a {
    vram: [u8; 0x4000],
    registers: [u8; 8],
    status: u8,
}

impl Tms9918a {

    pub fn new() -> Self {

        Self {
            vram: [0; 0x4000],
            registers: [0; 8],
            status: 0,
        }

    }

    pub fn vram_write(&mut self, address: u16, value: u8) {

        let address = (address & 0x3FFF) as usize;

        self.vram[address] = value;

    }

    pub fn vram_read(&self, address: u16) -> u8 {

        let address = (address & 0x3FFF) as usize;

        self.vram[address]

    }
}