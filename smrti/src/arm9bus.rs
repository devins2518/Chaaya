#![allow(dead_code, unused_variables)]
// TODO: Remove

use std::sync::{Arc, Mutex};

use crate::memory::{DataType, Memory};

struct Arm9Bus {
    itcm: Arc<Mutex<Vec<u8>>>,
    dtcm: Arc<Mutex<Vec<u8>>>,
    ram: Arc<Mutex<Vec<u8>>>,
    //swram: _
    //io: _
    palettes: Arc<Mutex<Vec<u8>>>,
    vram_abg: Arc<Mutex<Vec<u8>>>,
    vram_bbg: Arc<Mutex<Vec<u8>>>,
    vram_aobj: Arc<Mutex<Vec<u8>>>,
    vram_bobj: Arc<Mutex<Vec<u8>>>,
    vram_lcdc: Arc<Mutex<Vec<u8>>>,
}

impl Memory for Arm9Bus {
    fn map_address(&self, address: u32) -> &[u8] {
        match address {
            // why doesnt rust allow exclusive pattern matching :despair:
            // https://github.com/rust-lang/rust/issues/37854
            0x00000000..=0x01FFFFFF => println!("Read from TCM address {}", address),
            0x02000000..=0x02FFFFFF => println!("Read from RAM address {}", address),
            0x03000000..=0x03FFFFFF => println!("Read from shared WRAM address {}", address),
            0x04000000..=0x04FFFFFF => println!("Read from ARM9 I/O address {}", address),
            0x05000000..=0x05FFFFFF => println!("Read from standard palettes address {}", address),
            0x06000000..=0x061FFFFF => println!("Read from Engine A, BG VRAM address {}", address),
            0x06200000..=0x063FFFFF => println!("Read from Engine B, BG VRAM address {}", address),
            0x06400000..=0x065FFFFF => println!("Read from Engine A, OBJ VRAM address {}", address),
            0x06600000..=0x067FFFFF => println!("Read from Engine B, OBJ VRAM address {}", address),
            0x06800000..=0x06FFFFFF => println!("Read from LCDC VRAM address {}", address),
            0x07000000..=0x07FFFFFF => println!("Read from OAM address {}", address),
            0x08000000..=0x9FFFFFFF => println!("Read from GBA slot ROM address {}", address),
            0xA0000000..=0xFFFEFFFF => println!("Read from GBA slot RAM address {}", address),
            _ => eprintln!("Attempted read from unmapped address {:x}", address),
        }
        unimplemented!()
    }
    fn map_address_mut(&mut self, address: u32) -> &mut [u8] {
        match address {
            // why doesnt rust allow exclusive pattern matching :despair:
            // https://github.com/rust-lang/rust/issues/37854
            0x00000000..=0x01FFFFFF => println!("Write to TCM address {}", address),
            0x02000000..=0x02FFFFFF => println!("Write to RAM address {}", address),
            0x03000000..=0x03FFFFFF => println!("Write to shared WRAM address {}", address),
            0x04000000..=0x04FFFFFF => println!("Write to ARM9 I/O address {}", address),
            0x05000000..=0x05FFFFFF => println!("Write to standard palettes address {}", address),
            0x06000000..=0x061FFFFF => println!("Write to Engine A, BG VRAM address {}", address),
            0x06200000..=0x063FFFFF => println!("Write to Engine B, BG VRAM address {}", address),
            0x06400000..=0x065FFFFF => println!("Write to Engine A, OBJ VRAM address {}", address),
            0x06600000..=0x067FFFFF => println!("Write to Engine B, OBJ VRAM address {}", address),
            0x06800000..=0x06FFFFFF => println!("Write to LCDC VRAM address {}", address),
            0x07000000..=0x07FFFFFF => println!("Write to OAM address {}", address),
            0x08000000..=0x9FFFFFFF => println!("Write to GBA slot ROM address {}", address),
            0xA0000000..=0xFFFEFFFF => println!("Write to GBA slot RAM address {}", address),
            _ => eprintln!("Attempted read from unmapped address {:x}", address),
        }
        unimplemented!()
    }
    fn read_word(&self, address: u32) -> u32 {
        unimplemented!()
    }
    fn read_halfword(&self, address: u32) -> u16 {
        unimplemented!()
    }
    fn read_byte(&self, address: u32) -> u8 {
        unimplemented!()
    }
    fn write(&self, address: u32, value: DataType) {
        unimplemented!()
    }
}
