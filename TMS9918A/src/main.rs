mod tms9918a;
use tms9918a::Tms9918a;


fn main() {
    let mut vdp = Tms9918a::new();
    println!("Tms9918a Initialized");

    vdp.vram_write(0x1000, 0x55);   
    let value = vdp.vram_read(0x1000);
    println!("VRAM value: 0x{:02X}", value);

}
