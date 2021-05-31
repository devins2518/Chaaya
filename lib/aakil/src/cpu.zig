const arm7 = @import("arm7/cpu.zig");

pub const Ds_Cpu = struct {
    arm7: arm7,
    //arm9: arm9
};

pub const Gba_Cpu = struct {
    arm7: arm7.Arm7,

    pub fn init() Gba_Cpu {
        return Gba_Cpu{ .arm7 = arm7.Arm7.init() };
    }
};
