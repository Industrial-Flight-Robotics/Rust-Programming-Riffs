pub struct TMS9918A {
    vram: [u8; 0x4000],
    registers: [u8; 8],
    status: u8,
}

impl TMS9918A {
    pub fn new() -> Self {
        Self {
            vram: [0; 0x4000],
            registers: [0; 8],
            status: 0,
        }
    }
}

