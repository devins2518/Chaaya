const std = @import("std");

pub const Opcode = packed struct {
    const Self = @This();

    condition: u4,
    a: u4,
    b: u4,
    c: u4,
    d: u4,
    e: u4,
    f: u4,
    g: u4,

    pub fn init(opcode: u32) Self {
        return @bitCast(Self, opcode);
    }
};

const InstructionType = enum {
    move,
    move_not,
    orr,
    xor,
    _and,
    and_not,
    test_and,
    test_xor,
    add,
    add_carry,
    sub,
    sub_carry,
    rev_sub,
    rev_sub_carry,
    test_sub,
    test_add,
    mul,
    mul_add,
    umul_long,
};

// Tests
test {
    std.testing.refAllDecls(@This());
}
