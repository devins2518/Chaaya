#![allow(dead_code)]
extern crate proc_macro;
use crate::proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const R00: u8 = 0;
const R01: u8 = 1;
const R02: u8 = 2;
const R03: u8 = 3;
const R04: u8 = 4;
const R05: u8 = 5;
const R06: u8 = 6;
const R07: u8 = 7;
const R08: u8 = 8;
const R09: u8 = 9;
const R10: u8 = 10;
const R11: u8 = 11;
const R12: u8 = 12;
const R13SP: u8 = 13;
const R14LR: u8 = 14;
const R15PC: u8 = 15;
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

#[proc_macro_derive(ARM)]
pub fn arm_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
    impl ARM_TRAIT for #name {
        fn decode(&mut self, opcode: u32) {
            if false == self.program_registers[0].parse_condition_code(opcode) {
                return
            }

            // MUL/MLA Opcode masking
            let halfword = (opcode | 0x00000010);

            #[cfg(debug_assertions)]
            println!("opcode {:#04b}", opcode);
            println!("halfword {:#04b}", halfword);

        }

        fn logical_and(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] & op2;
        }

        fn logical_xor(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] ^ op2;
        }

        fn sub(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] - op2;
        }

        fn rev_sub(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = op2 - *self.gp_registers[rn]
        }

        fn add(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] + op2;
        }

        fn add_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] + op2 + self.program_registers[0].carry() as u32;
        }

        fn sub_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = *self.gp_registers[rn] - op2 + self.program_registers[0].carry() as u32 - 1;
        }

        fn rev_sub_with_carry(&mut self, rd: usize, rn: usize, op2: u32) {
            *self.gp_registers[rd] = op2 - *self.gp_registers[rn] + self.program_registers[0].carry() as u32 - 1;
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
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Thumb)]
pub fn thumb_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl crate::thumb::Thumb for #name {
            fn add(&mut self, index: usize) {
                self.gp_registers[0].value += self.gp_registers[index].value;
            }
        }
    };

    TokenStream::from(expanded)
}
