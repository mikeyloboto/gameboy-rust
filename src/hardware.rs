pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
    vram: Vec<u8>,
    oam: Vec<u8>,
    io_registers: Vec<u8>,
}

#[derive(Debug)]
struct Registers {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MBC {
    mem: Vec<u8>,
    key: u32,
}
impl MBC {
    pub fn new(mem: Vec<u8>, key: u32) -> MBC {
        MBC { mem, key }
    }
}

#[derive(Debug)]
pub struct Device {
    reg: Registers,
    banks: Vec<MBC>,
}

impl Device {
    pub fn new(banks: Vec<MBC>) -> Device {
        Device {
            reg: Registers {
                af: Register { value: 0x0000 },
                bc: Register { value: 0x0000 },
                de: Register { value: 0x0000 },
                hl: Register { value: 0x0000 },
                sp: Register { value: 0x0000 },
                pc: Register { value: 0x0000 },
            },
            banks,
        }
    }
}
