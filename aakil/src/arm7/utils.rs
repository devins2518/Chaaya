#![allow(dead_code)]
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Cpsr {
    pub negative: B1,   // N
    pub zero: B1,       // Z
    pub carry: B1,      // C
    pub overflow: B1,   // V
    pub reserved: B21,  // Reserved
    pub irq_mask: B1,   // I
    pub firq_mask: B1,  // F
    pub thumb_mode: B1, // T
    pub mode: B4,
}

impl Cpsr {
    pub(crate) fn parse_condition_code(&self, opcode: u32) -> bool {
        match opcode >> 28 {
            0x0 => self.zero() == 1,
            0x1 => self.zero() == 0,
            0x2 => self.carry() == 1,
            0x3 => self.carry() == 0,
            0x4 => self.negative() == 1,
            0x5 => self.negative() == 0,
            0x6 => self.overflow() == 1,
            0x7 => self.overflow() == 0,
            0x8 => (self.carry() == 1) & (self.zero() == 0),
            0x9 => (self.carry() == 0) | (self.zero() == 1),
            0xA => self.negative() == self.overflow(),
            0xB => self.negative() != self.overflow(),
            0xC => (self.zero() == 0) & (self.negative() == self.overflow()),
            0xD => (self.zero() == 1) | (self.negative() != self.overflow()),
            0xE => true,
            0xF => false,
            _ => panic!(),
        }
    }
}

#[test]
fn test_flags() {
    let mut flags = Cpsr::new();
    assert_eq!(flags.parse_condition_code(0x00000000), false);
    flags.set_zero(1);
    assert!(flags.parse_condition_code(0x00000000));
    assert_eq!(flags.parse_condition_code(0x10000000), false);
    flags.set_zero(0);
    assert!(flags.parse_condition_code(0x10000000));

    assert_eq!(flags.parse_condition_code(0x20000000), false);
    flags.set_carry(1);
    assert!(flags.parse_condition_code(0x20000000));
    assert_eq!(flags.parse_condition_code(0x30000000), false);
    flags.set_carry(0);
    assert!(flags.parse_condition_code(0x30000000));

    assert_eq!(flags.parse_condition_code(0x40000000), false);
    flags.set_negative(1);
    assert!(flags.parse_condition_code(0x40000000));
    assert_eq!(flags.parse_condition_code(0x50000000), false);
    flags.set_negative(0);
    assert!(flags.parse_condition_code(0x50000000));

    assert_eq!(flags.parse_condition_code(0x60000000), false);
    flags.set_overflow(1);
    assert!(flags.parse_condition_code(0x60000000));
    assert_eq!(flags.parse_condition_code(0x70000000), false);
    flags.set_overflow(0);
    assert!(flags.parse_condition_code(0x70000000));

    assert_eq!(flags.parse_condition_code(0x80000000), false);
    flags.set_carry(1);
    assert!(flags.parse_condition_code(0x80000000));
    flags.set_zero(1);
    assert_eq!(flags.parse_condition_code(0x80000000), false);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_carry(0);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_zero(0);
    assert!(flags.parse_condition_code(0x90000000));
    flags.set_carry(1);
    assert_eq!(flags.parse_condition_code(0x90000000), false);

    assert!(flags.parse_condition_code(0xA0000000));
    flags.set_negative(1);
    assert_eq!(flags.parse_condition_code(0xA0000000), false);
    assert!(flags.parse_condition_code(0xB0000000));
    flags.set_negative(0);
    assert_eq!(flags.parse_condition_code(0xB0000000), false);

    assert!(flags.parse_condition_code(0xC0000000));
    flags.set_zero(1);
    assert_eq!(flags.parse_condition_code(0xC0000000), false);
    flags.set_zero(0);
    flags.set_overflow(1);
    assert_eq!(flags.parse_condition_code(0xC0000000), false);
    flags.set_overflow(0);
    assert_eq!(flags.parse_condition_code(0xD0000000), false);
    flags.set_zero(1);
    assert!(flags.parse_condition_code(0xD0000000));
    flags.set_overflow(1);
    assert!(flags.parse_condition_code(0xD0000000));
    flags.set_zero(0);
    assert!(flags.parse_condition_code(0xD0000000));
}
