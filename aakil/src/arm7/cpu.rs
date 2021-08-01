#![allow(dead_code)]
use super::utils::Cpsr;
use crate::utils::{Endianness, Mode, Register};
use crate::ARM_TRAIT;

use tokio::sync::mpsc;

pub(crate) struct Arm7 {
    pub(super) fifo_send: mpsc::Sender<()>,
    pub(super) fifo_recv: mpsc::Receiver<()>,
    pub(super) endianness: Endianness,
    pub(super) mode: Mode,
    pub(super) gp_registers: [Register; 31],
    pub(super) program_registers: [Cpsr; 6],
}

impl Arm7 {
    pub(crate) fn new(receiver: mpsc::Receiver<()>, send: mpsc::Sender<()>) -> Self {
        Arm7 {
            fifo_send: send,
            fifo_recv: receiver,
            endianness: Endianness::Little,
            mode: Mode::User,
            gp_registers: [Register::new(0); 31],
            program_registers: [Cpsr::default(); 6],
        }
    }
}

#[test]
fn test_arm7() {
    let (rx, tx) = tokio::sync::mpsc::channel(64);
    let mut arm7 = Arm7::new(tx, rx);
    arm7.decode(0xEFFFFFFF)
}
