use std::mem::{size_of, size_of_val};

use modular_bitfield::prelude::*;

#[bitfield]
pub(crate) struct Cartridge {
    game_title: B12,
    game_code: B4,
    maker_code: B2,
    unit_code: B1,
    encryption_seed_select: B1,
    device_capacity: B1,
    #[skip]
    __: B7,
    #[skip]
    __: B1,
    region: B1,
    rom_version: B1,
    autostart: B1,
    arm9_rom_offset: B4,
    arm9_entry_address: B4,
    arm9_ram_address: B4,
    arm9_size: B4,
    arm7_rom_offset: B4,
    arm7_entry_address: B4,
    arm7_ram_address: B4,
    arm7_size: B4,
    fnt_offset: B4,
    fnt_size: B4,
    fat_offset: B4,
    fat_size: B4,
    arm9_overlay_offset: B4,
    arm9_overlay_size: B4,
    arm7_overlay_offset: B4,
    arm7_overlay_size: B4,
    normal_command_port: B4,
    key1_command_port: B4, // Port 40001A4h setting for KEY1 commands   (usually 001808F8h)
    icon_title_offset: B4, //  Icon/Title offset (0=None) (8000h and up)
    sec_area_checksum: B2, //  Secure Area Checksum, CRC-16 of [[020h]..00007FFFh]
    sec_area_delay: B2,    // Secure Area Delay (in 131kHz units) (051Eh=10ms or 0D7Eh=26ms)
    arm9_auto_load_list: B4, //    ARM9 Auto Load List Hook RAM Address (?) ;\endaddr of auto-load
    arm7_auto_load_list: B4, //  ARM7 Auto Load List Hook RAM Address (?) ;/functions
    sec_area_disable: B8,  //  Secure Area Disable (by encrypted "NmMdOnly") (usually zero)
    total_used_rom: B4,    //Total Used ROM size (remaining/unused bytes usually FFh-padded)
    rom_header_size: B4,   // ROM Header Size (4000h)
    #[skip]
    __: B40,   //Reserved (zero filled; except, [88h..93h] used on DSi)
    #[skip]
    __: B16,   // Reserved (zero filled; or "DoNotZeroFillMem"=unlaunch fastboot)
    nin_logo_bitmap: B128, //Nintendo Logo (compressed bitmap, same as in GBA Headers)
    nin_logo_bitmap2: B28, //Nintendo Logo (compressed bitmap, same as in GBA Headers)
    nin_logo_checksum: B2, //  Nintendo Logo Checksum, CRC-16 of [0C0h-15Bh], fixed CF56h
    header_checksum: B2,   //Header Checksum, CRC-16 of [000h-15Dh]
    debug_rom_offset: B4,  // Debug rom_offset   (0=none) (8000h and up)       ;only if debug
    debug_size: B4,        //Debug size         (0=none) (max 3BFE00h)        ;version with
    debug_ram_addr: B4,    // Debug ram_address  (0=none) (2400000h..27BFE00h) ;SIO and 8MB
    #[skip]
    __: B4,    //Reserved (zero filled) (transferred, and stored, but not used)
    #[skip]
    __: B128,  //Reserved (zero filled) (transferred, but not stored in RAM)
    #[skip]
    __: B16,   //Reserved (zero filled) (transferred, but not stored in RAM)
}

#[test]
fn test_cartridge() {
    assert_eq!(0x170 + 0x90, size_of::<Cartridge>() * 8);
}
