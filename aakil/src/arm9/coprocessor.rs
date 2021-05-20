use crate::utils::{Endianness, Register};
use modular_bitfield::prelude::*;

pub(super) struct Cp15 {
    main_id_reg: Register,
    cache_type_reg: Register,
    tcm_size_reg: Register,
    control_reg: ControlRegister,
    // ?????? i have no idea what this is supposed to be
    // :D ???? https://problemkaputt.github.io/gbatek.htm#armcp15protectionunitpu :D
    protection_unit: ProtectionUnit,
    // more stuff
}

impl Cp15 {
    pub(super) fn new() -> Self {
        let mut main_id_reg = Register::new("Main ID Register");
        main_id_reg.value = 0x41059461;
        let mut cache_type_reg = Register::new("Cache Type Register");
        cache_type_reg.value = 0x0F0D2112;
        let mut tcm_size_reg = Register::new("TCM Size Register");
        tcm_size_reg.value = 0x00140180;
        let control_reg = ControlRegister::from(0x00000078);
        let mut protection_unit = Register::new("Protection Unit");
        protection_unit.value = 0x00140180;
        Self {
            main_id_reg,
            cache_type_reg,
            tcm_size_reg,
            control_reg,
            protection_unit,
        }
    }
}

#[bitfield]
#[repr(u32)]
#[derive(Debug)]
pub(super) struct ControlRegister {
    mmu_pu_enable: B1,
    #[skip(setters)]
    alignment_fault_check: B1,
    data_unified_cache: B1,
    #[skip(setters)]
    write_buffer: B1,
    #[skip(setters)]
    exception_handling: B1,
    #[skip(setters)]
    _26bit_address_faults: B1,
    #[skip(setters)]
    abort_model: B1,
    #[bits = 1]
    endianness: Endianness,
    #[skip(setters)]
    sys_protection: B1,
    #[skip(setters)]
    rom_protection: B1,
    #[skip(setters)]
    impl_defined: B1,
    #[skip(setters)]
    branch_prediction: B1,
    instruction_cache: B1,
    exception_vectors: B1,
    cache_replacement: B1,
    pre_armv5: B1,
    dtcm_enabled: B1,
    dtcm_load_mode: B1,
    itcm_enabled: B1,
    itcm_load_mode: B1,
    #[skip]
    __: B2,
    #[skip(setters)]
    unaligned_access: B1,
    #[skip(setters)]
    extended_page_table: B1,
    #[skip]
    __: B1,
    #[skip(setters)]
    cpsr_e_on_exception: B1,
    #[skip]
    __: B1,
    #[skip(setters)]
    fiq_behavior: B1,
    #[skip(setters)]
    tex_remap: B1,
    #[skip(setters)]
    force_ap: B1,
    #[skip]
    __: B2,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug)]
pub(super) struct ProtectionUnit {
    mmu_pu_enable: B1,
    #[skip(setters)]
    alignment_fault_check: B1,
    data_unified_cache: B1,
    #[skip(setters)]
    write_buffer: B1,
    #[skip(setters)]
    exception_handling: B1,
    #[skip(setters)]
    _26bit_address_faults: B1,
    #[skip(setters)]
    abort_model: B1,
    #[bits = 1]
    endianness: Endianness,
    #[skip(setters)]
    sys_protection: B1,
    #[skip(setters)]
    rom_protection: B1,
    #[skip(setters)]
    impl_defined: B1,
    #[skip(setters)]
    branch_prediction: B1,
    instruction_cache: B1,
    exception_vectors: B1,
    cache_replacement: B1,
    pre_armv5: B1,
    dtcm_enabled: B1,
    dtcm_load_mode: B1,
    itcm_enabled: B1,
    itcm_load_mode: B1,
    #[skip]
    __: B2,
    #[skip(setters)]
    unaligned_access: B1,
    #[skip(setters)]
    extended_page_table: B1,
    #[skip]
    __: B1,
    #[skip(setters)]
    cpsr_e_on_exception: B1,
    #[skip]
    __: B1,
    #[skip(setters)]
    fiq_behavior: B1,
    #[skip(setters)]
    tex_remap: B1,
    #[skip(setters)]
    force_ap: B1,
    #[skip]
    __: B2,
}

#[test]
fn test_control_register() {
    let control_reg = ControlRegister::from(0x00000078);
    assert!(control_reg.write_buffer() == 1);
    assert!(control_reg.exception_handling() == 1);
    assert!(control_reg._26bit_address_faults() == 1);
    assert!(control_reg.abort_model() == 1);
}
