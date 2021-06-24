const std = @import("std");

// Ugh I really want to have feature compilation instead of having a DS struct and a
// GBA ready struct :despair:. Maybe zig already optimizes this out
pub const ds_cpu = @import("cpu.zig").DsCpu;
pub const gba_cpu = @import("cpu.zig").GbaCpu;

// Tests
test {
    std.testing.refAllDecls(@This());
}
