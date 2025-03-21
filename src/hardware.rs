use std::time::Duration;

use crate::{constants, hardware, meta};

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
    pub mem: Vec<u8>,
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
    ram: Vec<MBC>,
}

impl Device {
    pub fn new(metadata: meta::CartridgeMetadata) -> Device {
        Device {
            reg: Registers {
                af: Register { value: 0x0000 },
                bc: Register { value: 0x0000 },
                de: Register { value: 0x0000 },
                hl: Register { value: 0x0000 },
                sp: Register { value: 0x0000 },
                pc: Register { value: 0x0000 },
            },
            banks: generate_banks(metadata.rom_size(), true),
            ram: generate_ram(),
        }
    }

    pub fn get_bank(&self, index: u16) -> &MBC {
        &self.banks[index as usize]
    }

    pub fn get_bank_mut(&mut self, index: u16) -> &mut MBC {
        &mut self.banks[index as usize]
    }

    pub fn get_flag(&self, flag: char) -> bool {
        match flag {
            'Z' => (self.reg.af.value & 0x80) >> 7 != 0,
            'N' => (self.reg.af.value & 0x40) >> 6 != 0,
            'H' => (self.reg.af.value & 0x20) >> 5 != 0,
            'C' => (self.reg.af.value & 0x10) >> 4 != 0,
            _ => panic!("Invalid flag: {}", flag),
        }
    }

    pub fn set_flag(&mut self, flag: char, value: bool) {
        match flag {
            'Z' => {
                self.reg
                    .af
                    .set_low((self.reg.af.low() & 0x7F) | ((value as u8) << 7));
            }
            'N' => {
                self.reg
                    .af
                    .set_low((self.reg.af.low() & 0xBF) | ((value as u8) << 6));
            }
            'H' => {
                self.reg
                    .af
                    .set_low((self.reg.af.low() & 0xDF) | ((value as u8) << 5));
            }
            'C' => {
                self.reg
                    .af
                    .set_low((self.reg.af.low() & 0xEF) | ((value as u8) << 4));
            }
            _ => panic!("Invalid flag: {}", flag),
        }
    }

    pub fn start_exec(&mut self) {
        let debug = true;
        loop {
            // Fetch
            let opcode = self.get_bank(0).mem[self.reg.pc.value as usize];

            // Decode & Execute

            match opcode {
                0x00 => {
                    // NOOP
                    self.reg.pc.value += 1;
                }
                0x01 => {
                    // LD BC, u16
                    let mut value = self.get_bank(0).mem[self.reg.pc.value as usize + 1] as u16;
                    value |= (self.get_bank(0).mem[self.reg.pc.value as usize + 2] as u16) << 8;
                    self.reg.bc.value = value;
                    self.reg.pc.value += 3;
                }
                0x02 => {
                    // LD (BC), A
                    self.reg.pc.value += 1;
                }

                0x0C => {
                    // INC C
                    let value = self.reg.bc.value + 1;
                    self.reg.bc.value = value;
                    self.set_flag('Z', value == 0);
                    self.set_flag('N', false);
                    self.set_flag('H', (value & 0x0F) == 0);
                    self.set_flag('C', value > 0xFF);
                    self.reg.pc.value += 1;
                }

                0xBF => {
                    // CP A, u8
                    let value = self.get_bank(0).mem[self.reg.pc.value as usize + 1];
                    if self.reg.af.low() == value {
                        self.set_flag('Z', true);
                        self.set_flag('N', false);
                        self.set_flag('H', true);
                        self.set_flag('C', false);
                    } else {
                        self.set_flag('Z', false);
                        self.set_flag('N', true);
                        self.set_flag('H', false);
                        self.set_flag('C', self.reg.af.high() < value);
                    }
                    self.reg.pc.value += 2;
                }

                _ => {
                    println!(
                        "Unknown opcode: {:#X} at PC: {:#X} - halting",
                        opcode, self.reg.pc.value
                    );
                    break;
                }
            }
            if debug {
                println!("Registers: {:?}", self.reg);
                println!(
                    "Flags: Z={}, N={}, H={}, C={}",
                    self.get_flag('Z'),
                    self.get_flag('N'),
                    self.get_flag('H'),
                    self.get_flag('C')
                );
                println!();
                std::thread::sleep(Duration::from_millis(500)); // Simulate delay for debug purposes (100ms)
            }

            // Increment PC
        }
    }
}

fn generate_banks(size: u8, convert_banks_size: bool) -> Vec<hardware::MBC> {
    let mut rom_banks = Vec::new();
    let rom_banks_amount = if convert_banks_size {
        constants::get_num_rom_banks(size)
    } else {
        size as u16
    };
    for bank in 0..rom_banks_amount {
        let rom_bank = hardware::MBC::new(Vec::new(), (bank as u32) * (0x4000 as u32));
        rom_banks.push(rom_bank);
    }
    return rom_banks;
}

fn generate_ram() -> Vec<hardware::MBC> {
    let mut ram_banks = Vec::new();
    for _ in 0..5 {
        let ram_bank = hardware::MBC::new(Vec::new(), 0);
        ram_banks.push(ram_bank);
    }
    return ram_banks;
}
