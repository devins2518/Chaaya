#![allow(dead_code)]
use modular_bitfield::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Register(u32);

impl Register {
    pub(crate) const fn new(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for Register {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod registers {
    const R00: u8 = 0;
    const T00: u8 = 0;
    const R01: u8 = 1;
    const T01: u8 = 1;
    const R02: u8 = 2;
    const T02: u8 = 2;
    const R03: u8 = 3;
    const T03: u8 = 3;
    const R04: u8 = 4;
    const T04: u8 = 4;
    const R05: u8 = 5;
    const T05: u8 = 5;
    const R06: u8 = 6;
    const T06: u8 = 6;
    const R07: u8 = 7;
    const T07: u8 = 7;
    const R08: u8 = 8;
    const R09: u8 = 9;
    const R10: u8 = 10;
    const R11: u8 = 11;
    const R12: u8 = 12;
    const R13SP: u8 = 13;
    const T13SP: u8 = 13;
    const R14LR: u8 = 14;
    const T14LR: u8 = 14;
    const R15PC: u8 = 15;
    const T15PC: u8 = 15;
    const R08_FIQ: u8 = 16;
    const R09_FIQ: u8 = 17;
    const R10_FIQ: u8 = 18;
    const R11_FIQ: u8 = 19;
    const R12_FIQ: u8 = 20;
    const R13_FIQ: u8 = 21;
    const R14_FIQ: u8 = 22;
    const R13_SVC: u8 = 23;
    const R14_SVC: u8 = 24;
    const R13_ABT: u8 = 25;
    const R14_ABT: u8 = 26;
    const R13_IRQ: u8 = 27;
    const R14_IRQ: u8 = 28;
    const R13_UND: u8 = 29;
    const R14_UND: u8 = 30;
}

#[derive(BitfieldSpecifier, Debug)]
pub(crate) enum Endianness {
    Big,
    Little,
}

pub(crate) enum Mode {
    User,
    FastIrq,
    Irq,
    Supervisor,
    Abort,
    Undef,
    System,
}

#[bitfield()]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ARMInstruction {
    cond: B4,
    rest: B28,
}
