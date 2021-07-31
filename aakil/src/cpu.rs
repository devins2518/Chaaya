#![allow(dead_code)]
use crate::arm7::cpu::Arm7;
use crate::arm9::cpu::Arm9;
use tokio::sync::mpsc;

pub struct DSCpu {
    arm7: Arm7,
    arm9: Arm9,
}

#[cfg(feature = "gba")]
pub struct GBACpu {
    arm7: Arm7,
}

impl DSCpu {
    pub fn new() -> Self {
        let (arm7_tx, arm9_rx) = mpsc::channel(64);
        let (arm9_tx, arm7_rx) = mpsc::channel(64);
        DSCpu {
            arm7: Arm7::new(arm7_rx, arm9_tx),
            arm9: Arm9::new(arm9_rx, arm7_tx),
        }
    }
}

impl Default for DSCpu {
    fn default() -> Self {
        DSCpu::new()
    }
}
