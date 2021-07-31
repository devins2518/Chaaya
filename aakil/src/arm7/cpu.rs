#![allow(dead_code)]
use super::utils::Cpsr;
use crate::utils::{Endianness, Mode, Register};
use crate::ARM_DERIVE;
use crate::ARM_TRAIT;

use tokio::sync::mpsc;

pub(crate) struct Arm7 {
    fifo_send: mpsc::Sender<()>,
    fifo_recv: mpsc::Receiver<()>,
    endianness: Endianness,
    mode: Mode,
    gp_registers: [Register; 31],
    program_registers: [Cpsr; 7],
}

impl Arm7 {
    pub(crate) fn new(receiver: mpsc::Receiver<()>, send: mpsc::Sender<()>) -> Self {
        let gp_registers = [Register::new(0); 31];
        Arm7 {
            fifo_send: send,
            fifo_recv: receiver,
            endianness: Endianness::Little,
            mode: Mode::User,
            gp_registers,
            program_registers: [Cpsr::default(); 7],
        }
    }
}
impl ARM_TRAIT for Arm7 {
    fn decode(&mut self, opcode: u32) {
        if self.program_registers[0].parse_condition_code(opcode) {
            #[cfg(debug_assertions)]
            println!("opcode {:#04b}", opcode);
        }
    }

    fn logical_and(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] = *self.gp_registers[rn] & op2;
    }

    fn logical_xor(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] = *self.gp_registers[rn] ^ op2;
    }

    fn sub(&mut self, rd: usize, rn: usize, op2: u32) {
        let (val, wrap) = self.gp_registers[rn].overflowing_sub(op2);
        *self.gp_registers[rd] = val;
        if wrap {
            self.program_registers[0].set_overflow(true);
        }
    }

    fn rev_sub(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] = op2 - *self.gp_registers[rn]
    }

    fn add(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] = *self.gp_registers[rn] + op2;
    }

    fn add_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] =
            *self.gp_registers[rn] + op2 + self.program_registers[0].carry() as u32;
    }

    fn sub_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] =
            *self.gp_registers[rn] - op2 + self.program_registers[0].carry() as u32 - 1;
    }

    fn rev_sub_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] =
            op2 - *self.gp_registers[rn] + self.program_registers[0].carry() as u32 - 1;
    }

    //fn test_and(&mut self, rn: usize, op2: u32) {
    //*self.gp_registers[rn] & op2;
    //}

    //fn test_xor(&mut self, rn: usize, op2: u32) {
    //*self.gp_registers[rn] ^ op2;
    //}

    //fn test_sub(&mut self, rn: usize, op2: u32) {
    //*self.gp_registers[rn] - op2;
    //}

    //fn comp_neg(&mut self, rn: usize, op2: u32) {
    //*self.gp_registers[rn] + op2;
    //}
}

#[test]
fn test_arm7() {
    let (rx, tx) = tokio::sync::mpsc::channel(64);
    let mut arm7 = Arm7::new(tx, rx);
    // arith: 0b11
    // immediate: 0x1
    arm7.decode(0xEFFFFFFF)
}
