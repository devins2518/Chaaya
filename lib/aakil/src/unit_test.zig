const std = @import("std");
const testing = std.testing;

const lib = @import("lib.zig");

var cpu = lib.gba_cpu.init();

test "test registers" {
    cpu.arm7.r12 = 12;
    try testing.expect(cpu.arm7.r12 == 12);
    cpu.arm7.r12 = 0;
}
