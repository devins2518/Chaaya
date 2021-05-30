#![allow(dead_code)]
use std::future::Future;

use super::{coprocessor::Cp15, utils::Cpsr};
use crate::utils::{Endianness, Mode, Register};
use crate::ARM_DERIVE;
use crate::ARM_TRAIT;

use genawaiter::{rc::Gen, sync::gen, sync_producer, yield_};
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
    pub(crate) fn new(receiver: mpsc::Receiver<()>, send: mpsc::Sender<()>) -> Self {
        let gp_registers = [
            Register::new("R00"),
            Register::new("R01"),
            Register::new("R02"),
            Register::new("R03"),
            Register::new("R04"),
            Register::new("R05"),
            Register::new("R06"),
            Register::new("R07"),
            Register::new("R08"),
            Register::new("R09"),
            Register::new("R10"),
            Register::new("R11"),
            Register::new("R12"),
            Register::new("R13/SP"),
            Register::new("R14/LR"),
            Register::new("R15/PC"), // R15 is always used as the PC
            Register::new("R08_fiq"),
            Register::new("R09_fiq"),
            Register::new("R10_fiq"),
            Register::new("R11_fiq"),
            Register::new("R12_fiq"),
            Register::new("R13_fiq"),
            Register::new("R14_fiq"),
            Register::new("R13_svc"),
            Register::new("R14_svc"),
            Register::new("R13_abt"),
            Register::new("R14_abt"),
            Register::new("R13_irq"),
            Register::new("R14_irq"),
            Register::new("R13_und"),
            Register::new("R14_und"),
        ];
        Arm9 {
            fifo_send: send,
            fifo_recv: receiver,
            endianness: Endianness::Little,
            mode: Mode::User,
            gp_registers,
            program_registers: [Cpsr::default(); 7],
            cp15: Cp15::new(),
        }
    }
}
