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
    cpu.arm7.processOpcode(0xEF000000);
    cpu.arm7.processOpcode(0xE0000000);
    try testing.expect(cpu.arm7.clocks_completed == 2);
}
