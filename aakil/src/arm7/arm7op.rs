#![allow(dead_code)]
use super::cpu::*;
use crate::utils::registers::*;
use crate::ARM_TRAIT;

impl Arm7 {
    pub(crate) fn nop(&self) {}
}

impl ARM_TRAIT for Arm7 {
    fn decode(&mut self, opcode: u32) {
        if self.program_registers[0].parse_condition_code(opcode) {
            match opcode >> 24 {
                0x0 => println!("Multiply or multiply long"),
                0x1 => println!("SDS or BLE"),
                0xE => println!("Coprocessor instruction, probably noop"),
                0xF => println!("Software interrupt"),
                _ => unimplemented!(),
            }
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

    // Wrong
    fn add_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] =
            *self.gp_registers[rn] + op2 + self.program_registers[0].carry() as u32;
    }

    fn add(&mut self, rd: usize, rn: usize, op2: u32) {
        let (val, wrap) = self.gp_registers[rn].overflowing_add(op2);
        *self.gp_registers[rd] = val;
        if wrap {
            self.program_registers[0].set_overflow(true);
        }
    }

    fn logical_and(&mut self, rd: usize, rn: usize, op2: u32) {
        *self.gp_registers[rd] = *self.gp_registers[rn] & op2;
    }

    fn branch(&mut self, addr: u32) {}

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
