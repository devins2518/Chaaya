#![allow(dead_code)]
use super::{coprocessor::Cp15, utils::Cpsr};
use crate::utils::{Endianness, Mode, Register};
use crate::ARM_DERIVE;
use crate::ARM_TRAIT;

use tokio::sync::mpsc;

#[derive(ARM_DERIVE)]
pub(crate) struct Arm9 {
    fifo_send: mpsc::Sender<()>,
    fifo_recv: mpsc::Receiver<()>,
    endianness: Endianness,
    mode: Mode,
    gp_registers: [Register; 31],
    program_registers: [Cpsr; 7],
    cp15: Cp15,
}

impl Arm9 {
    pub(crate) fn new(fifo_recv: mpsc::Receiver<()>, fifo_send: mpsc::Sender<()>) -> Self {
        Self {
            fifo_send,
            fifo_recv,
            endianness: Endianness::Little,
            mode: Mode::User,
            gp_registers: [Register::new(0); 31],
            program_registers: [Cpsr::default(); 7],
            cp15: Cp15::new(),
        }
    }
}
