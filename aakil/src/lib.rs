mod traits;
pub use aakil_derives::ARM as ARM_DERIVE;
use traits::ARM as ARM_TRAIT;

mod arm7;
mod arm9;
mod utils;

mod cpu;
pub use cpu::Cpu;
