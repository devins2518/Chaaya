#![allow(dead_code)]
use modular_bitfield::prelude::*;
use std::ops::{Deref, DerefMut};

pub(crate) struct Register {
    pub id: &'static str,
    pub visible: bool,
    pub value: u32,
}

impl Register {
    pub(crate) fn new(id: &'static str) -> Self {
        Register {
            id,
            visible: true,
            value: 0u32,
        }
    }
}

impl Deref for Register {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
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
