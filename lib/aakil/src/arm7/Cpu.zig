const std = @import("std");
const utils = @import("../utils.zig");
pub const Self = @This();
const assert = std.debug.assert;

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

// TODO: verify that u8 will be enough. I'm guessing yes since max added clocks is 3S+1N
// and it's reset after every "full" clock cycle
clocks_left: u64 = 0,
clocks_completed: u64 = 0,

cpsr: Cpsr = Cpsr.init(),
spsr_fiq: Cpsr = Cpsr.init(),
spsr_svc: Cpsr = Cpsr.init(),
spsr_abt: Cpsr = Cpsr.init(),
spsr_irq: Cpsr = Cpsr.init(),
spsr_und: Cpsr = Cpsr.init(),

// mem: file =???

state: enum(u1) {
    Arm,
    Thumb,
} = .Arm,

const Cpsr = packed struct {
    c: u1 = 0,
    n: u1 = 0,
    z: u1 = 0,
    v: u1 = 0,
    _: u21 = 0,
    irq_disable: u1 = 0,
    fiq_disable: u1 = 0,
    state: u1 = 0,
    mode_bits: u4 = 0,

    fn init() Cpsr {
        return Cpsr{};
    }

    fn parseCpsrCode(self: Cpsr, code: u4) bool {
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
            0xA => self.n == self.v,
            // TODO: verify that this is actually what its supposed to do
            0xB => self.n != self.v,
            0xC => (self.z == 0) and (self.n == self.v),
            0xD => (self.z == 1) or (self.n != self.v),
            0xE => true,
            0xF => false,
        };
    }
};

pub fn init() Self {
    return Self{};
}

// maybe !void? idk
pub fn processOpcode(self: *Self, instruction: u32) void {
    const opcode = utils.Opcode.init(instruction);
    if (self.cpsr.parseCpsrCode(opcode.condition)) {
        const si = @truncate(u28, instruction);
        const ssi = @truncate(u24, instruction);
        switch (@truncate(u4, (instruction >> 24))) {
            // https://github.com/ziglang/zig/issues/358 :despair:
            0x0 => {
                std.debug.print("Multiply instruction 0x{}\n", .{ssi});
                assert(@truncate(u4, (instruction >> 4)) == 0x9);
                switch (@truncate(u4, (instruction >> 20))) {
                    0x0...0x1 => std.debug.print("multiply: 0x{X}\n", .{si}),
                    0x4...0x7 => std.debug.print("multiply long: 0x{X}\n", .{si}),

                    else => std.debug.print("Unhandled multiply opcode: 0x{X}\n", .{si}),
                }
            },
            0x1 => {
                std.debug.print("sds or bxe 0x{}\n", .{ssi});
                switch (@truncate(u1, (instruction >> 20))) {
                    0b0 => std.debug.print("sds 0x{}", .{ssi}),
                    0b1 => std.debug.print("bex 0x{}", .{ssi}),
                }
            },
            0x6...0x7 => std.debug.print("undefined opcode 0x{}", .{ssi}),
            0x8...0x9 => std.debug.print("block data transfer 0x{}", .{ssi}),
            0xA...0xB => std.debug.print("branch 0x{}", .{ssi}),
            // TODO
            // Coprocessor functions (shouldn't be necessary?)
            0xC...0xE => std.debug.print("coprocessor function 0x{}", .{ssi}),
            0xF => std.debug.print("Software interrupt 0x{}", .{ssi}),
            else => {
                switch (@truncate(u4, (instruction >> 20))) {
                    else => std.debug.print("Unhandled opcode: 0x{X}\n", .{si}),
                }
            },
        }
    }
    self.clocks_completed += 1;
}

// TODO: attach memory modules
pub fn clock(self: *Self) void {
    while (self.clocks_left > 0) {
        // continue
    }
    self.processOpcode(
    // TODO
    0xFFFFFFFF);
}

pub fn adc(self: *Self, rd: u32, op2: u32) void {}

// Tests
test {
    std.testing.refAllDecls(@This());
}
