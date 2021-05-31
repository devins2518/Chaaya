pub const Arm7 = packed struct {
    r0: u32 = 0,
    r1: u32 = 0,
    r2: u32 = 0,
    r3: u32 = 0,
    r4: u32 = 0,
    r5: u32 = 0,
    r6: u32 = 0,
    r7: u32 = 0,
    r8: u32 = 0,
    r9: u32 = 0,
    r10: u32 = 0,
    r11: u32 = 0,
    r12: u32 = 0,
    r13: u32 = 0,
    r14: u32 = 0,
    r15: u32 = 0,
    r8_fiq: u32 = 0,
    r9_fiq: u32 = 0,
    r10_fiq: u32 = 0,
    r11_fiq: u32 = 0,
    r12_fiq: u32 = 0,
    r13_fiq: u32 = 0,
    r14_fiq: u32 = 0,
    r13_svc: u32 = 0,
    r14_svc: u32 = 0,
    r13_abt: u32 = 0,
    r14_abt: u32 = 0,
    r13_irq: u32 = 0,
    r14_irq: u32 = 0,
    r13_und: u32 = 0,
    r14_und: u32 = 0,

    cpsr: Cpsr = Cpsr.init(),

    const Cpsr = packed struct {
        c: u1 = 0,
        n: u1 = 0,
        z: u1 = 0,
        v: u1 = 0,

        fn init() Cpsr {
            return Cpsr{};
        }
    };

    pub fn init() Arm7 {
        return Arm7{};
    }
};
