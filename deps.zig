const std = @import("std");
pub const pkgs = struct {
    pub const aakil = std.build.Pkg{
        .name = "aakil",
        .path = "lib/aakil/src/lib.zig",
    };

    pub const dasmaya = std.build.Pkg{
        .name = "dasmaya",
        .path = "lib/dasmaya/src/lib.zig",
    };

    pub const smrti = std.build.Pkg{
        .name = "smrti",
        .path = "lib/smrti/src/lib.zig",
    };

    pub fn addAllTo(artifact: *std.build.LibExeObjStep) void {
        @setEvalBranchQuota(1_000_000);
        inline for (std.meta.declarations(pkgs)) |decl| {
            if (decl.is_pub and decl.data == .Var) {
                artifact.addPackage(@field(pkgs, decl.name));
            }
        }
    }
};

pub const base_dirs = struct {
    pub const aakil = "lib/aakil";
    pub const dasmaya = "lib/dasmaya";
    pub const smrti = "lib/smrti";
};
