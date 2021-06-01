const std = @import("std");
const testing = std.testing;
const debug = std.debug;

const lib = @import("lib.zig");

var cpu = lib.gba_cpu.init();

test "test registers" {
    cpu.arm7.r12 = 12;
    try testing.expect(cpu.arm7.r12 == 12);
    cpu.arm7.r12 = 0;
}

test "test opcodes" {
    debug.assert(cpu.arm7.process_opcode(0xE0000000));
}
