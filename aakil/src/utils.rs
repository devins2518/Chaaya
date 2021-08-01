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

pub mod registers {
    pub const R00: usize = 0;
    pub const T00: usize = 0;
    pub const R01: usize = 1;
    pub const T01: usize = 1;
    pub const R02: usize = 2;
    pub const T02: usize = 2;
    pub const R03: usize = 3;
    pub const T03: usize = 3;
    pub const R04: usize = 4;
    pub const T04: usize = 4;
    pub const R05: usize = 5;
    pub const T05: usize = 5;
    pub const R06: usize = 6;
    pub const T06: usize = 6;
    pub const R07: usize = 7;
    pub const T07: usize = 7;
    pub const R08: usize = 8;
    pub const R09: usize = 9;
    pub const R10: usize = 10;
    pub const R11: usize = 11;
    pub const R12: usize = 12;
    pub const R13SP: usize = 13;
    pub const T13SP: usize = 13;
    pub const R14LR: usize = 14;
    pub const T14LR: usize = 14;
    pub const R15PC: usize = 15;
    pub const T15PC: usize = 15;
    pub const R08_FIQ: usize = 16;
    pub const R09_FIQ: usize = 17;
    pub const R10_FIQ: usize = 18;
    pub const R11_FIQ: usize = 19;
    pub const R12_FIQ: usize = 20;
    pub const R13_FIQ: usize = 21;
    pub const R14_FIQ: usize = 22;
    pub const R13_SVC: usize = 23;
    pub const R14_SVC: usize = 24;
    pub const R13_ABT: usize = 25;
    pub const R14_ABT: usize = 26;
    pub const R13_IRQ: usize = 27;
    pub const R14_IRQ: usize = 28;
    pub const R13_UND: usize = 29;
    pub const R14_UND: usize = 30;
    pub const CPSR: usize = 0;
    pub const SPSR_FIQ: usize = 1;
    pub const SPSR_SVC: usize = 2;
    pub const SPSR_ABT: usize = 3;
    pub const SPSR_IRQ: usize = 4;
    pub const SPSR_UND: usize = 5;
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
