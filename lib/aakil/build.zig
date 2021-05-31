const std = @import("std");
const pkgs = @import("deps.zig").pkgs;

pub fn build(b: *std.build.Builder) void {
    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    const lib = b.addStaticLibrary("aakil", "src/lib.zig");
    lib.setBuildMode(mode);
    pkgs.addAllTo(lib);
    lib.install();

    const test_step = b.step("test", "Run all the tests");
    test_step.dependOn(b.getInstallStep());

    var unit_tests = b.addTest("src/unit_test.zig");
    unit_tests.setBuildMode(.Debug);
    test_step.dependOn(&unit_tests.step);
}
