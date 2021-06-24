const std = @import("std");
const Arm7 = @import("arm7/Cpu.zig");

pub const DsCpu = struct {
    arm7: Arm7,
    //arm9: arm9
};

pub const GbaCpu = struct {
    arm7: Arm7 = Arm7.init(),

    pub fn init() GbaCpu {
        return GbaCpu{};
    }
};

// Tests
test {
    std.testing.refAllDecls(@This());
}
