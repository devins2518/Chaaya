#![allow(dead_code)]
use super::cpu::*;
use crate::utils::registers::*;
use crate::utils::Mode;
use crate::ARM_TRAIT;

impl Arm7 {
    pub(crate) fn nop(&self) {}
}

impl ARM_TRAIT for Arm7 {
    fn decode(&mut self, opcode: u32) {
        if self.program_registers[0].parse_condition_code(opcode) {
            #[cfg(debug_assertions)]
            println!("opcode {:#04b}", opcode);
        }
    }

    fn reset(&mut self) {
        self.gp_registers[R14_SVC] = self.gp_registers[R15PC];
        self.program_registers[SPSR_SVC] = self.program_registers[CPSR];
        self.program_registers[CPSR].set_mode(0b10011);
        self.program_registers[CPSR].set_irq_mask(true);
        self.program_registers[CPSR].set_fiq_mask(true);
        self.program_registers[CPSR].set_thumb_mode(true);
        *self.gp_registers[R15PC] = 0x00;
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
        let (val, wrap) = op2.overflowing_sub(*self.gp_registers[rn]);
        *self.gp_registers[rd] = val;
        if wrap {
            self.program_registers[0].set_overflow(true);
        }
    }

    fn add(&mut self, rd: usize, rn: usize, op2: u32) {
        let (val, wrap) = self.gp_registers[rn].overflowing_add(op2);
        *self.gp_registers[rd] = val;
        if wrap {
            self.program_registers[0].set_overflow(true);
        }
    }

    // Wrong
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
