# Aakil
Aakil is an implemenation of the Nintendo DS's ARM7TDMI and ARM946E-S CPUs along with their respective peripherals.

### About the ARM7
- Fixed 32-bit length instructions
- Fixed 16-bit lenght Thumb instructions
- ARMv4 instructions
- Bi endian
    - Little endian
    ```
    ┌───────────────┬───────────────┬───────────────┬───────────────┐
    │▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│
    └───────────────┴───────────────┴───────────────┴───────────────┘
    └──────────────────────Whole word at 0xA────────────────────────┘

    └───────Half word at 0xA+2──────┴────────Half word at 0xA───────┘

    └─Byte at 0xA+3─┴─Byte at 0xA+2─┴─Byte at 0xA+1─┴──Byte at 0xA──┘
    ```
    - Big endian
    ```
    ┌───────────────┬───────────────┬───────────────┬───────────────┐
    │▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒│
    └───────────────┴───────────────┴───────────────┴───────────────┘
    └──────────────────────Whole word at 0xA────────────────────────┘

    └───────Half word at 0xA────────┴───────Half word at 0xA+2──────┘

    └──Byte at 0xA──┴─Byte at 0xA+1─┴─Byte at 0xA+2─┴─Byte at 0xA+3─┘
    ```
- Data types
    - Words
    - Half-Words
    - Bytes

### About the ARM9
- ARMv5TE instructions

## Useful Resources
- Hardware overview: https://www.copetti.org/writings/consoles/nintendo-ds/
- ARMv4 Instruction Set Descriptions: https://www.ecs.csun.edu/~smirzaei/docs/ece425/arm7tdmi_instruction_set_reference.pdf
- ARMv5TE Instruction Set Descriptions: https://developer.arm.com/documentation/ddi0100/latest/
- ARM7 Technical Manual: http://ww1.microchip.com/downloads/en/DeviceDoc/DDI0029G_7TDMI_R3_trm.pdf
- ARM9 Technical Manual: https://developer.arm.com/documentation/ddi0201/d/programmer-s-model
- General assistance: The EmuDev discord
- The Arm Instruction Set: https://simplemachines.it/doc/arm_inst.pdf
- No$GBA: https://problemkaputt.de/gbatek.htm
