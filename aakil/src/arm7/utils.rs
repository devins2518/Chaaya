#![allow(dead_code)]
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Cpsr {
    pub negative: bool,   // N
    pub zero: bool,       // Z
    pub carry: bool,      // C
    pub overflow: bool,   // V
    pub reserved: B20,    // Reserved
    pub irq_mask: bool,   // I
    pub fiq_mask: bool,   // F
    pub thumb_mode: bool, // T
    pub mode: B5,
}

impl Cpsr {
    pub(crate) fn parse_condition_code(&self, opcode: u32) -> bool {
        match opcode >> 28 {
            0x0 => self.zero(),
            0x1 => !self.zero(),
            0x2 => self.carry(),
            0x3 => !self.carry(),
            0x4 => self.negative(),
            0x5 => !self.negative(),
            0x6 => self.overflow(),
            0x7 => !self.overflow(),
            0x8 => self.carry() & !self.zero(),
            0x9 => !self.carry() | self.zero(),
            0xA => self.negative() == self.overflow(),
            0xB => self.negative() != self.overflow(),
            0xC => !self.zero() & (self.negative() == self.overflow()),
            0xD => self.zero() | (self.negative() != self.overflow()),
            0xE => true,
            0xF => false,
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_flags() {
    let mut flags = Cpsr::new();
    assert_eq!(flags.parse_condition_code(0x00000000), false);
    flags.set_zero(true);
    assert!(flags.parse_condition_code(0x00000000));
    assert_eq!(flags.parse_condition_code(0x10000000), false);
    flags.set_zero(false);
    assert!(flags.parse_condition_code(0x10000000));

    assert_eq!(flags.parse_condition_code(0x20000000), false);
    flags.set_carry(true);
    assert!(flags.parse_condition_code(0x20000000));
    assert_eq!(flags.parse_condition_code(0x30000000), false);
    flags.set_carry(false);
    assert!(flags.parse_condition_code(0x30000000));

    assert_eq!(flags.parse_condition_code(0x40000000), false);
    flags.set_negative(true);
    assert!(flags.parse_condition_code(0x40000000));
    assert_eq!(flags.parse_condition_code(0x50000000), false);
    flags.set_negative(false);
    assert!(flags.parse_condition_code(0x50000000));

    assert_eq!(flags.parse_condition_code(0x60000000), false);
    flags.set_overflow(true);
    assert!(flags.parse_condition_code(0x60000000));
    assert_eq!(flags.parse_condition_code(0x70000000), false);
    flags.set_overflow(false);
    assert!(flags.parse_condition_code(0x70000000));

    assert_eq!(flags.parse_condition_code(0x80000000), false);
    flags.set_carry(true);
    assert!(flags.parse_condition_code(0x80000000));
    flags.set_zero(true);
    assert_eq!(flags.parse_condition_code(0x80000000), false);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_carry(false);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_zero(false);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_carry(true);
    assert_eq!(flags.parse_condition_code(0x90000000), false);

    assert!(flags.parse_condition_code(0xA0000000));
    flags.set_negative(true);
    assert_eq!(flags.parse_condition_code(0xA0000000), false);
    assert!(flags.parse_condition_code(0xB0000000));
    flags.set_negative(false);
    assert_eq!(flags.parse_condition_code(0xB0000000), false);

    assert!(flags.parse_condition_code(0xC0000000));
    flags.set_zero(true);
    assert_eq!(flags.parse_condition_code(0xC0000000), false);
    flags.set_zero(false);
    flags.set_overflow(true);
    assert_eq!(flags.parse_condition_code(0xC0000000), false);
    flags.set_overflow(false);
    assert_eq!(flags.parse_condition_code(0xD0000000), false);
    flags.set_zero(true);
    assert!(flags.parse_condition_code(0xD0000000));
    flags.set_overflow(true);
    assert!(flags.parse_condition_code(0xD0000000));
    flags.set_zero(false);
    assert!(flags.parse_condition_code(0xD0000000));
}
