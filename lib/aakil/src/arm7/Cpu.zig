pub const Arm7 = @This();

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

state: enum(u1) {
    Arm,
    Thumb,
} = .Arm,

const Cpsr = packed struct {
    c: u1 = 0,
    n: u1 = 0,
    z: u1 = 0,
    v: u1 = 0,

    fn init() Cpsr {
        return Cpsr{};
    }

    fn parse_cpsr_code(self: Cpsr, code: u4) bool {
        return switch (code) {
            0x0 => self.z == 1,
            0x1 => self.z == 0,
            0x2 => self.c == 1,
            0x3 => self.c == 0,
            0x4 => self.n == 1,
            0x5 => self.n == 0,
            0x6 => self.v == 1,
            0x7 => self.v == 0,
            0x8 => (self.c == 1) and (self.z == 0),
            0x9 => (self.c == 0) or (self.z == 1),
            0xa => self.n == self.v,
            // TODO: verify that this is actually what its supposed to do
            0xb => self.n != self.v,
            0xc => (self.z == 0) and (self.n == self.v),
            0xd => (self.z == 1) or (self.n != self.v),
            0xe => true,
            0xf => false,
        };
    }
};

pub fn init() Arm7 {
    return Arm7{};
}

// maybe !void? idk
pub fn process_opcode(self: *Arm7, opcode: u32) bool {
    return self.cpsr.parse_cpsr_code(@truncate(u4, opcode >> 28));
}
