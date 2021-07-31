mod traits;
use aakil_derives::ARM as ARM_DERIVE;
use traits::ARM as ARM_TRAIT;

mod arm7;
mod arm9;
mod utils;

mod cpu;
pub use cpu::DSCpu;
#[cfg(feature = "gba")]
pub use cpu::GBACpu;
