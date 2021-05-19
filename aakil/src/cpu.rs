#![allow(dead_code)]
use crate::arm7::cpu::Arm7;
use crate::arm9::cpu::Arm9;
use tokio::sync::mpsc;

pub struct Cpu {
    arm7: Arm7,
    arm9: Arm9,
}

impl Cpu {
    pub fn new() -> Self {
        let (arm7_tx, arm9_rx) = mpsc::channel(64);
        let (arm9_tx, arm7_rx) = mpsc::channel(64);
        Cpu {
            arm7: Arm7::new(arm7_rx, arm9_tx),
            arm9: Arm9::new(arm9_rx, arm7_tx),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}
