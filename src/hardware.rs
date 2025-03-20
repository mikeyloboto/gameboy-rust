pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
    vram: Vec<u8>,
    oam: Vec<u8>,
    io_registers: Vec<u8>,
}

struct Registers {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
}

struct Register {
    value: u16,
}
impl Register {
    fn high(&self) -> u8 {
        (self.value & 0x00FF) as u8
    }
    fn low(&self) -> u8 {
        ((self.value & 0xFF00) >> 8) as u8
    }
    fn set_high(&mut self, value: u8) {
        self.value = (self.value & 0x00FF) | ((value as u16) << 8);
    }
    fn set_low(&mut self, value: u8) {
        self.value = (self.value & 0xFF00) | value as u16;
    }
}

pub struct Cpu {
    registers: Registers,
}
