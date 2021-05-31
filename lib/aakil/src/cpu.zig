const std = @import("std");

pub const CPU = struct {
    pub fn hello_world() void {
        std.log.debug("hello", .{});
    }
};
